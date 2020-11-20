use super::RepoFactory;
use crate::core::{
    team::TeamId,
    user::{AuthId, User, UserId},
};
use anyhow::Result;
use async_trait::async_trait;
use derive_more::{Display, From, Into};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum WorkspaceServiceError {
    #[error("requesting user {auth_id} does not have permission to {action}")]
    Unauthorized { auth_id: AuthId, action: String },
    #[error("user with auth_id {auth_id} cannot {action}")]
    CannotDemoteYourself { auth_id: AuthId, action: String },
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(From, Into, Default, Display, Copy, Clone, Debug, PartialEq)]
pub struct WorkspaceId(Uuid);

#[derive(Default)]
pub struct Workspace {
    pub id: WorkspaceId,
    pub title: String,
    pub description: String,
    pub admins: TeamId,
    pub members: TeamId,
}
#[derive(Copy, Clone, PartialEq)]
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

#[derive(Copy, Clone)]
pub enum RoleFilter {
    /// Only return Admins
    Admin,
    /// Only return Non-Admins
    NonAdmin,
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait WorkspaceRepo {
    async fn create(
        &mut self,
        title: &str,
        description: &str,
        admins_team_id: TeamId,
        members_team_id: TeamId,
    ) -> Result<Workspace>;

    async fn find_all(&mut self) -> Result<Vec<Workspace>>;

    async fn find_by_id(&mut self, id: WorkspaceId) -> Result<Workspace>;

    async fn update(
        &mut self,
        id: WorkspaceId,
        title: &str,
        description: &str,
    ) -> Result<Workspace>;

    async fn delete(&mut self, id: WorkspaceId) -> Result<Workspace>;
}

#[async_trait]
pub trait WorkspaceService<'a, 'b> {
    async fn find_all<T>(&self, repo_factory: &'a mut T) -> Result<Vec<Workspace>>
    where
        T: RepoFactory<'b> + Send,
        'b: 'a;

    async fn find_by_id<T>(&self, repo_factory: &'a mut T, id: WorkspaceId) -> Result<Workspace>
    where
        T: RepoFactory<'b> + Send,
        'b: 'a;

    async fn members<T>(
        &self,
        repo_factory: &'a mut T,
        admins: TeamId,
        members: TeamId,
        filter: Option<RoleFilter>,
    ) -> Result<Vec<User>>
    where
        T: RepoFactory<'b> + Send,
        'b: 'a;

    async fn create_workspace<T>(
        &self,
        repo_factory: &'a mut T,
        title: &str,
        description: &str,
        requesting_user: AuthId,
    ) -> Result<Workspace>
    where
        T: RepoFactory<'b> + Send,
        'b: 'a;

    async fn update<T>(
        &self,
        repo_factory: &'a mut T,
        id: WorkspaceId,
        title: &str,
        description: &str,
        requesting_user: AuthId,
    ) -> Result<Workspace>
    where
        T: RepoFactory<'b> + Send,
        'b: 'a;

    async fn delete<T>(
        &self,
        repo_factory: &'a mut T,
        id: WorkspaceId,
        requesting_user: AuthId,
    ) -> Result<Workspace>
    where
        T: RepoFactory<'b> + Send,
        'b: 'a;

    async fn change_workspace_membership<T>(
        &self,
        repo_factory: &'a mut T,
        workspace_id: WorkspaceId,
        user_id: UserId,
        new_role: Role,
        requesting_user: AuthId,
    ) -> Result<Workspace, WorkspaceServiceError>
    where
        T: RepoFactory<'b> + Send,
        'b: 'a;

    async fn requesting_user_workspace_rights<T>(
        &self,
        repo_factory: &'a mut T,
        workspace_id: WorkspaceId,
        requesting_user: AuthId,
    ) -> Result<Role>
    where
        T: RepoFactory<'b> + Send,
        'b: 'a;
}
