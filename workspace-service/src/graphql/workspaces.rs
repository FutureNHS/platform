use crate::{
    core::workspace::{self, Role, WorkspaceService},
    db::{self, RepoFactory},
    graphql::{users::User, RequestingUser},
    services::workspace::WorkspaceServiceImpl,
};
use async_graphql::{Context, Enum, FieldResult, InputObject, Object, ID};
use fnhs_event_models::EventClient;
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

impl From<workspace::RoleFilter> for RoleFilter {
    fn from(f: workspace::RoleFilter) -> Self {
        use workspace::RoleFilter::*;
        match f {
            Admin => RoleFilter::Admin,
            NonAdmin => RoleFilter::NonAdmin,
        }
    }
}

impl From<RoleFilter> for workspace::RoleFilter {
    fn from(f: RoleFilter) -> Self {
        use workspace::RoleFilter::*;
        match f {
            RoleFilter::Admin => Admin,
            RoleFilter::NonAdmin => NonAdmin,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum WorkspaceMembership {
    /// Promote to admin
    Admin,
    /// Add as a non-admin member or demote an admin
    NonAdmin,
    /// Remove member
    NonMember,
}

impl From<WorkspaceMembership> for Role {
    fn from(role: WorkspaceMembership) -> Self {
        match role {
            WorkspaceMembership::Admin => Role::Admin,
            WorkspaceMembership::NonAdmin => Role::NonAdmin,
            WorkspaceMembership::NonMember => Role::NonMember,
        }
    }
}

impl From<Role> for WorkspaceMembership {
    fn from(role: Role) -> Self {
        match role {
            Role::Admin => WorkspaceMembership::Admin,
            Role::NonAdmin => WorkspaceMembership::NonAdmin,
            Role::NonMember => WorkspaceMembership::NonMember,
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
        let workspace_service = context.data::<WorkspaceServiceImpl>()?;
        let pool: &PgPool = context.data()?;
        let mut repos = RepoFactory::new(pool.begin().await?);
        let members = workspace_service
            .members(
                &mut repos,
                self.admins.into(),
                self.members.into(),
                filter.map(Into::into),
            )
            .await?;

        repos.commit().await?;

        Ok(members.iter().cloned().map(Into::into).collect())
    }
}

impl From<db::DbWorkspace> for Workspace {
    fn from(d: db::DbWorkspace) -> Self {
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
    new_role: WorkspaceMembership,
}

#[derive(Default)]
pub struct WorkspacesQuery;

#[Object]
impl WorkspacesQuery {
    /// Get all Workspaces
    async fn workspaces(&self, context: &Context<'_>) -> FieldResult<Vec<Workspace>> {
        let workspace_service = context.data::<WorkspaceServiceImpl>()?;
        let pool: &PgPool = context.data()?;
        let mut repos = RepoFactory::new(pool.begin().await?);
        let workspaces = workspace_service.find_all(&mut repos).await?;
        repos.commit().await?;
        Ok(workspaces.into_iter().map(Into::into).collect())
    }

    /// Get workspace by ID
    async fn workspace(&self, context: &Context<'_>, id: ID) -> FieldResult<Workspace> {
        self.get_workspace(context, id).await
    }

    #[graphql(entity)]
    async fn get_workspace(&self, context: &Context<'_>, id: ID) -> FieldResult<Workspace> {
        let workspace_service = context.data::<WorkspaceServiceImpl>()?;
        let pool: &PgPool = context.data()?;
        let mut repos = RepoFactory::new(pool.begin().await?);
        let id = Uuid::parse_str(id.as_str())?;
        let workspace = workspace_service.find_by_id(&mut repos, id.into()).await?;

        repos.commit().await?;

        Ok(workspace.into())
    }

    // Returns the requesting user's rights for a particular workspace.
    // Returns ADMIN if the user is_platform_admin.
    async fn requesting_user_workspace_rights(
        &self,
        context: &Context<'_>,
        workspace_id: ID,
    ) -> FieldResult<WorkspaceMembership> {
        todo!()
        // let requesting_user = context.data()?;
        // let pool = context.data()?;
        // let event_client = context.data()?;

        // requesting_user_workspace_rights(
        //     workspace_id.try_into()?,
        //     requesting_user,
        //     pool,
        //     event_client,
        // )
        // .await
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
        let pool: &PgPool = context.data()?;
        let mut repos = RepoFactory::new(pool.begin().await?);

        let new_workspace = workspace_service
            .create_workspace(
                &mut repos,
                &new_workspace.title,
                &new_workspace.description,
                requesting_user.auth_id.into(),
            )
            .await?;

        repos.commit().await?;

        Ok(new_workspace.into())
    }

    /// Update workspace (returns updated workspace)
    async fn update_workspace(
        &self,
        context: &Context<'_>,
        id: ID,
        workspace: UpdateWorkspace,
    ) -> FieldResult<Workspace> {
        let workspace_service = context.data::<WorkspaceServiceImpl>()?;
        let pool: &PgPool = context.data()?;
        let mut repos = RepoFactory::new(pool.begin().await?);
        let requesting_user = context.data::<RequestingUser>()?;
        let workspace = workspace_service
            .update(
                &mut repos,
                Uuid::parse_str(id.as_str())?.into(),
                &workspace.title,
                &workspace.description,
                requesting_user.auth_id.into(),
            )
            .await?;

        repos.commit().await?;

        // TODO: Add event
        Ok(workspace.into())
    }

    /// Delete workspace (returns deleted workspace)
    async fn delete_workspace(&self, context: &Context<'_>, id: ID) -> FieldResult<Workspace> {
        let workspace_service = context.data::<WorkspaceServiceImpl>()?;
        let pool: &PgPool = context.data()?;
        let mut repos = RepoFactory::new(pool.begin().await?);
        let requesting_user = context.data::<RequestingUser>()?;
        let workspace = workspace_service
            .delete(
                &mut repos,
                Uuid::parse_str(id.as_str())?.into(),
                requesting_user.auth_id.into(),
            )
            .await?;

        repos.commit().await?;

        // TODO: Add event
        Ok(workspace.into())
    }

    /// Changes workspace permissions for a user (Admin/NonAdmin/NonMember)
    async fn change_workspace_membership(
        &self,
        context: &Context<'_>,
        input: MembershipChange,
    ) -> FieldResult<Workspace> {
        let workspace_service = context.data::<WorkspaceServiceImpl>()?;
        let pool: &PgPool = context.data()?;
        let mut repos = RepoFactory::new(pool.begin().await?);
        let requesting_user = context.data::<RequestingUser>()?;
        let workspace_id: Uuid = input.workspace.try_into()?;
        let user_id: Uuid = input.user.try_into()?;
        let workspace = workspace_service
            .change_workspace_membership(
                &mut repos,
                workspace_id.into(),
                user_id.into(),
                input.new_role.into(),
                requesting_user.auth_id.into(),
            )
            .await?;

        repos.commit().await?;
        // let event_client: &EventClient = context.data()?;
        Ok(workspace.into())
    }
}

// pub async fn requesting_user_workspace_rights(
//     workspace_id: Uuid,
//     requesting_user: &RequestingUser,
//     pool: &PgPool,
//     _event_client: &EventClient,
// ) -> FieldResult<WorkspaceMembership> {
//     let user = db::UserRepo::find_by_auth_id(&requesting_user.auth_id, pool)
//         .await?
//         .ok_or_else(|| anyhow::anyhow!("user not found"))?;

//     if user.is_platform_admin {
//         return Ok(WorkspaceMembership::Admin);
//     }

//     let user_role = WorkspaceRepo::get_user_role(workspace_id, user.id, pool).await?;

//     Ok(user_role.into())
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::graphql::test_mocks::*;
//     use fnhs_event_models::EventData;

//     #[async_std::test]
//     async fn a_workspace_admin_cannot_demote_themselves_to_member() -> anyhow::Result<()> {
//         use db::TeamRepo;

//         let pool = mock_connection_pool()?;
//         let (events, event_client) = mock_event_emitter();
//         let requesting_user = mock_unprivileged_requesting_user().await?;
//         let requesting_user_user = user_repo
//             .find_by_auth_id(&requesting_user.auth_id, &pool)
//             .await?
//             .ok_or_else(|| anyhow::anyhow!("user not found"))?;

//         let workspace = WorkspaceRepo::create("", "", &pool).await?;
//         TeamRepo::add_member(workspace.members, requesting_user_user.id, &pool).await?;
//         TeamRepo::add_member(workspace.admins, requesting_user_user.id, &pool).await?;
//         let result = change_workspace_membership(
//             workspace.id,
//             requesting_user_user.id,
//             Role::NonAdmin,
//             &requesting_user,
//             &pool,
//             &event_client,
//         )
//         .await;

//         assert_eq!(
//             result.err().unwrap().message,
//             format!(
//                 "user with auth_id {} cannot demote themselves to NonAdmin",
//                 requesting_user.auth_id
//             )
//         );

//         assert_eq!(events.try_iter().count(), 0);

//         Ok(())
//     }

//     #[async_std::test]
//     async fn a_workspace_admin_cannot_demote_themselves_to_non_member() -> anyhow::Result<()> {
//         use db::TeamRepo;

//         let pool = mock_connection_pool()?;
//         let (events, event_client) = mock_event_emitter();
//         let requesting_user = mock_unprivileged_requesting_user().await?;
//         let requesting_user_user = user_repo
//             .find_by_auth_id(&requesting_user.auth_id, &pool)
//             .await?
//             .ok_or_else(|| anyhow::anyhow!("user not found"))?;

//         let workspace = WorkspaceRepo::create("", "", &pool).await?;
//         TeamRepo::add_member(workspace.members, requesting_user_user.id, &pool).await?;
//         TeamRepo::add_member(workspace.admins, requesting_user_user.id, &pool).await?;
//         let result = change_workspace_membership(
//             workspace.id,
//             requesting_user_user.id,
//             Role::NonMember,
//             &requesting_user,
//             &pool,
//             &event_client,
//         )
//         .await;

//         assert_eq!(
//             result.err().unwrap().message,
//             format!(
//                 "user with auth_id {} cannot demote themselves to NonMember",
//                 requesting_user.auth_id
//             )
//         );

//         assert_eq!(events.try_iter().count(), 0);

//         Ok(())
//     }

//     #[async_std::test]
//     async fn a_user_cannot_add_another_if_they_are_neither_site_nor_workspace_admin(
//     ) -> anyhow::Result<()> {
//         let pool = mock_connection_pool()?;
//         let (events, event_client) = mock_event_emitter();
//         let requesting_user = mock_unprivileged_requesting_user().await?;

//         let workspace = WorkspaceRepo::create("", "", &pool).await?;
//         let result = change_workspace_membership(
//             workspace.id,
//             Uuid::new_v4(),
//             Role::NonAdmin,
//             &requesting_user,
//             &pool,
//             &event_client,
//         )
//         .await;

//         assert_eq!(
//             result.err().unwrap().message,
//             format!(
//                 "user with auth_id {} does not have permission to update workspace membership",
//                 requesting_user.auth_id
//             )
//         );

//         assert_eq!(events.try_iter().count(), 0);

//         Ok(())
//     }

//     #[async_std::test]
//     async fn a_user_cannot_add_another_if_they_are_just_a_workspace_member() -> anyhow::Result<()> {
//         use db::TeamRepo;

//         let pool = mock_connection_pool()?;
//         let (events, event_client) = mock_event_emitter();
//         let requesting_user = mock_unprivileged_requesting_user().await?;
//         let user = user_repo
//             .find_by_auth_id(&requesting_user.auth_id, &pool)
//             .await?
//             .ok_or_else(|| anyhow::anyhow!("user not found"))?;

//         let workspace = WorkspaceRepo::create("", "", &pool).await?;
//         TeamRepo::add_member(workspace.members, user.id, &pool).await?;

//         let result = change_workspace_membership(
//             workspace.id,
//             Uuid::new_v4(),
//             Role::Admin,
//             &requesting_user,
//             &pool,
//             &event_client,
//         )
//         .await;

//         assert_eq!(
//             result.err().unwrap().message,
//             format!(
//                 "user with auth_id {} does not have permission to update workspace membership",
//                 requesting_user.auth_id
//             )
//         );

//         assert_eq!(events.try_iter().count(), 0);

//         Ok(())
//     }

//     #[async_std::test]
//     async fn a_workspace_admin_can_add_another_user_as_member() -> anyhow::Result<()> {
//         use db::TeamRepo;

//         let pool = mock_connection_pool()?;
//         let (events, event_client) = mock_event_emitter();
//         let requesting_user = mock_unprivileged_requesting_user().await?;
//         let requesting_user_user = user_repo
//             .find_by_auth_id(&requesting_user.auth_id, &pool)
//             .await?
//             .ok_or_else(|| anyhow::anyhow!("user not found"))?;

//         let workspace = WorkspaceRepo::create("", "", &pool).await?;
//         TeamRepo::add_member(workspace.members, requesting_user_user.id, &pool).await?;
//         TeamRepo::add_member(workspace.admins, requesting_user_user.id, &pool).await?;

//         let user_id = Uuid::new_v4();
//         change_workspace_membership(
//             workspace.id,
//             user_id,
//             Role::NonAdmin,
//             &requesting_user,
//             &pool,
//             &event_client,
//         )
//         .await
//         .unwrap();

//         let is_admin = TeamRepo::is_member(workspace.admins, user_id, &pool)
//             .await
//             .unwrap();
//         let is_member = TeamRepo::is_member(workspace.members, user_id, &pool)
//             .await
//             .unwrap();

//         assert_eq!(is_admin, false, "should not be an admin");
//         assert_eq!(is_member, true, "should be a member");
//         assert_eq!(events.try_iter().count(), 1);

//         Ok(())
//     }

//     #[async_std::test]
//     async fn a_workspace_admin_can_add_another_user_as_admin() -> anyhow::Result<()> {
//         use db::TeamRepo;

//         let pool = mock_connection_pool()?;
//         let (events, event_client) = mock_event_emitter();
//         let requesting_user = mock_unprivileged_requesting_user().await?;
//         let requesting_user_user = user_repo
//             .find_by_auth_id(&requesting_user.auth_id, &pool)
//             .await?
//             .ok_or_else(|| anyhow::anyhow!("user not found"))?;

//         let workspace = WorkspaceRepo::create("", "", &pool).await?;
//         TeamRepo::add_member(workspace.members, requesting_user_user.id, &pool).await?;
//         TeamRepo::add_member(workspace.admins, requesting_user_user.id, &pool).await?;

//         let user_id = Uuid::new_v4();
//         change_workspace_membership(
//             workspace.id,
//             user_id,
//             Role::Admin,
//             &requesting_user,
//             &pool,
//             &event_client,
//         )
//         .await
//         .unwrap();

//         let is_admin = TeamRepo::is_member(workspace.admins, user_id, &pool)
//             .await
//             .unwrap();
//         let is_member = TeamRepo::is_member(workspace.members, user_id, &pool)
//             .await
//             .unwrap();

//         assert_eq!(is_admin, true, "should be an admin");
//         assert_eq!(is_member, true, "should be a member");
//         assert_eq!(events.try_iter().count(), 1);

//         Ok(())
//     }

//     #[async_std::test]
//     async fn a_workspace_admin_can_remove_another_user_from_the_workspace() -> anyhow::Result<()> {
//         use db::TeamRepo;

//         let pool = mock_connection_pool()?;
//         let (events, event_client) = mock_event_emitter();
//         let requesting_user = mock_unprivileged_requesting_user().await?;
//         let requesting_user_user = user_repo
//             .find_by_auth_id(&requesting_user.auth_id, &pool)
//             .await?
//             .ok_or_else(|| anyhow::anyhow!("user not found"))?;

//         let workspace = WorkspaceRepo::create("", "", &pool).await?;
//         TeamRepo::add_member(workspace.members, requesting_user_user.id, &pool).await?;
//         TeamRepo::add_member(workspace.admins, requesting_user_user.id, &pool).await?;

//         let user_id = Uuid::new_v4();
//         change_workspace_membership(
//             workspace.id,
//             user_id,
//             Role::NonMember,
//             &requesting_user,
//             &pool,
//             &event_client,
//         )
//         .await
//         .unwrap();

//         let is_admin = TeamRepo::is_member(workspace.admins, user_id, &pool)
//             .await
//             .unwrap();
//         let is_member = TeamRepo::is_member(workspace.members, user_id, &pool)
//             .await
//             .unwrap();

//         assert_eq!(is_admin, false, "should not be an admin");
//         assert_eq!(is_member, false, "should not be a member");
//         assert_eq!(events.try_iter().count(), 1);

//         Ok(())
//     }

//     #[async_std::test]
//     async fn a_platform_admin_can_add_a_member() -> anyhow::Result<()> {
//         use db::TeamRepo;
//         const NON_ADMIN_USER: &str = "1be12ec1-41bd-4384-b86f-de10fa754c12";

//         let pool = mock_connection_pool()?;
//         let (events, event_client) = mock_event_emitter();
//         let requesting_user = mock_admin_requesting_user().await?;
//         let user_id = Uuid::parse_str(NON_ADMIN_USER).unwrap();

//         let workspace = WorkspaceRepo::create("", "", &pool).await?;
//         TeamRepo::remove_member(workspace.admins, user_id, &pool).await?;
//         TeamRepo::add_member(workspace.members, user_id, &pool).await?;
//         change_workspace_membership(
//             workspace.id,
//             user_id,
//             Role::NonAdmin,
//             &requesting_user,
//             &pool,
//             &event_client,
//         )
//         .await
//         .unwrap();

//         let is_admin = TeamRepo::is_member(workspace.admins, user_id, &pool)
//             .await
//             .unwrap();
//         let is_member = TeamRepo::is_member(workspace.members, user_id, &pool)
//             .await
//             .unwrap();

//         assert_eq!(is_admin, false, "should not be an admin");
//         assert_eq!(is_member, true, "should be a member");
//         assert_eq!(events.try_iter().count(), 1);

//         Ok(())
//     }

//     #[async_std::test]
//     async fn a_platform_admin_can_add_an_admin() -> anyhow::Result<()> {
//         use db::TeamRepo;
//         const NON_ADMIN_USER: &str = "1be12ec1-41bd-4384-b86f-de10fa754c12";

//         let pool = mock_connection_pool()?;
//         let (events, event_client) = mock_event_emitter();
//         let requesting_user = mock_admin_requesting_user().await?;
//         let user_id = Uuid::parse_str(NON_ADMIN_USER).unwrap();

//         let workspace = WorkspaceRepo::create("", "", &pool).await?;
//         TeamRepo::remove_member(workspace.admins, user_id, &pool).await?;
//         TeamRepo::add_member(workspace.members, user_id, &pool).await?;
//         change_workspace_membership(
//             workspace.id,
//             user_id,
//             Role::Admin,
//             &requesting_user,
//             &pool,
//             &event_client,
//         )
//         .await
//         .unwrap();

//         let is_admin = TeamRepo::is_member(workspace.admins, user_id, &pool)
//             .await
//             .unwrap();
//         let is_member = TeamRepo::is_member(workspace.members, user_id, &pool)
//             .await
//             .unwrap();

//         assert_eq!(is_admin, true, "should be an admin");
//         assert_eq!(is_member, true, "should be a member");
//         assert_eq!(events.try_iter().count(), 1);

//         Ok(())
//     }
// }
