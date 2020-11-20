// sqlx::query_file_as!() causes spurious errors with this lint enabled
#![allow(clippy::suspicious_else_formatting)]

use crate::core::{
    folder::{Folder, FolderId, FolderRepo},
    workspace::WorkspaceId,
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use sqlx::{types::Uuid, Postgres, Transaction};

#[derive(Clone)]
pub struct DbFolder {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub role_required: String,
    pub workspace: Uuid,
}

impl From<DbFolder> for Folder {
    fn from(f: DbFolder) -> Self {
        Self {
            id: f.id.into(),
            title: f.title,
            description: f.description,
            role_required: f.role_required,
            workspace: f.workspace.into(),
        }
    }
}

pub struct FolderRepoImpl<'a, 'ex> {
    pub(crate) executor: &'a mut Transaction<'ex, Postgres>,
}

#[async_trait]
impl<'a, 'ex> FolderRepo for FolderRepoImpl<'a, 'ex> {
    async fn create(
        &mut self,
        title: &str,
        description: &str,
        role_required: &str,
        workspace: WorkspaceId,
    ) -> Result<Folder> {
        let workspace_id: Uuid = workspace.into();
        let folder = sqlx::query_file_as!(
            DbFolder,
            "sql/folders/create.sql",
            title,
            description,
            role_required,
            workspace_id,
        )
        .fetch_one(&mut *self.executor)
        .await
        .context("create folder")?
        .into();

        Ok(folder)
    }

    async fn find_by_workspace(&mut self, workspace: WorkspaceId) -> Result<Vec<Folder>> {
        let workspace_id: Uuid = workspace.into();
        let folders =
            sqlx::query_file_as!(DbFolder, "sql/folders/find_by_workspace.sql", workspace_id)
                .fetch_all(&mut *self.executor)
                .await
                .context("find folders for workspace")?;
        let folders = folders.iter().cloned().map(Into::into).collect();

        Ok(folders)
    }

    async fn find_by_id(&mut self, id: FolderId) -> Result<Folder> {
        let folder_id: Uuid = id.into();
        let folder = sqlx::query_file_as!(DbFolder, "sql/folders/find_by_id.sql", folder_id)
            .fetch_one(&mut *self.executor)
            .await
            .context("find folder by id")?
            .into();

        Ok(folder)
    }

    async fn update(
        &mut self,
        id: FolderId,
        title: &str,
        description: &str,
        role_required: &str,
    ) -> Result<Folder> {
        let folder_id: Uuid = id.into();
        let folder = sqlx::query_file_as!(
            DbFolder,
            "sql/folders/update.sql",
            folder_id,
            title,
            description,
            role_required,
        )
        .fetch_one(&mut *self.executor)
        .await
        .context("update folder")?
        .into();

        Ok(folder)
    }

    async fn delete(&mut self, id: FolderId) -> Result<Folder> {
        let folder_id: Uuid = id.into();
        let folder = sqlx::query_file_as!(DbFolder, "sql/folders/delete.sql", folder_id)
            .fetch_one(&mut *self.executor)
            .await
            .context("delete folder")?
            .into();

        Ok(folder)
    }
}
