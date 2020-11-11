// sqlx::query_file_as!() causes spurious errors with this lint enabled
#![allow(clippy::suspicious_else_formatting)]

use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::{types::Uuid, Executor, Postgres};

#[derive(Clone)]
pub struct FileVersion {
    pub id: Uuid,
    pub folder: Uuid,
    pub file: Uuid,
    pub file_title: String,
    pub file_description: String,
    pub file_name: String,
    pub file_type: String,
    pub blob_storage_path: String,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub version_number: i16,
    pub version_label: String,
}

pub struct FileVersionRepo {}

impl FileVersionRepo {
    #[allow(clippy::too_many_arguments)]
    pub async fn create<'c, E>(
        id: Uuid,
        folder: Uuid,
        file: Uuid,
        file_title: &str,
        file_description: &str,
        file_name: &str,
        file_type: &str,
        blob_storage_path: &str,
        created_by: Uuid,
        version_number: i16,
        version_label: &str,
        executor: E,
    ) -> Result<FileVersion>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let file_version = sqlx::query_file_as!(
            FileVersion,
            "sql/file_versions/create.sql",
            id,
            folder,
            file,
            file_title,
            file_description,
            file_name,
            file_type,
            blob_storage_path,
            created_by,
            version_number,
            version_label,
        )
        .fetch_one(executor)
        .await?;

        Ok(file_version)
    }
}
