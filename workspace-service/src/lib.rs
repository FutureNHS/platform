pub mod azure;
pub mod config;
mod core;
mod db;
mod graphql;
mod services;

use fnhs_event_models::EventClient;
pub use graphql::generate_graphql_schema;
use opentelemetry::api::{Extractor, TraceContextExt};
use services::workspace::WorkspaceServiceImpl;
use sqlx::PgPool;
use tide::{Middleware, Next, Redirect, Request, Server};
use tracing::info_span;
use tracing_futures::Instrument;
use tracing_opentelemetry::OpenTelemetrySpanExt;

struct RequestExtractor<'a, State> {
    req: &'a Request<State>,
}

impl<'a, State> Extractor for RequestExtractor<'a, State> {
    /// Get a value for a key from the HeaderMap.  If the value is not valid ASCII, returns None.
    fn get(&self, key: &str) -> Option<&str> {
        self.req.header(key).map(|value| value.as_str())
    }
}

struct TracingMiddleware;

#[async_trait::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for TracingMiddleware {
    async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> tide::Result {
        let parent_context = opentelemetry::global::get_http_text_propagator(|propagator| {
            propagator.extract(&RequestExtractor { req: &req })
        });

        let span = info_span!(
            "request",
            otel.kind = "server",
            http.status_code = tracing::field::Empty,
            http.method = req.method().as_ref(),
            http.target = req.url().as_ref()
        );

        // B3Propagator has a bug in version, which should be fixed in opentelemetry >0.9.1. When
        // no headers are present it returns an invalid span context, which causes propagation to
        // fail. This check if a workaround. After the version upgrade we can unconditionally call
        // set_parent.
        if (parent_context
            .remote_span_context()
            .map(|span_context| span_context.is_valid())
            .unwrap_or(false))
        {
            span.set_parent(&parent_context);
        }

        let res = next.run(req).instrument(span.clone()).await;
        span.record("http.status_code", &u16::from(res.status()));

        Ok(res)
    }
}

pub async fn create_app(
    connection_pool: PgPool,
    event_client: EventClient,
    azure_config: azure::Config,
) -> anyhow::Result<Server<graphql::State>> {
    let service = WorkspaceServiceImpl {};
    let mut app = tide::with_state(graphql::State::new(
        connection_pool,
        event_client,
        azure_config,
        service,
    ));

    app.with(TracingMiddleware);

    app.at("/").get(Redirect::permanent("/graphiql"));
    app.at("/healthz").get(graphql::handle_healthz);
    app.at("/graphql").post(graphql::handle_graphql);
    app.at("/graphiql").get(graphql::handle_graphiql);

    Ok(app)
}
