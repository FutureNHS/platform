// sqlx::query_file_as!() causes spurious errors with this lint enabled
#![allow(clippy::suspicious_else_formatting)]

use crate::db;
use anyhow::{Context, Result};
use sqlx::{types::Uuid, PgPool, Postgres, Transaction};
use std::fmt::Display;

#[derive(Clone)]
pub struct Workspace {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub admins: Uuid,
    pub members: Uuid,
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

impl Display for Role {
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

struct RepoFactory<'conn> {
    pub(crate) executor: Transaction<'conn, Postgres>,
}

impl<'conn> RepoFactory<'conn> {
    fn team<'r>(&'r mut self) -> TeamRepo<'r>
    where
        'conn: 'r,
    {
        TeamRepo {
            executor: &mut self.executor,
        }
    }
}

#[cfg_attr(test, allow(dead_code))]
pub struct WorkspaceRepo {}

#[cfg_attr(test, allow(dead_code))]
impl WorkspaceRepo {
    pub async fn create(title: &str, description: &str, pool: &PgPool) -> Result<Workspace> {
        let tx: Transaction<Postgres> = pool.begin().await?;

        let mut fact = RepoFactory { executor: tx };
        let mut team_repo = fact.team();
        let admins = team_repo.create(&format!("{} Admins", title)).await?;
        drop(team_repo);

        let mut team_repo = fact.team();
        let members = team_repo.create(&format!("{} Members", title)).await?;
        drop(team_repo);

        let workspace = sqlx::query_file_as!(
            Workspace,
            "sql/workspaces/create.sql",
            title,
            description,
            admins.id,
            members.id
        )
        .fetch_one(&mut fact.executor)
        .await
        .context("create workspace")?;
        fact.executor.commit().await?;

        Ok(workspace)
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<Workspace>> {
        let workspaces = sqlx::query_file_as!(Workspace, "sql/workspaces/find_all.sql")
            .fetch_all(pool)
            .await
            .context("find all workspaces")?;

        Ok(workspaces)
    }

    pub async fn find_by_id(id: Uuid, pool: &PgPool) -> Result<Workspace> {
        let workspace = sqlx::query_file_as!(Workspace, "sql/workspaces/find_by_id.sql", id)
            .fetch_one(pool)
            .await
            .context("find a workspace by id")?;

        Ok(workspace)
    }

    pub async fn update(
        id: Uuid,
        title: &str,
        description: &str,
        pool: &PgPool,
    ) -> Result<Workspace> {
        let workspace = sqlx::query_file_as!(
            Workspace,
            "sql/workspaces/update.sql",
            id,
            title,
            description
        )
        .fetch_one(pool)
        .await
        .context("update workspace")?;

        Ok(workspace)
    }

    pub async fn delete(id: Uuid, pool: &PgPool) -> Result<Workspace> {
        let workspace = sqlx::query_file_as!(Workspace, "sql/workspaces/delete.sql", id)
            .fetch_one(pool)
            .await
            .context("delete workspace")?;

        Ok(workspace)
    }

    pub async fn is_admin(workspace_id: Uuid, user_id: Uuid, pool: &PgPool) -> Result<bool> {
        match db::UserRepo::find_by_id(&user_id, pool).await? {
            Some(user) => {
                let workspace = WorkspaceRepo::find_by_id(workspace_id, pool).await?;
                db::TeamRepo::is_member(workspace.admins, user.id, pool).await
            }
            None => Ok(false),
        }
    }

    pub async fn change_workspace_membership(
        workspace_id: Uuid,
        user_id: Uuid,
        new_role: Role,
        pool: &PgPool,
    ) -> Result<Workspace> {
        let mut tx = pool.begin().await?;

        let workspace = WorkspaceRepo::find_by_id(workspace_id, pool).await?;

        match new_role {
            Role::Admin => {
                db::TeamRepo::add_member(workspace.admins, user_id, &mut tx).await?;
                db::TeamRepo::add_member(workspace.members, user_id, &mut tx).await?;
            }
            Role::NonAdmin => {
                db::TeamRepo::remove_member(workspace.admins, user_id, &mut tx).await?;
                db::TeamRepo::add_member(workspace.members, user_id, &mut tx).await?;
            }
            Role::NonMember => {
                db::TeamRepo::remove_member(workspace.admins, user_id, &mut tx).await?;
                db::TeamRepo::remove_member(workspace.members, user_id, &mut tx).await?;
            }
        }

        tx.commit().await?;

        Ok(workspace)
    }
}

// Fake implementation for tests. If you want integration tests that exercise the database,
// see https://doc.rust-lang.org/rust-by-example/testing/integration_testing.html.
#[cfg(test)]
pub struct WorkspaceRepoFake {}

#[cfg(test)]
use std::collections::HashMap;
#[cfg(test)]
use std::sync::Mutex;

use super::TeamRepo;

#[cfg(test)]
lazy_static::lazy_static! {
    static ref WORKSPACES: Mutex<HashMap<Uuid, Workspace>> = Mutex::new(HashMap::new());
}

#[cfg(test)]
impl WorkspaceRepoFake {
    pub async fn create(title: &str, description: &str, _pool: &PgPool) -> Result<Workspace> {
        let workspace = Workspace {
            id: Uuid::new_v4(),
            title: title.to_string(),
            description: description.to_string(),
            admins: Uuid::new_v4(),
            members: Uuid::new_v4(),
        };
        let mut teams = WORKSPACES.lock().unwrap();
        teams.insert(workspace.id, workspace.clone());
        Ok(workspace)
    }

    pub async fn find_all(_pool: &PgPool) -> Result<Vec<Workspace>> {
        Ok(vec![])
    }

    pub async fn find_by_id(id: Uuid, _pool: &PgPool) -> Result<Workspace> {
        let teams = WORKSPACES.lock().unwrap();
        Ok(teams.get(&id).unwrap().clone())
    }

    pub async fn update(
        id: Uuid,
        title: &str,
        description: &str,
        _pool: &PgPool,
    ) -> Result<Workspace> {
        let workspace = Workspace {
            id,
            title: title.to_string(),
            description: description.to_string(),
            admins: Uuid::new_v4(),
            members: Uuid::new_v4(),
        };
        Ok(workspace)
    }

    pub async fn delete(id: Uuid, _pool: &PgPool) -> Result<Workspace> {
        let workspace = Workspace {
            id,
            title: "fake deleted workspace".into(),
            description: "fake deleted workspace for tests".into(),
            admins: Uuid::new_v4(),
            members: Uuid::new_v4(),
        };
        Ok(workspace)
    }

    pub async fn is_admin(workspace_id: Uuid, user_id: Uuid, pool: &PgPool) -> Result<bool> {
        match db::UserRepo::find_by_id(&user_id, pool).await? {
            Some(user) => {
                let workspace = WorkspaceRepoFake::find_by_id(workspace_id, pool).await?;
                db::TeamRepo::is_member(workspace.admins, user.id, pool).await
            }
            None => Ok(false),
        }
    }

    pub async fn change_workspace_membership(
        workspace_id: Uuid,
        user_id: Uuid,
        new_role: Role,
        pool: &PgPool,
    ) -> Result<Workspace> {
        let workspace = WorkspaceRepoFake::find_by_id(workspace_id, pool).await?;
        match new_role {
            Role::Admin => {
                db::TeamRepo::add_member(workspace.admins, user_id, pool).await?;
                db::TeamRepo::add_member(workspace.members, user_id, pool).await?;
            }
            Role::NonAdmin => {
                db::TeamRepo::remove_member(workspace.admins, user_id, pool).await?;
                db::TeamRepo::add_member(workspace.members, user_id, pool).await?;
            }
            Role::NonMember => {
                db::TeamRepo::remove_member(workspace.admins, user_id, pool).await?;
                db::TeamRepo::remove_member(workspace.members, user_id, pool).await?;
            }
        }

        Ok(workspace)
    }
}
