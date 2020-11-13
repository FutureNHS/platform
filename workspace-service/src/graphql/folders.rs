use super::{db, RequestingUser};
use crate::graphql::workspaces::{requesting_user_workspace_rights, WorkspaceMembership};
use async_graphql::{
    Context, Enum, Error, ErrorExtensions, FieldResult, InputObject, Object, SimpleObject, ID,
};
use fnhs_event_models::{
    Event, EventClient, EventPublisher, FolderCreatedData, FolderDeletedData, FolderUpdatedData,
};
use sqlx::PgPool;
use std::fmt::Display;
use std::str::FromStr;
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

impl From<db::Folder> for Folder {
    fn from(d: db::Folder) -> Self {
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
        let pool = context.data()?;
        let workspace = Uuid::parse_str(&workspace)?;
        let requesting_user = context.data()?;
        let folders = db::FolderRepo::find_by_workspace(workspace, pool).await?;
        let user_role = requesting_user_workspace_rights(workspace, requesting_user, pool).await?;
        Ok(folders
            .into_iter()
            .map(Into::into)
            .filter(|folder: &Folder| {
                !(folder.role_required == RoleRequired::WorkspaceMember
                    && user_role == WorkspaceMembership::NonMember)
            })
            .collect())
    }

    /// Get folder by ID
    async fn folder(&self, context: &Context<'_>, id: ID) -> FieldResult<Folder> {
        self.get_folder(context, id).await
    }

    #[graphql(entity)]
    async fn get_folder(&self, context: &Context<'_>, id: ID) -> FieldResult<Folder> {
        let pool = context.data()?;
        let id = Uuid::parse_str(&id)?;
        let requesting_user = context.data()?;
        let folder = db::FolderRepo::find_by_id(id, pool).await?;
        let user_rights =
            requesting_user_workspace_rights(folder.workspace, requesting_user, pool).await?;
        if folder.role_required == "WORKSPACE_MEMBER"
            && user_rights == WorkspaceMembership::NonMember
        {
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
        let pool = context.data()?;
        let workspace = Uuid::parse_str(&new_folder.workspace)?;
        let event_client = context.data()?;
        let requesting_user = context.data()?;
        create_folder(
            &new_folder.title,
            &new_folder.description,
            &new_folder.role_required.to_string(),
            workspace,
            pool,
            requesting_user,
            event_client,
        )
        .await
    }

    /// Update folder (returns updated folder)
    async fn update_folder(
        &self,
        context: &Context<'_>,
        folder: UpdateFolder,
    ) -> FieldResult<Folder> {
        let pool = context.data()?;
        let requesting_user = context.data()?;
        let event_client = context.data()?;

        update_folder(folder, pool, requesting_user, event_client).await
    }

    /// Delete folder (returns deleted folder)
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
    role_required: &str,
    workspace: Uuid,
    pool: &PgPool,
    requesting_user: &RequestingUser,
    event_client: &EventClient,
) -> FieldResult<Folder> {
    let folder: Folder =
        db::FolderRepo::create(&title, &description, &role_required, workspace, pool)
            .await?
            .into();

    let user = db::UserRepo::find_by_auth_id(&requesting_user.auth_id, pool)
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
                role_required: folder.role_required.to_string(),
            },
        )])
        .await?;
    Ok(folder)
}

async fn update_folder(
    folder: UpdateFolder,
    pool: &PgPool,
    requesting_user: &RequestingUser,
    event_client: &EventClient,
) -> FieldResult<Folder> {
    let updated_folder = db::FolderRepo::update(
        Uuid::parse_str(&folder.id)?,
        &folder.title,
        &folder.description,
        &folder.role_required.to_string(),
        pool,
    )
    .await?;

    let user = db::UserRepo::find_by_auth_id(&requesting_user.auth_id, pool)
        .await?
        .ok_or_else(|| anyhow::anyhow!("user not found"))?;

    event_client
        .publish_events(&[Event::new(
            folder.id,
            FolderUpdatedData {
                folder_id: updated_folder.id.to_string(),
                workspace_id: updated_folder.workspace.to_string(),
                title: updated_folder.title.to_string(),
                description: updated_folder.description.to_string(),
                user_id: user.id.to_string(),
                role_required: updated_folder.role_required.to_string(),
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
    let user = db::UserRepo::find_by_auth_id(&requesting_user.auth_id, pool)
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::graphql::test_mocks::*;
    use fnhs_event_models::EventData;

    #[async_std::test]
    async fn deleting_folder_emits_an_event() -> anyhow::Result<()> {
        let pool = mock_connection_pool()?;
        let (events, event_client) = mock_event_emitter();
        let requesting_user = mock_unprivileged_requesting_user().await?;

        let folder = delete_folder(
            "d890181d-6b17-428e-896b-f76add15b54a".into(),
            &pool,
            &requesting_user,
            &event_client,
        )
        .await
        .unwrap();

        assert_eq!(folder.id, "d890181d-6b17-428e-896b-f76add15b54a");
        assert!(events
            .try_iter()
            .any(|e| matches!(e.data, EventData::FolderDeleted(_))));

        Ok(())
    }

    #[async_std::test]
    async fn creating_folder_emits_an_event() -> anyhow::Result<()> {
        let pool = mock_connection_pool()?;
        let (events, event_client) = mock_event_emitter();
        let requesting_user = mock_unprivileged_requesting_user().await?;

        let folder = create_folder(
            "title",
            "description",
            &RoleRequired::PlatformMember.to_string(),
            Uuid::new_v4(),
            &pool,
            &requesting_user,
            &event_client,
        )
        .await
        .unwrap();

        assert_eq!(folder.title, "title");
        assert_eq!(folder.description, "description");

        assert!(events
            .try_iter()
            .any(|e| matches!(e.data, EventData::FolderCreated(_))));

        Ok(())
    }

    #[async_std::test]
    async fn update_folder_emits_an_event() -> anyhow::Result<()> {
        let pool = mock_connection_pool()?;
        let (events, event_client) = mock_event_emitter();
        let requesting_user = mock_unprivileged_requesting_user().await?;
        let current_folder = UpdateFolder {
            id: "d890181d-6b17-428e-896b-f76add15b54a".into(),
            title: "title".to_string(),
            description: "description".to_string(),
            role_required: RoleRequired::PlatformMember,
        };

        let folder = update_folder(current_folder, &pool, &requesting_user, &event_client)
            .await
            .unwrap();

        assert_eq!(folder.title, "title");
        assert_eq!(folder.description, "description");
        assert!(events
            .try_iter()
            .any(|e| matches!(e.data, EventData::FolderUpdated(_))));

        Ok(())
    }
}
