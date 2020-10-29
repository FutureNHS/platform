use crate::db;
use crate::db::Workspace;
use crate::graphql::users::User;
use crate::graphql::RequestingUser;
use async_graphql::{Context, FieldResult, InputObject, Object, ID};
use fnhs_event_models::{Event, EventClient, EventPublisher as _, WorkspaceCreatedData};
use sqlx::PgPool;
use uuid::Uuid;

#[Object]
/// A workspace
impl Workspace {
    /// The id of the workspace
    async fn id(&self) -> ID {
        self.id.into()
    }
    /// The title of the workspace
    async fn title(&self) -> String {
        self.title.clone()
    }
    /// The description of the workspace
    async fn description(&self) -> String {
        self.description.clone()
    }

    /// List of users in the admin group
    async fn admins(&self, context: &Context<'_>) -> FieldResult<Vec<User>> {
        let pool = context.data()?;
        let users = db::Group::group_members(self.admins, pool).await?;
        Ok(users.into_iter().map(Into::into).collect())
    }

    /// List of all users who are members of this workspace
    async fn members(&self, context: &Context<'_>) -> FieldResult<Vec<User>> {
        let pool = context.data()?;
        let users = db::Group::group_members(self.members, pool).await?;
        Ok(users.into_iter().map(Into::into).collect())
    }
}

#[derive(InputObject)]
struct NewWorkspace {
    title: String,
    description: String,
}

#[derive(InputObject)]
struct UpdateWorkspace {
    title: String,
    description: String,
}

#[derive(Default)]
pub struct WorkspacesQuery;

#[Object]
impl WorkspacesQuery {
    /// Get all Workspaces
    async fn workspaces(&self, context: &Context<'_>) -> FieldResult<Vec<Workspace>> {
        let pool = context.data()?;
        let workspaces = db::Workspace::find_all(pool).await?;
        Ok(workspaces.into_iter().map(Into::into).collect())
    }

    /// Get workspace by ID
    async fn workspace(&self, context: &Context<'_>, id: ID) -> FieldResult<Workspace> {
        self.get_workspace(context, id).await
    }

    #[graphql(entity)]
    async fn get_workspace(&self, context: &Context<'_>, id: ID) -> FieldResult<Workspace> {
        let pool = context.data()?;
        let id = Uuid::parse_str(id.as_str())?;
        let workspace = db::Workspace::find_by_id(id, pool).await?;
        Ok(workspace.into())
    }
}

#[derive(Default)]
pub struct WorkspacesMutation;

#[Object]
impl WorkspacesMutation {
    /// Create a new workspace (returns the created workspace)
    async fn create_workspace(
        &self,
        context: &Context<'_>,
        new_workspace: NewWorkspace,
    ) -> FieldResult<Workspace> {
        let pool = context.data()?;
        let event_client: &EventClient = context.data()?;
        let requesting_user = context.data::<super::RequestingUser>()?;

        create_workspace(
            &new_workspace.title,
            &new_workspace.description,
            requesting_user,
            pool,
            event_client,
        )
        .await
    }

    /// Update workspace (returns updated workspace
    async fn update_workspace(
        &self,
        context: &Context<'_>,
        id: ID,
        workspace: UpdateWorkspace,
    ) -> FieldResult<Workspace> {
        // TODO: Add event
        let pool = context.data()?;
        let workspace = db::Workspace::update(
            Uuid::parse_str(id.as_str())?,
            &workspace.title,
            &workspace.description,
            pool,
        )
        .await?;

        Ok(workspace.into())
    }

    /// Delete workspace(returns deleted workspace
    async fn delete_workspace(&self, context: &Context<'_>, id: ID) -> FieldResult<Workspace> {
        // TODO: Add event
        let pool = context.data()?;
        let workspace = db::Workspace::delete(Uuid::parse_str(id.as_str())?, pool).await?;

        Ok(workspace.into())
    }
}

async fn create_workspace(
    title: &str,
    description: &str,
    requesting_user: &RequestingUser,
    pool: &PgPool,
    event_client: &EventClient,
) -> FieldResult<Workspace> {
    let user = db::User::find_by_auth_id(&requesting_user.auth_id, pool).await?;
    if !user.is_platform_admin {
        return Err(anyhow::anyhow!(
            "User with auth_id {} does not have permission to create a workspace.",
            requesting_user.auth_id,
        )
        .into());
    }

    let workspace: Workspace = db::Workspace::create(title, description, pool).await?;

    event_client
        .publish_events(&[Event::new(
            workspace.id.to_string(),
            WorkspaceCreatedData {
                workspace_id: workspace.id.to_string(),
                // TODO: Fill this in when we have users in the db.
                user_id: "".into(),
                title: workspace.title.clone(),
            },
        )])
        .await?;

    Ok(workspace)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::graphql::test_mocks::*;
    use fnhs_event_models::EventData;

    #[async_std::test]
    async fn creating_workspace_emits_an_event() -> anyhow::Result<()> {
        let pool = mock_connection_pool()?;
        let (events, event_client) = mock_event_emitter();

        let workspace = create_workspace(
            "title",
            "description",
            &mock_admin_requesting_user(),
            &pool,
            &event_client,
        )
        .await
        .unwrap();

        assert_eq!(workspace.title, "title");
        assert_eq!(workspace.description, "description");

        assert!(events
            .try_iter()
            .any(|e| matches!(e.data, EventData::WorkspaceCreated(_))));

        Ok(())
    }

    #[async_std::test]
    async fn creating_workspace_as_non_admin_fails() -> anyhow::Result<()> {
        let pool = mock_connection_pool()?;
        let (events, event_client) = mock_event_emitter();

        let result = create_workspace(
            "title",
            "description",
            &mock_unprivileged_requesting_user(),
            &pool,
            &event_client,
        )
        .await;

        assert_eq!(result.err().unwrap().message, "User with auth_id deadbeef-0000-0000-0000-000000000000 does not have permission to create a workspace.");

        assert_eq!(events.try_iter().count(), 0);

        Ok(())
    }
}
