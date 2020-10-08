mod file_download_urls;
mod file_uploads;
mod file_versions;
mod file_upload_urls;
mod files;
mod folders;
mod schema;
#[cfg(test)]
mod test_mocks;
mod users;
mod workspaces;

use super::azure;
use super::db;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, GQLMergedObject, Schema,
};
use fnhs_event_models::EventClient;
use sqlx::PgPool;
use tide::{http::mime, Request, Response, StatusCode};
use uuid::Uuid;

#[derive(Clone)]
pub struct State {
    schema: Schema<Query, Mutation, EmptySubscription>,
    event_client: EventClient,
}

impl State {
    pub fn new(pool: PgPool, event_client: EventClient, azure_config: azure::Config) -> State {
        State {
            schema: Schema::build(Query::default(), Mutation::default(), EmptySubscription)
                .data(pool)
                .data(event_client.clone())
                .data(azure_config)
                .finish(),
            event_client,
        }
    }
}

#[derive(GQLMergedObject, Default)]
struct Query(
    files::FilesQuery,
    folders::FoldersQuery,
    workspaces::WorkspacesQuery,
    file_download_urls::FileDownloadUrlsQuery,
    file_upload_urls::FileUploadUrlsQuery,
);

#[derive(GQLMergedObject, Default)]
struct Mutation(
    folders::FoldersMutation,
    workspaces::WorkspacesMutation,
    files::FilesMutation,
    users::UsersMutation,
);

#[derive(Debug)]
pub struct RequestingUser {
    auth_id: Uuid,
}

pub async fn handle_healthz(req: Request<State>) -> tide::Result {
    let response = if !req.state().event_client.is_configured() {
        Response::builder(500).body("invalid event client").build()
    } else {
        Response::new(204)
    };

    Ok(response)
}

pub async fn handle_graphql(req: Request<State>) -> tide::Result {
    let schema = req.state().schema.clone();
    let auth_id = req
        .header("x-user-auth-id")
        .and_then(|values| values.get(0))
        .and_then(|value| Uuid::parse_str(value.as_str()).ok());

    async_graphql_tide::graphql(req, schema, |query_builder| match auth_id {
        Some(auth_id) => query_builder.data(RequestingUser { auth_id }),
        None => query_builder,
    })
    .await
}

pub async fn handle_graphiql(_: Request<State>) -> tide::Result {
    let response = Response::builder(StatusCode::Ok)
        .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
        .content_type(mime::HTML)
        .build();

    Ok(response)
}

pub async fn generate_graphql_schema() -> anyhow::Result<String> {
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription).finish();
    schema::generate_introspection_schema(&schema).await
}
