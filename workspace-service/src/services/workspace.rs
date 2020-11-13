use super::{
    team::{TeamId, TeamRepo},
    user::{AuthId, User, UserId, UserRepo},
};
use anyhow::Result;
use async_trait::async_trait;
use derive_more::{Display, From, Into};
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

#[derive(Copy, Clone)]
pub enum RoleFilter {
    /// Only return Admins
    Admin,
    /// Only return Non-Admins
    NonAdmin,
}

#[derive(From, Into, Display, Copy, Clone, Debug, PartialEq)]
pub struct WorkspaceId(Uuid);

#[cfg_attr(test, mockall::automock)]
pub trait RepoCreator<'a> {
    fn team<'r>(&'r mut self) -> Box<dyn TeamRepo + Send + 'r>
    where
        'a: 'r;

    fn user<'r>(&'r mut self) -> Box<dyn UserRepo + Send + 'r>
    where
        'a: 'r;

    fn workspace<'r>(&'r mut self) -> Box<dyn WorkspaceRepo + Send + 'r>
    where
        'a: 'r;
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
        T: RepoCreator<'b> + Send,
        'b: 'a;

    async fn find_by_id<T>(&self, repo_factory: &'a mut T, id: WorkspaceId) -> Result<Workspace>
    where
        T: RepoCreator<'b> + Send,
        'b: 'a;

    async fn members<T>(
        &self,
        repo_factory: &'a mut T,
        admins: TeamId,
        members: TeamId,
        filter: Option<RoleFilter>,
    ) -> Result<Vec<User>>
    where
        T: RepoCreator<'b> + Send,
        'b: 'a;

    async fn create_workspace<T>(
        &self,
        repo_factory: &'a mut T,
        title: &str,
        description: &str,
        requesting_user: AuthId,
    ) -> Result<Workspace>
    where
        T: RepoCreator<'b> + Send,
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
        T: RepoCreator<'b> + Send,
        'b: 'a;

    async fn delete<T>(
        &self,
        repo_factory: &'a mut T,
        id: WorkspaceId,
        requesting_user: AuthId,
    ) -> Result<Workspace>
    where
        T: RepoCreator<'b> + Send,
        'b: 'a;

    async fn change_workspace_membership<T>(
        &self,
        repo_factory: &'a mut T,
        workspace_id: WorkspaceId,
        user_id: UserId,
        new_role: Role,
        requesting_user: AuthId,
    ) -> Result<Workspace>
    where
        T: RepoCreator<'b> + Send,
        'b: 'a;
}

#[derive(Clone)]
pub struct WorkspaceServiceImpl {}

#[async_trait]
impl<'a, 'b> WorkspaceService<'a, 'b> for WorkspaceServiceImpl {
    async fn find_all<T>(&self, repo_factory: &'a mut T) -> Result<Vec<Workspace>>
    where
        T: RepoCreator<'b> + Send,
        'b: 'a,
    {
        let workspaces = repo_factory.workspace().find_all().await?;
        let workspaces = workspaces.into_iter().map(Into::into).collect();
        Ok(workspaces)
    }

    async fn find_by_id<T>(&self, repo_factory: &'a mut T, id: WorkspaceId) -> Result<Workspace>
    where
        T: RepoCreator<'b> + Send,
        'b: 'a,
    {
        let workspace = repo_factory.workspace().find_by_id(id).await?;
        Ok(workspace)
    }

    async fn members<T>(
        &self,
        repo_factory: &'a mut T,
        admins: TeamId,
        members: TeamId,
        filter: Option<RoleFilter>,
    ) -> Result<Vec<User>>
    where
        T: RepoCreator<'b> + Send,
        'b: 'a,
    {
        let users = match filter {
            Some(RoleFilter::Admin) => repo_factory.team().members(admins).await?,
            Some(RoleFilter::NonAdmin) => {
                repo_factory
                    .team()
                    .members_difference(members, admins)
                    .await?
            }
            None => repo_factory.team().members(members).await?,
        };
        let users = users.into_iter().map(Into::into).collect();
        Ok(users)
    }

    async fn create_workspace<T>(
        &self,
        repo_factory: &'a mut T,
        title: &str,
        description: &str,
        requesting_user: AuthId,
    ) -> Result<Workspace>
    where
        T: RepoCreator<'b> + Send,
        'b: 'a,
    {
        let user = repo_factory
            .user()
            .find_by_auth_id(requesting_user)
            .await?
            .ok_or_else(|| anyhow::anyhow!("user not found"))?;
        if !user.is_platform_admin {
            return Err(anyhow::anyhow!(
                "User with auth_id {} does not have permission to create a workspace.",
                requesting_user,
            ));
        }
        let admins = repo_factory
            .team()
            .create(&format!("{} Admins", title))
            .await?;
        let members = repo_factory
            .team()
            .create(&format!("{} Members", title))
            .await?;

        let workspace: Workspace = repo_factory
            .workspace()
            .create(title, description, admins.id, members.id)
            .await?;

        // self.event_client
        //     .publish_events(&[Event::new(
        //         workspace.id.to_string(),
        //         WorkspaceCreatedData {
        //             workspace_id: workspace.id.to_string(),
        //             user_id: requesting_user.to_string(),
        //             title: workspace.title,
        //         },
        //     )])
        //     .await?;

        Ok(workspace)
    }

    async fn update<T>(
        &self,
        repo_factory: &'a mut T,
        id: WorkspaceId,
        title: &str,
        description: &str,
        _requesting_user: AuthId,
    ) -> Result<Workspace>
    where
        T: RepoCreator<'b> + Send,
        'b: 'a,
    {
        let workspace = repo_factory
            .workspace()
            .update(id, title, description)
            .await?;
        Ok(workspace)
    }

    async fn delete<T>(
        &self,
        repo_factory: &'a mut T,
        id: WorkspaceId,
        _requesting_user: AuthId,
    ) -> Result<Workspace>
    where
        T: RepoCreator<'b> + Send,
        'b: 'a,
    {
        let workspace = repo_factory.workspace().delete(id).await?;
        Ok(workspace)
    }

    async fn change_workspace_membership<T>(
        &self,
        repo_factory: &'a mut T,
        workspace_id: WorkspaceId,
        user_id: UserId,
        new_role: Role,
        requesting_user: AuthId,
    ) -> Result<Workspace>
    where
        T: RepoCreator<'b> + Send,
        'b: 'a,
    {
        let requesting_user = repo_factory
            .user()
            .find_by_auth_id(requesting_user)
            .await?
            .ok_or_else(|| anyhow::anyhow!("user not found"))?;

        let user = repo_factory.user().find_by_id(user_id).await?;
        let is_workspace_admin = match user {
            Some(user) => {
                let workspace = repo_factory.workspace().find_by_id(workspace_id).await?;
                repo_factory
                    .team()
                    .is_member(workspace.admins, user.id)
                    .await?
            }
            None => false,
        };

        if !requesting_user.is_platform_admin && !is_workspace_admin {
            return Err(anyhow::anyhow!(
                "user with auth_id {} does not have permission to update workspace membership",
                requesting_user.auth_id,
            ));
        }

        if requesting_user.id == user_id {
            return Err(anyhow::anyhow!(
                "user with auth_id {} cannot demote themselves to {}",
                requesting_user.auth_id,
                new_role
            ));
        }

        let workspace = repo_factory.workspace().find_by_id(workspace_id).await?;

        match new_role {
            Role::Admin => {
                repo_factory
                    .team()
                    .add_member(workspace.admins, user_id)
                    .await?;
                repo_factory
                    .team()
                    .add_member(workspace.members, user_id)
                    .await?;
            }
            Role::NonAdmin => {
                repo_factory
                    .team()
                    .remove_member(workspace.admins, user_id)
                    .await?;
                repo_factory
                    .team()
                    .add_member(workspace.members, user_id)
                    .await?;
            }
            Role::NonMember => {
                repo_factory
                    .team()
                    .remove_member(workspace.admins, user_id)
                    .await?;
                repo_factory
                    .team()
                    .remove_member(workspace.members, user_id)
                    .await?;
            }
        }

        // self.event_client
        //     .publish_events(&[Event::new(
        //         workspace.id.clone(),
        //         WorkspaceMembershipChangedData {
        //             requesting_user_id: requesting_user.auth_id.to_string(),
        //             affected_workspace_id: workspace.id.clone().into(),
        //             affected_user_id: user_id.to_string(),
        //             affected_role: new_role.to_string(),
        //         },
        //     )])
        //     .await?;

        Ok(workspace)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::services::{team::*, user::*};
    use mockall::predicate::*;

    #[async_std::test]
    async fn creating_workspace_emits_an_event() -> anyhow::Result<()> {
        let service = WorkspaceServiceImpl {};
        let requesting_user: AuthId = Uuid::parse_str("feedface-0000-0000-0000-000000000000")
            .unwrap()
            .into();

        let admins_team_id: TeamId = Uuid::new_v4().into();
        let members_team_id: TeamId = Uuid::new_v4().into();
        let workspace_id: WorkspaceId = Uuid::new_v4().into();

        let mut repos = MockRepoCreator::new();

        let mut user_repo = MockUserRepo::new();
        user_repo
            .expect_find_by_auth_id()
            .with(eq(requesting_user))
            .return_once(|auth_id| {
                Ok(Some(User {
                    auth_id,
                    id: Uuid::new_v4().into(),
                    email_address: "".to_string(),
                    name: "".to_string(),
                    is_platform_admin: true,
                }))
            });
        repos.expect_user().return_once(move || Box::new(user_repo));

        repos.expect_team().times(2).returning(move || {
            let mut team_repo = MockTeamRepo::new();
            team_repo
                .expect_create()
                .with(eq("my workspace Admins"))
                .returning(move |title| {
                    Ok(Team {
                        id: admins_team_id,
                        title: title.to_string(),
                    })
                });
            team_repo
                .expect_create()
                .with(eq("my workspace Members"))
                .returning(move |title| {
                    Ok(Team {
                        id: members_team_id,
                        title: title.to_string(),
                    })
                });
            Box::new(team_repo)
        });

        let mut workspace_repo = MockWorkspaceRepo::new();
        workspace_repo
            .expect_create()
            .with(
                eq("my workspace"),
                eq("description"),
                eq(admins_team_id),
                eq(members_team_id),
            )
            .return_once(move |title, description, admins, members| {
                Ok(Workspace {
                    id: workspace_id,
                    title: title.to_string(),
                    description: description.to_string(),
                    admins,
                    members,
                })
            });
        repos
            .expect_workspace()
            .return_once(move || Box::new(workspace_repo));

        let workspace = service
            .create_workspace(&mut repos, "my workspace", "description", requesting_user)
            .await?;

        assert_eq!(workspace.id, workspace_id);
        assert_eq!(workspace.title, "my workspace");
        assert_eq!(workspace.description, "description");
        assert_eq!(workspace.members, members_team_id);
        assert_eq!(workspace.admins, admins_team_id);

        // assert!(events
        //     .try_iter()
        //     .any(|e| matches!(e.data, EventData::WorkspaceCreated(_))));

        Ok(())
    }

    #[async_std::test]
    async fn creating_workspace_as_non_admin_fails() -> anyhow::Result<()> {
        let service = WorkspaceServiceImpl {};
        let requesting_user: AuthId = Uuid::parse_str("deadbeef-0000-0000-0000-000000000000")
            .unwrap()
            .into();
        let mut user_repo = MockUserRepo::new();
        user_repo
            .expect_find_by_auth_id()
            .with(eq(requesting_user))
            .returning(|auth_id| {
                Ok(Some(User {
                    auth_id,
                    id: Uuid::new_v4().into(),
                    email_address: "".to_string(),
                    name: "".to_string(),
                    is_platform_admin: false,
                }))
            });
        let mut repos = MockRepoCreator::new();
        repos.expect_user().return_once(move || Box::new(user_repo));

        let result = service
            .create_workspace(&mut repos, "title", "description", requesting_user)
            .await;

        assert_eq!(result.err().unwrap().to_string(), "User with auth_id deadbeef-0000-0000-0000-000000000000 does not have permission to create a workspace.");

        // assert_eq!(events.try_iter().count(), 0);

        Ok(())
    }
}
