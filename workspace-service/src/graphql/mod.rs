mod file_download_urls;
mod file_upload_urls;
mod files;
mod folders;
mod schema;
mod tracing_ext;
mod users;
mod validation;
mod workspaces;

use super::{azure, db};
use crate::services::{folder::FolderServiceImpl, workspace::WorkspaceServiceImpl};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, MergedObject, Schema,
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
    pub fn new(
        pool: PgPool,
        event_client: EventClient,
        azure_config: azure::Config,
        workspace_service: WorkspaceServiceImpl,
        folder_service: FolderServiceImpl,
    ) -> Self {
        State {
            schema: Schema::build(Query::default(), Mutation::default(), EmptySubscription)
                .extension(tracing_ext::Tracing)
                .data(pool)
                .data(workspace_service)
                .data(folder_service)
                .data(event_client.clone())
                .data(azure_config)
                .finish(),
            event_client,
        }
    }
}

#[derive(MergedObject, Default)]
struct Query(
    files::FilesQuery,
    folders::FoldersQuery,
    workspaces::WorkspacesQuery,
);

#[derive(MergedObject, Default)]
struct Mutation(
    file_download_urls::FileDownloadUrlsMutation,
    file_upload_urls::FileUploadUrlsMutation,
    files::FilesMutation,
    folders::FoldersMutation,
    workspaces::WorkspacesMutation,
    users::UsersMutation,
);

// TODO: move this to the domain and use the AuthId newtype (or make it use a UserId instead)
#[derive(Debug)]
pub struct RequestingUser {
    auth_id: Uuid,
}

pub async fn handle_healthz(_req: Request<State>) -> tide::Result {
    // let response = if !req.state().event_client.is_configured() {
    //     Response::builder(500).body("invalid event client").build()
    // } else {
    //     Response::new(204)
    // };

    let response = Response::new(204);
    Ok(response)
}

pub async fn handle_graphql(req: Request<State>) -> tide::Result {
    let schema = req.state().schema.clone();
    let auth_id = req
        .header("x-user-auth-id")
        .and_then(|values| values.get(0))
        .and_then(|value| Uuid::parse_str(value.as_str()).ok());

    let mut req = async_graphql_tide::receive_request(req).await?;
    if let Some(auth_id) = auth_id {
        req = req.data(RequestingUser { auth_id });
    }

    async_graphql_tide::respond(schema.execute(req).await)
}

pub async fn handle_graphiql(_: Request<State>) -> tide::Result {
    let response = Response::builder(StatusCode::Ok)
        .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
        .content_type(mime::HTML)
        .build();

    Ok(response)
}

pub async fn generate_graphql_schema() -> anyhow::Result<String> {
    let schema = Schema::new(Query::default(), Mutation::default(), EmptySubscription);
    schema::generate_introspection_schema(&schema).await
}
