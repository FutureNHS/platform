// sqlx::query_file_as!() causes spurious errors with this lint enabled
#![allow(clippy::suspicious_else_formatting)]

use anyhow::Result;
use sqlx::{types::Uuid, PgPool};

#[derive(Clone)]
pub struct Folder {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub workspace: Uuid,
}

pub struct FolderRepo {}

impl FolderRepo {
    pub async fn create(
        title: &str,
        description: &str,
        workspace: Uuid,
        pool: &PgPool,
    ) -> Result<Folder> {
        let folder = sqlx::query_file_as!(
            Folder,
            "sql/folders/create.sql",
            title,
            description,
            workspace
        )
        .fetch_one(pool)
        .await?;

        Ok(folder)
    }

    pub async fn find_by_workspace(workspace: Uuid, pool: &PgPool) -> Result<Vec<Folder>> {
        let folders = sqlx::query_file_as!(Folder, "sql/folders/find_by_workspace.sql", workspace)
            .fetch_all(pool)
            .await?;

        Ok(folders)
    }

    pub async fn find_by_id(id: Uuid, pool: &PgPool) -> Result<Folder> {
        let folder = sqlx::query_file_as!(Folder, "sql/folders/find_by_id.sql", id)
            .fetch_one(pool)
            .await?;

        Ok(folder)
    }

    pub async fn update(id: Uuid, title: &str, description: &str, pool: &PgPool) -> Result<Folder> {
        let folder = sqlx::query_file_as!(Folder, "sql/folders/update.sql", id, title, description)
            .fetch_one(pool)
            .await?;

        Ok(folder)
    }

    pub async fn delete(id: Uuid, pool: &PgPool) -> Result<Folder> {
        let folder = sqlx::query_file_as!(Folder, "sql/folders/delete.sql", id)
            .fetch_one(pool)
            .await?;

        Ok(folder)
    }
}
