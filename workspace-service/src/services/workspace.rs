use crate::core::{
    team::TeamId,
    user::{AuthId, User, UserId},
    workspace::{Role, RoleFilter, Workspace, WorkspaceId, WorkspaceService},
    RepoCreator,
};
use anyhow::Result;
use async_trait::async_trait;

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

        let workspace = repo_factory.workspace().find_by_id(workspace_id).await?;

        if !requesting_user.is_platform_admin
            && !repo_factory
                .team()
                .is_member(workspace.admins, requesting_user.id)
                .await?
        {
            return Err(anyhow::anyhow!(
                "user with auth_id {} does not have permission to update workspace membership",
                requesting_user.auth_id,
            ));
        }

        if new_role != Role::Admin && requesting_user.id == user_id {
            return Err(anyhow::anyhow!(
                "user with auth_id {} cannot demote themselves to {}",
                requesting_user.auth_id,
                new_role
            ));
        }

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
    use crate::core::{team::*, user::*, workspace::*, *};
    use mockall::predicate::*;
    use uuid::Uuid;

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

        repos.expect_user().return_once(move || {
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
            Box::new(user_repo)
        });

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

        repos.expect_workspace().return_once(move || {
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
            Box::new(workspace_repo)
        });

        let actual = service
            .create_workspace(&mut repos, "my workspace", "description", requesting_user)
            .await?;

        assert_eq!(actual.id, workspace_id);
        assert_eq!(actual.title, "my workspace");
        assert_eq!(actual.description, "description");
        assert_eq!(actual.members, members_team_id);
        assert_eq!(actual.admins, admins_team_id);

        // assert!(events
        //     .try_iter()
        //     .any(|e| matches!(e.data, EventData::WorkspaceCreated(_))));

        Ok(())
    }

    #[async_std::test]
    async fn creating_workspace_as_non_platform_admin_fails() -> anyhow::Result<()> {
        let service = WorkspaceServiceImpl {};
        let requesting_user: AuthId = Uuid::parse_str("deadbeef-0000-0000-0000-000000000000")
            .unwrap()
            .into();

        let mut repos = MockRepoCreator::new();

        repos.expect_user().return_once(move || {
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
            Box::new(user_repo)
        });

        let actual = service
            .create_workspace(&mut repos, "title", "description", requesting_user)
            .await
            .err()
            .unwrap()
            .to_string();
        let expected = "User with auth_id deadbeef-0000-0000-0000-000000000000 does not have permission to create a workspace.";

        assert_eq!(actual, expected);

        // assert_eq!(events.try_iter().count(), 0);

        Ok(())
    }

    #[async_std::test]
    async fn a_workspace_admin_cannot_demote_themselves_to_member() -> anyhow::Result<()> {
        let service = WorkspaceServiceImpl {};

        let requesting_user: AuthId = Uuid::new_v4().into();
        let user_id: UserId = Uuid::new_v4().into();
        let admins_team_id: TeamId = Uuid::new_v4().into();
        let members_team_id: TeamId = Uuid::new_v4().into();
        let workspace_id: WorkspaceId = Uuid::new_v4().into();

        let mut repos = MockRepoCreator::new();

        repos.expect_user().returning(move || {
            let mut user_repo = MockUserRepo::new();
            user_repo
                .expect_find_by_auth_id()
                .with(eq(requesting_user))
                .return_once(move |auth_id| {
                    Ok(Some(User {
                        auth_id,
                        id: user_id,
                        email_address: "".to_string(),
                        name: "".to_string(),
                        is_platform_admin: false,
                    }))
                });
            user_repo
                .expect_find_by_id()
                .with(eq(user_id))
                .return_once(move |id| {
                    Ok(Some(User {
                        auth_id: requesting_user,
                        id,
                        email_address: "".to_string(),
                        name: "".to_string(),
                        is_platform_admin: false,
                    }))
                });
            Box::new(user_repo)
        });

        repos.expect_team().return_once(move || {
            let mut team_repo = MockTeamRepo::new();
            team_repo
                .expect_is_member()
                .with(eq(admins_team_id), eq(user_id))
                .return_once(|_, _| Ok(true));
            Box::new(team_repo)
        });

        repos.expect_workspace().return_once(move || {
            let mut workspace_repo = MockWorkspaceRepo::new();
            workspace_repo
                .expect_find_by_id()
                .with(eq(workspace_id))
                .return_once(move |id| {
                    Ok(Workspace {
                        id,
                        title: "title".to_string(),
                        description: "description".to_string(),
                        admins: admins_team_id,
                        members: members_team_id,
                    })
                });
            Box::new(workspace_repo)
        });

        let actual = service
            .change_workspace_membership(
                &mut repos,
                workspace_id,
                user_id,
                Role::NonAdmin,
                requesting_user,
            )
            .await
            .err()
            .unwrap()
            .to_string();
        let expected = format!(
            "user with auth_id {} cannot demote themselves to NonAdmin",
            requesting_user
        );

        assert_eq!(actual, expected);

        // assert_eq!(events.try_iter().count(), 0);

        Ok(())
    }
}
