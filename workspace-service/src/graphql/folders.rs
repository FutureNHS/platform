use super::{db, RequestingUser};
use crate::{db::RepoFactory, services::workspace::RepoCreator};
use async_graphql::{Context, FieldResult, InputObject, Object, SimpleObject, ID};
use fnhs_event_models::{
    Event, EventClient, EventPublisher, FolderCreatedData, FolderDeletedData, FolderUpdatedData,
};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

/// A folder
#[derive(SimpleObject)]
pub struct Folder {
    /// The id of the folder
    id: ID,
    /// The title of the folder
    title: String,
    /// The description of the folder
    description: String,
    /// The workspace that this folder is in
    workspace: ID,
}

impl From<db::Folder> for Folder {
    fn from(d: db::Folder) -> Self {
        Self {
            id: d.id.into(),
            title: d.title,
            description: d.description,
            workspace: d.workspace.into(),
        }
    }
}

#[derive(InputObject)]
struct NewFolder {
    title: String,
    description: String,
    workspace: ID,
}

#[derive(InputObject)]
struct UpdateFolder {
    title: String,
    description: String,
}

#[derive(Default)]
pub struct FoldersQuery;

#[Object]
impl FoldersQuery {
    /// Get all Folders in a workspace
    async fn folders_by_workspace(
        &self,
        context: &Context<'_>,
        workspace: ID,
    ) -> FieldResult<Vec<Folder>> {
        let pool = context.data()?;
        let workspace = Uuid::parse_str(&workspace)?;
        let folders = db::FolderRepo::find_by_workspace(workspace, pool).await?;
        Ok(folders.into_iter().map(Into::into).collect())
    }

    /// Get folder by ID
    async fn folder(&self, context: &Context<'_>, id: ID) -> FieldResult<Folder> {
        self.get_folder(context, id).await
    }

    #[graphql(entity)]
    async fn get_folder(&self, context: &Context<'_>, id: ID) -> FieldResult<Folder> {
        let pool = context.data()?;
        let id = Uuid::parse_str(&id)?;
        let folder = db::FolderRepo::find_by_id(id, pool).await?;
        Ok(folder.into())
    }
}

#[derive(Default)]
pub struct FoldersMutation;

#[Object]
impl FoldersMutation {
    /// Create a new folder (returns the created folder)
    async fn create_folder(
        &self,
        context: &Context<'_>,
        new_folder: NewFolder,
    ) -> FieldResult<Folder> {
        let pool = context.data()?;
        let workspace = Uuid::parse_str(&new_folder.workspace)?;
        let event_client = context.data()?;
        let requesting_user = context.data()?;

        create_folder(
            &new_folder.title,
            &new_folder.description,
            workspace,
            pool,
            requesting_user,
            event_client,
        )
        .await
    }

    /// Update folder (returns updated folder
    async fn update_folder(
        &self,
        context: &Context<'_>,
        id: ID,
        folder: UpdateFolder,
    ) -> FieldResult<Folder> {
        let pool = context.data()?;
        let requesting_user = context.data()?;
        let event_client = context.data()?;

        update_folder(id, folder, pool, requesting_user, event_client).await
    }

    /// Delete folder (returns deleted folder
    async fn delete_folder(&self, context: &Context<'_>, id: ID) -> FieldResult<Folder> {
        let pool = context.data()?;
        let requesting_user = context.data()?;
        let event_client = context.data()?;

        delete_folder(id, pool, requesting_user, event_client).await
    }
}

async fn create_folder(
    title: &str,
    description: &str,
    workspace: Uuid,
    pool: &PgPool,
    requesting_user: &RequestingUser,
    event_client: &EventClient,
) -> FieldResult<Folder> {
    let folder: Folder = db::FolderRepo::create(&title, &description, workspace, pool)
        .await?
        .into();

    let tx: Transaction<Postgres> = pool.begin().await?;
    let mut repos = RepoFactory { executor: tx };
    let user = repos
        .user()
        .find_by_auth_id(requesting_user.auth_id.into())
        .await?
        .ok_or_else(|| anyhow::anyhow!("user not found"))?;

    event_client
        .publish_events(&[Event::new(
            folder.id.clone(),
            FolderCreatedData {
                folder_id: folder.id.clone().into(),
                workspace_id: folder.workspace.clone().into(),
                user_id: user.id.to_string(),
                title: folder.title.clone(),
                description: folder.description.clone(),
            },
        )])
        .await?;
    Ok(folder)
}

async fn update_folder(
    id: ID,
    folder: UpdateFolder,
    pool: &PgPool,
    requesting_user: &RequestingUser,
    event_client: &EventClient,
) -> FieldResult<Folder> {
    let updated_folder = db::FolderRepo::update(
        Uuid::parse_str(&id)?,
        &folder.title,
        &folder.description,
        pool,
    )
    .await?;
    let tx: Transaction<Postgres> = pool.begin().await?;
    let mut repos = RepoFactory { executor: tx };
    let user = repos
        .user()
        .find_by_auth_id(requesting_user.auth_id.into())
        .await?
        .ok_or_else(|| anyhow::anyhow!("user not found"))?;

    event_client
        .publish_events(&[Event::new(
            id,
            FolderUpdatedData {
                folder_id: updated_folder.id.to_string(),
                workspace_id: updated_folder.workspace.to_string(),
                title: updated_folder.title.to_string(),
                description: updated_folder.description.to_string(),
                user_id: user.id.to_string(),
            },
        )])
        .await?;

    Ok(updated_folder.into())
}

async fn delete_folder(
    id: ID,
    pool: &PgPool,
    requesting_user: &RequestingUser,
    event_client: &EventClient,
) -> FieldResult<Folder> {
    let folder = db::FolderRepo::delete(Uuid::parse_str(&id)?, pool).await?;
    let tx: Transaction<Postgres> = pool.begin().await?;
    let mut repos = RepoFactory { executor: tx };
    let user = repos
        .user()
        .find_by_auth_id(requesting_user.auth_id.into())
        .await?
        .ok_or_else(|| anyhow::anyhow!("user not found"))?;
    event_client
        .publish_events(&[Event::new(
            id,
            FolderDeletedData {
                folder_id: folder.id.to_string(),
                user_id: user.id.to_string(),
                workspace_id: folder.workspace.to_string(),
            },
        )])
        .await?;
    Ok(folder.into())
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::graphql::test_mocks::*;
//     use fnhs_event_models::EventData;

//     #[async_std::test]
//     async fn deleting_folder_emits_an_event() -> anyhow::Result<()> {
//         let pool = mock_connection_pool()?;
//         let (events, event_client) = mock_event_emitter();
//         let requesting_user = mock_unprivileged_requesting_user().await?;

//         let folder = delete_folder(
//             "d890181d-6b17-428e-896b-f76add15b54a".into(),
//             &pool,
//             &requesting_user,
//             &event_client,
//         )
//         .await
//         .unwrap();

//         assert_eq!(folder.id, "d890181d-6b17-428e-896b-f76add15b54a");
//         assert!(events
//             .try_iter()
//             .any(|e| matches!(e.data, EventData::FolderDeleted(_))));

//         Ok(())
//     }

//     #[async_std::test]
//     async fn creating_folder_emits_an_event() -> anyhow::Result<()> {
//         let pool = mock_connection_pool()?;
//         let (events, event_client) = mock_event_emitter();
//         let requesting_user = mock_unprivileged_requesting_user().await?;

//         let folder = create_folder(
//             "title",
//             "description",
//             Uuid::new_v4(),
//             &pool,
//             &requesting_user,
//             &event_client,
//         )
//         .await
//         .unwrap();

//         assert_eq!(folder.title, "title");
//         assert_eq!(folder.description, "description");

//         assert!(events
//             .try_iter()
//             .any(|e| matches!(e.data, EventData::FolderCreated(_))));

//         Ok(())
//     }

//     #[async_std::test]
//     async fn update_folder_emits_an_event() -> anyhow::Result<()> {
//         let pool = mock_connection_pool()?;
//         let (events, event_client) = mock_event_emitter();
//         let requesting_user = mock_unprivileged_requesting_user().await?;
//         let current_folder = UpdateFolder {
//             title: "title".to_string(),
//             description: "description".to_string(),
//         };

//         let folder = update_folder(
//             "d890181d-6b17-428e-896b-f76add15b54a".into(),
//             current_folder,
//             &pool,
//             &requesting_user,
//             &event_client,
//         )
//         .await
//         .unwrap();

//         assert_eq!(folder.title, "title");
//         assert_eq!(folder.description, "description");
//         assert!(events
//             .try_iter()
//             .any(|e| matches!(e.data, EventData::FolderUpdated(_))));

//         Ok(())
//     }
// }
