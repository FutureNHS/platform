use crate::{
    db,
    db::{Role, WorkspaceRepo},
    graphql::{users::User, RequestingUser},
    services::workspace::{self, WorkspaceService, WorkspaceServiceImpl},
};
use async_graphql::{Context, Enum, FieldResult, InputObject, Object, ID};
use fnhs_event_models::{
    Event, EventClient, EventPublisher as _, WorkspaceCreatedData, WorkspaceMembershipChangedData,
};
use sqlx::PgPool;
use std::convert::TryInto;
use uuid::Uuid;

pub struct Workspace {
    id: ID,
    title: String,
    description: String,
    admins: Uuid,
    members: Uuid,
}

impl From<workspace::Workspace> for Workspace {
    fn from(workspace: workspace::Workspace) -> Self {
        Workspace {
            id: workspace.id.into(),
            title: workspace.title,
            description: workspace.description,
            admins: workspace.admins.into(),
            members: workspace.members.into(),
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum RoleFilter {
    /// Only return Admins
    Admin,
    /// Only return Non-Admins
    NonAdmin,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum NewRole {
    /// Promote to admin
    Admin,
    /// Add as a non-admin member or demote an admin
    NonAdmin,
    /// Remove member
    NonMember,
}

impl From<NewRole> for Role {
    fn from(role: NewRole) -> Self {
        match role {
            NewRole::Admin => Role::Admin,
            NewRole::NonAdmin => Role::NonAdmin,
            NewRole::NonMember => Role::NonMember,
        }
    }
}

#[Object]
/// A workspace
impl Workspace {
    /// The id of the workspace
    async fn id(&self) -> ID {
        self.id.clone()
    }
    /// The title of the workspace
    async fn title(&self) -> String {
        self.title.clone()
    }
    /// The description of the workspace
    async fn description(&self) -> String {
        self.description.clone()
    }

    /// List of users who are members of this workspace.
    ///
    /// Pass RoleFilter: Admin or NonAdmin for finer control over
    /// which members are returned.
    async fn members(
        &self,
        context: &Context<'_>,
        filter: Option<RoleFilter>,
    ) -> FieldResult<Vec<User>> {
        let pool = context.data()?;
        let users = match filter {
            Some(RoleFilter::Admin) => db::TeamRepo::members(self.admins, pool).await?,
            Some(RoleFilter::NonAdmin) => {
                db::TeamRepo::members_difference(self.members, self.admins, pool).await?
            }
            None => db::TeamRepo::members(self.members, pool).await?,
        };
        Ok(users.into_iter().map(Into::into).collect())
    }
}

impl From<db::Workspace> for Workspace {
    fn from(d: db::Workspace) -> Self {
        Self {
            id: d.id.into(),
            title: d.title,
            description: d.description,
            admins: d.admins,
            members: d.members,
        }
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

#[derive(InputObject)]
struct MembershipChange {
    workspace: ID,
    user: ID,
    new_role: NewRole,
}

#[derive(Default)]
pub struct WorkspacesQuery;

#[Object]
impl WorkspacesQuery {
    /// Get all Workspaces
    async fn workspaces(&self, context: &Context<'_>) -> FieldResult<Vec<Workspace>> {
        let pool = context.data()?;
        let workspaces = WorkspaceRepo::find_all(pool).await?;
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
        let workspace = WorkspaceRepo::find_by_id(id, pool).await?;
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
        let workspace_service = context.data::<WorkspaceServiceImpl>()?;
        let requesting_user = context.data::<RequestingUser>()?;

        let new_workspace = workspace_service
            .create(
                &new_workspace.title,
                &new_workspace.description,
                requesting_user.auth_id.into(),
            )
            .await?;
        Ok(new_workspace.into())
    }

    /// Update workspace (returns updated workspace)
    async fn update_workspace(
        &self,
        context: &Context<'_>,
        id: ID,
        workspace: UpdateWorkspace,
    ) -> FieldResult<Workspace> {
        // TODO: Add event
        let pool = context.data()?;
        let workspace = WorkspaceRepo::update(
            Uuid::parse_str(id.as_str())?,
            &workspace.title,
            &workspace.description,
            pool,
        )
        .await?;

        Ok(workspace.into())
    }

    /// Delete workspace (returns deleted workspace)
    async fn delete_workspace(&self, context: &Context<'_>, id: ID) -> FieldResult<Workspace> {
        // TODO: Add event
        let pool = context.data()?;
        let workspace = WorkspaceRepo::delete(Uuid::parse_str(id.as_str())?, pool).await?;

        Ok(workspace.into())
    }

    /// Changes workspace permissions for a user (Admin/NonAdmin/NonMember)
    async fn change_workspace_membership(
        &self,
        context: &Context<'_>,
        input: MembershipChange,
    ) -> FieldResult<Workspace> {
        let pool = context.data()?;
        let requesting_user = context.data::<RequestingUser>()?;
        let event_client: &EventClient = context.data()?;

        change_workspace_membership(
            input.workspace.try_into()?,
            input.user.try_into()?,
            input.new_role.into(),
            requesting_user,
            pool,
            event_client,
        )
        .await
    }
}

async fn create_workspace(
    title: &str,
    description: &str,
    requesting_user: &RequestingUser,
    pool: &PgPool,
    event_client: &EventClient,
) -> FieldResult<Workspace> {
    let user = db::UserRepo::find_by_auth_id(&requesting_user.auth_id, pool)
        .await?
        .ok_or_else(|| anyhow::anyhow!("user not found"))?;
    if !user.is_platform_admin {
        return Err(anyhow::anyhow!(
            "User with auth_id {} does not have permission to create a workspace.",
            requesting_user.auth_id,
        )
        .into());
    }

    let workspace: Workspace = WorkspaceRepo::create(title, description, pool)
        .await?
        .into();

    event_client
        .publish_events(&[Event::new(
            workspace.id.clone(),
            WorkspaceCreatedData {
                workspace_id: workspace.id.clone().into(),
                // TODO: Fill this in when we have users in the db.
                user_id: "".into(),
                title: workspace.title.clone(),
            },
        )])
        .await?;

    Ok(workspace)
}

async fn change_workspace_membership(
    workspace_id: Uuid,
    user_id: Uuid,
    role: Role,
    requesting_user: &RequestingUser,
    pool: &PgPool,
    event_client: &EventClient,
) -> FieldResult<Workspace> {
    let user = db::UserRepo::find_by_auth_id(&requesting_user.auth_id, pool)
        .await?
        .ok_or_else(|| anyhow::anyhow!("user not found"))?;

    if !user.is_platform_admin && !WorkspaceRepo::is_admin(workspace_id, user.id, pool).await? {
        return Err(anyhow::anyhow!(
            "user with auth_id {} does not have permission to update workspace membership",
            user.auth_id,
        )
        .into());
    }

    if user.id == user_id {
        return Err(anyhow::anyhow!(
            "user with auth_id {} cannot demote themselves to {}",
            user.auth_id,
            role
        )
        .into());
    }

    let workspace: Workspace =
        WorkspaceRepo::change_workspace_membership(workspace_id, user_id, role, pool)
            .await?
            .into();

    event_client
        .publish_events(&[Event::new(
            workspace.id.clone(),
            WorkspaceMembershipChangedData {
                requesting_user_id: requesting_user.auth_id.to_string(),
                affected_workspace_id: workspace.id.clone().into(),
                affected_user_id: user_id.to_string(),
                affected_role: role.to_string(),
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
            &mock_admin_requesting_user().await?,
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
            &mock_unprivileged_requesting_user().await?,
            &pool,
            &event_client,
        )
        .await;

        assert_eq!(result.err().unwrap().message, "User with auth_id deadbeef-0000-0000-0000-000000000000 does not have permission to create a workspace.");

        assert_eq!(events.try_iter().count(), 0);

        Ok(())
    }

    #[async_std::test]
    async fn a_workspace_admin_cannot_demote_themselves_to_member() -> anyhow::Result<()> {
        use db::TeamRepo;

        let pool = mock_connection_pool()?;
        let (events, event_client) = mock_event_emitter();
        let requesting_user = mock_unprivileged_requesting_user().await?;
        let requesting_user_user = db::UserRepo::find_by_auth_id(&requesting_user.auth_id, &pool)
            .await?
            .ok_or_else(|| anyhow::anyhow!("user not found"))?;

        let workspace = WorkspaceRepo::create("", "", &pool).await?;
        TeamRepo::add_member(workspace.members, requesting_user_user.id, &pool).await?;
        TeamRepo::add_member(workspace.admins, requesting_user_user.id, &pool).await?;
        let result = change_workspace_membership(
            workspace.id,
            requesting_user_user.id,
            Role::NonAdmin,
            &requesting_user,
            &pool,
            &event_client,
        )
        .await;

        assert_eq!(
            result.err().unwrap().message,
            format!(
                "user with auth_id {} cannot demote themselves to NonAdmin",
                requesting_user.auth_id
            )
        );

        assert_eq!(events.try_iter().count(), 0);

        Ok(())
    }

    #[async_std::test]
    async fn a_workspace_admin_cannot_demote_themselves_to_non_member() -> anyhow::Result<()> {
        use db::TeamRepo;

        let pool = mock_connection_pool()?;
        let (events, event_client) = mock_event_emitter();
        let requesting_user = mock_unprivileged_requesting_user().await?;
        let requesting_user_user = db::UserRepo::find_by_auth_id(&requesting_user.auth_id, &pool)
            .await?
            .ok_or_else(|| anyhow::anyhow!("user not found"))?;

        let workspace = WorkspaceRepo::create("", "", &pool).await?;
        TeamRepo::add_member(workspace.members, requesting_user_user.id, &pool).await?;
        TeamRepo::add_member(workspace.admins, requesting_user_user.id, &pool).await?;
        let result = change_workspace_membership(
            workspace.id,
            requesting_user_user.id,
            Role::NonMember,
            &requesting_user,
            &pool,
            &event_client,
        )
        .await;

        assert_eq!(
            result.err().unwrap().message,
            format!(
                "user with auth_id {} cannot demote themselves to NonMember",
                requesting_user.auth_id
            )
        );

        assert_eq!(events.try_iter().count(), 0);

        Ok(())
    }

    #[async_std::test]
    async fn a_user_cannot_add_another_if_they_are_neither_site_nor_workspace_admin(
    ) -> anyhow::Result<()> {
        let pool = mock_connection_pool()?;
        let (events, event_client) = mock_event_emitter();
        let requesting_user = mock_unprivileged_requesting_user().await?;

        let workspace = WorkspaceRepo::create("", "", &pool).await?;
        let result = change_workspace_membership(
            workspace.id,
            Uuid::new_v4(),
            Role::NonAdmin,
            &requesting_user,
            &pool,
            &event_client,
        )
        .await;

        assert_eq!(
            result.err().unwrap().message,
            format!(
                "user with auth_id {} does not have permission to update workspace membership",
                requesting_user.auth_id
            )
        );

        assert_eq!(events.try_iter().count(), 0);

        Ok(())
    }

    #[async_std::test]
    async fn a_user_cannot_add_another_if_they_are_just_a_workspace_member() -> anyhow::Result<()> {
        use db::TeamRepo;

        let pool = mock_connection_pool()?;
        let (events, event_client) = mock_event_emitter();
        let requesting_user = mock_unprivileged_requesting_user().await?;
        let user = db::UserRepo::find_by_auth_id(&requesting_user.auth_id, &pool)
            .await?
            .ok_or_else(|| anyhow::anyhow!("user not found"))?;

        let workspace = WorkspaceRepo::create("", "", &pool).await?;
        TeamRepo::add_member(workspace.members, user.id, &pool).await?;

        let result = change_workspace_membership(
            workspace.id,
            Uuid::new_v4(),
            Role::Admin,
            &requesting_user,
            &pool,
            &event_client,
        )
        .await;

        assert_eq!(
            result.err().unwrap().message,
            format!(
                "user with auth_id {} does not have permission to update workspace membership",
                requesting_user.auth_id
            )
        );

        assert_eq!(events.try_iter().count(), 0);

        Ok(())
    }

    #[async_std::test]
    async fn a_workspace_admin_can_add_another_user_as_member() -> anyhow::Result<()> {
        use db::TeamRepo;

        let pool = mock_connection_pool()?;
        let (events, event_client) = mock_event_emitter();
        let requesting_user = mock_unprivileged_requesting_user().await?;
        let requesting_user_user = db::UserRepo::find_by_auth_id(&requesting_user.auth_id, &pool)
            .await?
            .ok_or_else(|| anyhow::anyhow!("user not found"))?;

        let workspace = WorkspaceRepo::create("", "", &pool).await?;
        TeamRepo::add_member(workspace.members, requesting_user_user.id, &pool).await?;
        TeamRepo::add_member(workspace.admins, requesting_user_user.id, &pool).await?;

        let user_id = Uuid::new_v4();
        change_workspace_membership(
            workspace.id,
            user_id,
            Role::NonAdmin,
            &requesting_user,
            &pool,
            &event_client,
        )
        .await
        .unwrap();

        let is_admin = TeamRepo::is_member(workspace.admins, user_id, &pool)
            .await
            .unwrap();
        let is_member = TeamRepo::is_member(workspace.members, user_id, &pool)
            .await
            .unwrap();

        assert_eq!(is_admin, false, "should not be an admin");
        assert_eq!(is_member, true, "should be a member");
        assert_eq!(events.try_iter().count(), 1);

        Ok(())
    }

    #[async_std::test]
    async fn a_workspace_admin_can_add_another_user_as_admin() -> anyhow::Result<()> {
        use db::TeamRepo;

        let pool = mock_connection_pool()?;
        let (events, event_client) = mock_event_emitter();
        let requesting_user = mock_unprivileged_requesting_user().await?;
        let requesting_user_user = db::UserRepo::find_by_auth_id(&requesting_user.auth_id, &pool)
            .await?
            .ok_or_else(|| anyhow::anyhow!("user not found"))?;

        let workspace = WorkspaceRepo::create("", "", &pool).await?;
        TeamRepo::add_member(workspace.members, requesting_user_user.id, &pool).await?;
        TeamRepo::add_member(workspace.admins, requesting_user_user.id, &pool).await?;

        let user_id = Uuid::new_v4();
        change_workspace_membership(
            workspace.id,
            user_id,
            Role::Admin,
            &requesting_user,
            &pool,
            &event_client,
        )
        .await
        .unwrap();

        let is_admin = TeamRepo::is_member(workspace.admins, user_id, &pool)
            .await
            .unwrap();
        let is_member = TeamRepo::is_member(workspace.members, user_id, &pool)
            .await
            .unwrap();

        assert_eq!(is_admin, true, "should be an admin");
        assert_eq!(is_member, true, "should be a member");
        assert_eq!(events.try_iter().count(), 1);

        Ok(())
    }

    #[async_std::test]
    async fn a_workspace_admin_can_remove_another_user_from_the_workspace() -> anyhow::Result<()> {
        use db::TeamRepo;

        let pool = mock_connection_pool()?;
        let (events, event_client) = mock_event_emitter();
        let requesting_user = mock_unprivileged_requesting_user().await?;
        let requesting_user_user = db::UserRepo::find_by_auth_id(&requesting_user.auth_id, &pool)
            .await?
            .ok_or_else(|| anyhow::anyhow!("user not found"))?;

        let workspace = WorkspaceRepo::create("", "", &pool).await?;
        TeamRepo::add_member(workspace.members, requesting_user_user.id, &pool).await?;
        TeamRepo::add_member(workspace.admins, requesting_user_user.id, &pool).await?;

        let user_id = Uuid::new_v4();
        change_workspace_membership(
            workspace.id,
            user_id,
            Role::NonMember,
            &requesting_user,
            &pool,
            &event_client,
        )
        .await
        .unwrap();

        let is_admin = TeamRepo::is_member(workspace.admins, user_id, &pool)
            .await
            .unwrap();
        let is_member = TeamRepo::is_member(workspace.members, user_id, &pool)
            .await
            .unwrap();

        assert_eq!(is_admin, false, "should not be an admin");
        assert_eq!(is_member, false, "should not be a member");
        assert_eq!(events.try_iter().count(), 1);

        Ok(())
    }

    #[async_std::test]
    async fn a_platform_admin_can_add_a_member() -> anyhow::Result<()> {
        use db::TeamRepo;
        const NON_ADMIN_USER: &str = "1be12ec1-41bd-4384-b86f-de10fa754c12";

        let pool = mock_connection_pool()?;
        let (events, event_client) = mock_event_emitter();
        let requesting_user = mock_admin_requesting_user().await?;
        let user_id = Uuid::parse_str(NON_ADMIN_USER).unwrap();

        let workspace = WorkspaceRepo::create("", "", &pool).await?;
        TeamRepo::remove_member(workspace.admins, user_id, &pool).await?;
        TeamRepo::add_member(workspace.members, user_id, &pool).await?;
        change_workspace_membership(
            workspace.id,
            user_id,
            Role::NonAdmin,
            &requesting_user,
            &pool,
            &event_client,
        )
        .await
        .unwrap();

        let is_admin = TeamRepo::is_member(workspace.admins, user_id, &pool)
            .await
            .unwrap();
        let is_member = TeamRepo::is_member(workspace.members, user_id, &pool)
            .await
            .unwrap();

        assert_eq!(is_admin, false, "should not be an admin");
        assert_eq!(is_member, true, "should be a member");
        assert_eq!(events.try_iter().count(), 1);

        Ok(())
    }

    #[async_std::test]
    async fn a_platform_admin_can_add_an_admin() -> anyhow::Result<()> {
        use db::TeamRepo;
        const NON_ADMIN_USER: &str = "1be12ec1-41bd-4384-b86f-de10fa754c12";

        let pool = mock_connection_pool()?;
        let (events, event_client) = mock_event_emitter();
        let requesting_user = mock_admin_requesting_user().await?;
        let user_id = Uuid::parse_str(NON_ADMIN_USER).unwrap();

        let workspace = WorkspaceRepo::create("", "", &pool).await?;
        TeamRepo::remove_member(workspace.admins, user_id, &pool).await?;
        TeamRepo::add_member(workspace.members, user_id, &pool).await?;
        change_workspace_membership(
            workspace.id,
            user_id,
            Role::Admin,
            &requesting_user,
            &pool,
            &event_client,
        )
        .await
        .unwrap();

        let is_admin = TeamRepo::is_member(workspace.admins, user_id, &pool)
            .await
            .unwrap();
        let is_member = TeamRepo::is_member(workspace.members, user_id, &pool)
            .await
            .unwrap();

        assert_eq!(is_admin, true, "should be an admin");
        assert_eq!(is_member, true, "should be a member");
        assert_eq!(events.try_iter().count(), 1);

        Ok(())
    }
}
