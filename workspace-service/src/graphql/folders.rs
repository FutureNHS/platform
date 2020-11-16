use super::RequestingUser;
use crate::{
    core::{
        folder::{
            FolderService, {self},
        },
        workspace::{Role, WorkspaceService},
    },
    db::RepoFactory,
    services::{folder::FolderServiceImpl, workspace::WorkspaceServiceImpl},
};
use async_graphql::{
    Context, Enum, Error, ErrorExtensions, FieldResult, InputObject, Object, SimpleObject, ID,
};
use sqlx::PgPool;
use std::{convert::TryInto, fmt::Display, str::FromStr};
use uuid::Uuid;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
enum RoleRequired {
    PlatformMember,
    WorkspaceMember,
}

impl Display for RoleRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RoleRequired::PlatformMember => "PLATFORM_MEMBER",
                RoleRequired::WorkspaceMember => "WORKSPACE_MEMBER",
            }
        )
    }
}

impl FromStr for RoleRequired {
    type Err = ();
    fn from_str(input: &str) -> Result<RoleRequired, Self::Err> {
        match input {
            "PLATFORM_MEMBER" => Ok(RoleRequired::PlatformMember),
            "WORKSPACE_MEMBER" => Ok(RoleRequired::WorkspaceMember),
            _ => Err(()),
        }
    }
}

/// A folder
#[derive(SimpleObject)]
pub struct Folder {
    /// The id of the folder
    id: ID,
    /// The title of the folder
    title: String,
    /// The description of the folder
    description: String,
    /// The group that can access the folder
    role_required: RoleRequired,
    /// The workspace that this folder is in
    workspace: ID,
}

impl From<folder::Folder> for Folder {
    fn from(d: folder::Folder) -> Self {
        Self {
            id: d.id.into(),
            title: d.title,
            description: d.description,
            role_required: RoleRequired::from_str(&d.role_required).unwrap(),
            workspace: d.workspace.into(),
        }
    }
}

#[derive(InputObject)]
struct NewFolder {
    title: String,
    description: String,
    role_required: RoleRequired,
    workspace: ID,
}

#[derive(InputObject)]
struct UpdateFolder {
    id: ID,
    title: String,
    description: String,
    role_required: RoleRequired,
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
        let workspace_service = context.data::<WorkspaceServiceImpl>()?;
        let folder_service = context.data::<FolderServiceImpl>()?;
        let pool = context.data::<PgPool>()?;
        let mut repos = RepoFactory::new(pool.begin().await?);
        let requesting_user = context.data::<RequestingUser>()?;
        // let event_client: &EventClient = context.data()?;
        let workspace_id = Uuid::parse_str(&workspace)?;

        let folders = folder_service
            .find_workspace_folders(&mut repos, workspace_id.into())
            .await?;

        let user_role = workspace_service
            .requesting_user_workspace_rights(
                &mut repos,
                workspace_id.into(),
                requesting_user.auth_id.into(),
            )
            .await?;

        let folders = folders
            .into_iter()
            .map(Into::into)
            .filter(|folder: &Folder| {
                !(folder.role_required == RoleRequired::WorkspaceMember
                    && user_role == Role::NonMember)
            })
            .collect();

        Ok(folders)
    }

    /// Get folder by ID
    async fn folder(&self, context: &Context<'_>, id: ID) -> FieldResult<Folder> {
        self.get_folder(context, id).await
    }

    #[graphql(entity)]
    async fn get_folder(&self, context: &Context<'_>, id: ID) -> FieldResult<Folder> {
        let workspace_service = context.data::<WorkspaceServiceImpl>()?;
        let folder_service = context.data::<FolderServiceImpl>()?;
        let pool = context.data::<PgPool>()?;
        let mut repos = RepoFactory::new(pool.begin().await?);
        let requesting_user = context.data::<RequestingUser>()?;

        let folder_id: Uuid = id.try_into()?;
        // let event_client: &EventClient = context.data()?;

        let folder = folder_service
            .find_folder_by_id(&mut repos, folder_id.into())
            .await?;
        let user_rights = workspace_service
            .requesting_user_workspace_rights(
                &mut repos,
                folder.workspace,
                requesting_user.auth_id.into(),
            )
            .await?;

        if folder.role_required == "WORKSPACE_MEMBER" && user_rights == Role::NonMember {
            Err(Error::new("Insufficient permissions: access denied")
                .extend_with(|_, e| e.set("details", "ACCESS_DENIED")))
        } else {
            Ok(folder.into())
        }
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
        let folder_service = context.data::<FolderServiceImpl>()?;
        let pool: &PgPool = context.data()?;
        let mut repos = RepoFactory::new(pool.begin().await?);

        let workspace_id: Uuid = new_folder.workspace.try_into()?;
        // let event_client = context.data()?;
        let requesting_user: &RequestingUser = context.data()?;

        let folder = folder_service
            .create_folder(
                &mut repos,
                &new_folder.title,
                &new_folder.description,
                &new_folder.role_required.to_string(),
                workspace_id.into(),
                requesting_user.auth_id.into(),
            )
            .await?
            .into();

        Ok(folder)
    }

    /// Update folder (returns updated folder)
    async fn update_folder(
        &self,
        context: &Context<'_>,
        folder: UpdateFolder,
    ) -> FieldResult<Folder> {
        let folder_service = context.data::<FolderServiceImpl>()?;
        let pool: &PgPool = context.data()?;
        let mut repos = RepoFactory::new(pool.begin().await?);
        let requesting_user: &RequestingUser = context.data()?;
        // let event_client = context.data()?;
        let folder_id: Uuid = folder.id.try_into()?;

        let folder = folder_service
            .update_folder(
                &mut repos,
                folder_id.into(),
                &folder.title,
                &folder.description,
                &folder.role_required.to_string(),
                requesting_user.auth_id.into(),
            )
            .await?
            .into();

        Ok(folder)
    }

    /// Delete folder (returns deleted folder)
    async fn delete_folder(&self, context: &Context<'_>, id: ID) -> FieldResult<Folder> {
        let folder_service = context.data::<FolderServiceImpl>()?;
        let pool: &PgPool = context.data()?;
        let mut repos = RepoFactory::new(pool.begin().await?);
        let requesting_user: &RequestingUser = context.data()?;
        // let event_client = context.data()?;
        let folder_id: Uuid = id.try_into()?;

        let folder = folder_service
            .delete_folder(&mut repos, folder_id.into(), requesting_user.auth_id.into())
            .await?
            .into();

        Ok(folder)
    }
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
//             &RoleRequired::PlatformMember.to_string(),
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
//             id: "d890181d-6b17-428e-896b-f76add15b54a".into(),
//             title: "title".to_string(),
//             description: "description".to_string(),
//             role_required: RoleRequired::PlatformMember,
//         };

//         let folder = update_folder(current_folder, &pool, &requesting_user, &event_client)
//             .await
//             .unwrap();

//         assert_eq!(folder.title, "title");
//         assert_eq!(folder.description, "description");
//         assert!(events
//             .try_iter()
//             .any(|e| matches!(e.data, EventData::FolderUpdated(_))));

//         Ok(())
//     }
// }
