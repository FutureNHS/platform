// sqlx::query_file_as!() causes spurious errors with this lint enabled
#![allow(clippy::suspicious_else_formatting)]

use crate::core::{
    team::TeamId,
    workspace::{Workspace, WorkspaceId, WorkspaceRepo},
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use sqlx::{types::Uuid, Postgres, Transaction};

#[derive(Clone)]
pub struct DbWorkspace {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub admins: Uuid,
    pub members: Uuid,
}

impl From<DbWorkspace> for Workspace {
    fn from(w: DbWorkspace) -> Self {
        Self {
            id: w.id.into(),
            title: w.title,
            description: w.description,
            admins: w.admins.into(),
            members: w.members.into(),
        }
    }
}

pub struct WorkspaceRepoImpl<'a, 'ex> {
    pub(crate) executor: &'a mut Transaction<'ex, Postgres>,
}

#[async_trait]
impl<'a, 'ex> WorkspaceRepo for WorkspaceRepoImpl<'a, 'ex> {
    async fn create(
        &mut self,
        title: &str,
        description: &str,
        admins_team_id: TeamId,
        members_team_id: TeamId,
    ) -> Result<Workspace> {
        let admins_team_id: Uuid = admins_team_id.into();
        let members_team_id: Uuid = members_team_id.into();
        let workspace = sqlx::query_file_as!(
            DbWorkspace,
            "sql/workspaces/create.sql",
            title,
            description,
            admins_team_id,
            members_team_id
        )
        .fetch_one(&mut *self.executor)
        .await
        .context("create workspace")?
        .into();

        Ok(workspace)
    }

    async fn find_all(&mut self) -> Result<Vec<Workspace>> {
        let workspaces: Vec<DbWorkspace> =
            sqlx::query_file_as!(DbWorkspace, "sql/workspaces/find_all.sql")
                .fetch_all(&mut *self.executor)
                .await
                .context("find all workspaces")?;

        Ok(workspaces.iter().cloned().map(Into::into).collect())
    }

    async fn find_by_id(&mut self, id: WorkspaceId) -> Result<Workspace> {
        let id: Uuid = id.into();
        let workspace = sqlx::query_file_as!(DbWorkspace, "sql/workspaces/find_by_id.sql", id)
            .fetch_one(&mut *self.executor)
            .await
            .context("find a workspace by id")?
            .into();

        Ok(workspace)
    }

    async fn update(
        &mut self,
        id: WorkspaceId,
        title: &str,
        description: &str,
    ) -> Result<Workspace> {
        let id: Uuid = id.into();
        let workspace = sqlx::query_file_as!(
            DbWorkspace,
            "sql/workspaces/update.sql",
            id,
            title,
            description
        )
        .fetch_one(&mut *self.executor)
        .await
        .context("update workspace")?
        .into();

        Ok(workspace)
    }

    async fn delete(&mut self, id: WorkspaceId) -> Result<Workspace> {
        let id: Uuid = id.into();
        let workspace = sqlx::query_file_as!(DbWorkspace, "sql/workspaces/delete.sql", id)
            .fetch_one(&mut *self.executor)
            .await
            .context("delete workspace")?
            .into();

        Ok(workspace)
    }
}
