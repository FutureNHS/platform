use fnhs_event_models::EventClient;
use sqlx::PgPool;
use std::future::Future;
use std::pin::Pin;
use tide::{Next, Redirect, Request, Server};
use tracing::info_span;
use tracing_futures::Instrument;

mod db;
mod graphql;

pub fn log<'a>(
    req: Request<graphql::State>,
    next: Next<'a, graphql::State>,
) -> Pin<Box<dyn Future<Output = Result<tide::Response, http_types::Error>> + Send + 'a>> {
    Box::pin(async {
        let span = info_span!(
            "request",
            otel.kind = "server",
            http.status_code = tracing::field::Empty,
            http.method = req.method().as_ref(),
            http.target = req.url().as_ref()
        );

        let res = next.run(req).instrument(span.clone()).await;
        span.record("http.status_code", &u16::from(res.status()));

        Ok(res)
    })
}

pub async fn create_app(
    connection_pool: PgPool,
    event_client: EventClient,
) -> anyhow::Result<Server<graphql::State>> {
    let mut app = tide::with_state(graphql::State::new(connection_pool, event_client));

    app.with(log);

    app.at("/").get(Redirect::permanent("/graphiql"));
    app.at("/healthz").get(graphql::handle_healthz);
    app.at("/graphql").post(graphql::handle_graphql);
    app.at("/graphiql").get(graphql::handle_graphiql);

    Ok(app)
}

#[cfg(test)]
mod tests {
    use super::*;
    use http_types::{Method, Response, StatusCode, Url};
    use std::env;

    async fn create_app_for_test() -> anyhow::Result<Server<graphql::State>> {
        let database_url =
            env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL env var not found");
        let connection_pool = PgPool::connect(&database_url).await?;

        create_app(connection_pool, EventClient::default()).await
    }

    #[async_std::test]
    async fn root_redirects_to_graphiql() -> anyhow::Result<()> {
        let app = create_app_for_test().await?;
        let req = http_types::Request::new(
            Method::Get,
            Url::parse("http://workspace-service.workspace-service/").unwrap(),
        );

        let resp: Response = app.respond(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::PermanentRedirect);
        assert_eq!(resp.header("location").unwrap().as_str(), "/graphiql");

        Ok(())
    }

    #[async_std::test]
    async fn graphql_schema_has_query_and_mutation() -> anyhow::Result<()> {
        let app = create_app_for_test().await?;

        let mut req = http_types::Request::new(
            Method::Post,
            Url::parse("http://workspace-service.workspace-service/graphql").unwrap(),
        );
        req.set_body(
            r#"{ "operationName": "IntrospectionQuery", "variables": {}, "query": "
                {
                    __schema {
                        queryType {
                            name
                        }
                        mutationType {
                            name
                        }
                        subscriptionType {
                            name
                        }
                    }
                }
            "}"#
            .replace("\n", "\\n"),
        );

        let mut resp: Response = dbg!(app.respond(req).await.unwrap());

        assert_eq!(
            resp.take_body().into_string().await.unwrap(),
            r#"{"data":{"__schema":{"queryType":{"name":"Query"},"mutationType":{"name":"Mutation"},"subscriptionType":null}}}"#,
        );
        assert_eq!(resp.status(), StatusCode::Ok);

        Ok(())
    }
}
