use super::{
    team::TeamRepo,
    user::{AuthId, UserId, UserRepo},
    DB,
};
use anyhow::Result;
use async_trait::async_trait;
use derive_more::{Display, From, Into};
use fnhs_event_models::{Event, WorkspaceCreatedData};
use uuid::Uuid;

pub struct Workspace {
    pub id: WorkspaceId,
    pub title: String,
    pub description: String,
    pub admins: TeamId,
    pub members: TeamId,
}
#[derive(Copy, Clone)]
pub enum Role {
    /// User is a workspace administrator
    Admin,
    /// User is a workspace member
    NonAdmin,
    /// User is not a workspace member
    NonMember,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Role::Admin => "Admin",
                Role::NonAdmin => "NonAdmin",
                Role::NonMember => "NonMember",
            }
        )
    }
}

#[derive(From, Into, Display, Copy, Clone)]
pub struct WorkspaceId(Uuid);

#[derive(From, Into, Display)]
pub struct TeamId(Uuid);

#[async_trait::async_trait]
pub trait WorkspaceRepo<'c> {
    async fn create(
        &self,
        title: &str,
        description: &str,
        admins_team_id: TeamId,
        members_team_id: TeamId,
        executor: DB<'c>,
    ) -> Result<Workspace>;

    async fn find_all(&self, executor: DB<'c>) -> Result<Vec<Workspace>>;

    async fn find_by_id(&self, id: WorkspaceId, executor: DB<'c>) -> Result<Workspace>;

    async fn update(
        &self,
        id: WorkspaceId,
        title: &str,
        description: &str,
        executor: DB<'c>,
    ) -> Result<Workspace>;

    async fn delete(&self, id: WorkspaceId, executor: DB<'c>) -> Result<Workspace>;
}

#[async_trait::async_trait]
pub trait EventRepo {}

#[async_trait]
pub trait WorkspaceService {
    async fn create(
        &self,
        title: &str,
        description: &str,
        requesting_user: AuthId,
    ) -> Result<Workspace>;

    async fn is_admin(&self, workspace_id: WorkspaceId, user_id: UserId) -> Result<bool>;

    async fn change_workspace_membership(
        &self,
        workspace_id: WorkspaceId,
        user_id: UserId,
        new_role: Role,
        executor: DB<'c>,
    ) -> Result<Workspace>;
}

#[derive(Clone)]
pub struct WorkspaceServiceImpl<'c, E, T, U, W>
where
    E: EventRepo,
    T: TeamRepo,
    U: UserRepo,
    W: WorkspaceRepo<'c>,
{
    event_repo: E,
    team_repo: T,
    user_repo: U,
    workspace_repo: W,
}

impl<'c, E, T, U, W> WorkspaceServiceImpl<'c, E, T, U, W> {
    pub fn new(event_repo: E, team_repo: T, user_repo: U, workspace_repo: W) -> Self {
        Self {
            event_repo,
            team_repo,
            user_repo,
            workspace_repo,
        }
    }
}

#[async_trait]
impl<'c, E, T, U, W> WorkspaceService for WorkspaceServiceImpl<'c, E, T, U, W> {
    async fn create(
        &self,
        title: &str,
        description: &str,
        requesting_user: AuthId,
        executor: DB<'c>,
    ) -> Result<Workspace> {
        let user = self
            .user_repo
            .find_by_auth_id(&requesting_user)
            .await?
            .ok_or_else(|| anyhow::anyhow!("user not found"))?;
        if !user.is_platform_admin {
            return Err(anyhow::anyhow!(
                "User with auth_id {} does not have permission to create a workspace.",
                requesting_user,
            )
            .into());
        }
        let mut tx = executor.begin().await?;

        let admins = self
            .team_repo
            .create(&format!("{} Admins", title), &mut tx)
            .await?;
        let members = self
            .team_repo
            .create(&format!("{} Members", title), &mut tx)
            .await?;

        let workspace: Workspace = self.workspace_repo.create(title, description).await?.into();

        tx.commit().await?;

        self.event_client
            .publish_events(&[Event::new(
                workspace.id.to_string(),
                WorkspaceCreatedData {
                    workspace_id: workspace.id.to_string(),
                    user_id: requesting_user.to_string(),
                    title: workspace.title,
                },
            )])
            .await?;

        Ok(workspace)
    }

    async fn is_admin(
        &self,
        workspace_id: WorkspaceId,
        user_id: UserId,
        executor: DB<'c>,
    ) -> Result<bool> {
        match self.user_repo.find_by_id(&user_id, executor).await? {
            Some(user) => {
                let workspace = self
                    .workspace_repo
                    .find_by_id(workspace_id, executor)
                    .await?;
                self.team_repo
                    .is_member(workspace.admins, user.id, executor)
                    .await
            }
            None => Ok(false),
        }
    }

    async fn change_workspace_membership(
        &self,
        workspace_id: WorkspaceId,
        user_id: UserId,
        new_role: Role,
        executor: DB<'c>,
    ) -> Result<Workspace> {
        let mut tx = executor.begin().await?;

        let workspace = self
            .workspace_repo
            .find_by_id(workspace_id, executor)
            .await?;

        match new_role {
            Role::Admin => {
                self.team_repo
                    .add_member(workspace.admins, user_id, &mut tx)
                    .await?;
                self.team_repo
                    .add_member(workspace.members, user_id, &mut tx)
                    .await?;
            }
            Role::NonAdmin => {
                self.team_repo
                    .remove_member(workspace.admins, user_id, &mut tx)
                    .await?;
                self.team_repo
                    .add_member(workspace.members, user_id, &mut tx)
                    .await?;
            }
            Role::NonMember => {
                self.team_repo
                    .remove_member(workspace.admins, user_id, &mut tx)
                    .await?;
                self.team_repo
                    .remove_member(workspace.members, user_id, &mut tx)
                    .await?;
            }
        }

        tx.commit().await?;

        Ok(workspace)
    }
}
