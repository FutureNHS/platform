// sqlx::query_file_as!() causes spurious errors with this lint enabled
#![allow(clippy::suspicious_else_formatting)]

use crate::services::{
    team::TeamId,
    workspace::{Workspace, WorkspaceId, WorkspaceRepo},
};
use anyhow::{Context, Result};
use sqlx::{types::Uuid, Executor, Postgres};

#[derive(Clone)]
pub struct DbWorkspace {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub admins: Uuid,
    pub members: Uuid,
}

impl From<DbWorkspace> for Workspace {
    fn from(_: DbWorkspace) -> Self {
        todo!()
    }
}

pub struct WorkspaceRepoImpl {}

#[async_trait::async_trait]
impl WorkspaceRepo for WorkspaceRepoImpl {
    async fn create<'c, E>(
        title: &str,
        description: &str,
        admins_team_id: TeamId,
        members_team_id: TeamId,
        executor: E,
    ) -> Result<Workspace>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let admins_team_id: Uuid = admins_team_id.into();
        let members_team_id: Uuid = members_team_id.into();
        let workspace = sqlx::query_file_as!(
            Workspace,
            "sql/workspaces/create.sql",
            title,
            description,
            admins_team_id,
            members_team_id
        )
        .fetch_one(executor)
        .await
        .context("create workspace")?;

        Ok(workspace)
    }

    async fn find_all<'c, E>(executor: E) -> Result<Vec<Workspace>>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let workspaces = sqlx::query_file_as!(Workspace, "sql/workspaces/find_all.sql")
            .fetch_all(executor)
            .await
            .context("find all workspaces")?;

        Ok(workspaces)
    }

    async fn find_by_id<'c, E>(id: WorkspaceId, executor: E) -> Result<Workspace>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let id: Uuid = id.into();
        let workspace = sqlx::query_file_as!(Workspace, "sql/workspaces/find_by_id.sql", id)
            .fetch_one(executor)
            .await
            .context("find a workspace by id")?;

        Ok(workspace)
    }

    async fn update<'c, E>(
        id: WorkspaceId,
        title: &str,
        description: &str,
        executor: E,
    ) -> Result<Workspace>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let workspace = sqlx::query_file_as!(
            Workspace,
            "sql/workspaces/update.sql",
            id,
            title,
            description
        )
        .fetch_one(executor)
        .await
        .context("update workspace")?;

        Ok(workspace)
    }

    async fn delete<'c, E>(id: WorkspaceId, executor: E) -> Result<Workspace>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let workspace = sqlx::query_file_as!(Workspace, "sql/workspaces/delete.sql", id)
            .fetch_one(executor)
            .await
            .context("delete workspace")?;

        Ok(workspace)
    }
}
