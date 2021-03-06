// sqlx::query_file_as!() causes spurious errors with this lint enabled
#![allow(clippy::suspicious_else_formatting)]

use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::{types::Uuid, Executor, PgPool, Postgres};

#[derive(Clone)]
pub struct File {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
    pub latest_version: Uuid,
}

#[cfg_attr(test, allow(dead_code))]
pub struct FileRepo {}

#[cfg_attr(test, allow(dead_code))]
impl FileRepo {
    async fn create<'c, E>(created_by: Uuid, latest_version: Uuid, executor: E) -> Result<File>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let file = sqlx::query_file_as!(File, "sql/files/create.sql", created_by, latest_version)
            .fetch_one(executor)
            .await?;

        Ok(file)
    }

    async fn update_latest_version<'c, E>(
        id: Uuid,
        current_latest_version: Uuid,
        new_latest_version: Uuid,
        executor: E,
    ) -> Result<File>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let file = sqlx::query_file_as!(
            File,
            "sql/files/update_latest_version.sql",
            id,
            current_latest_version,
            new_latest_version
        )
        .fetch_one(executor)
        .await?;

        Ok(file)
    }
}

#[derive(Clone)]
pub struct FileWithVersion {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub folder: Uuid,
    pub file_name: String,
    pub file_type: String,
    pub blob_storage_path: String,
    pub version: Uuid,
    pub version_number: i16,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<(File, super::FileVersion)> for FileWithVersion {
    fn from(pair: (File, super::FileVersion)) -> Self {
        let (file, file_version) = pair;
        Self {
            id: file.id,
            title: file_version.file_title,
            description: file_version.file_description,
            folder: file_version.folder,
            file_name: file_version.file_name,
            file_type: file_version.file_type,
            blob_storage_path: file_version.blob_storage_path,
            version: file_version.id,
            version_number: file_version.version_number,
            created_at: file.created_at,
            modified_at: file_version.created_at,
            deleted_at: file.deleted_at,
        }
    }
}

#[derive(Clone)]
pub struct CreateFileArgs<'a> {
    pub user_id: Uuid,
    pub folder_id: Uuid,
    pub title: &'a str,
    pub description: &'a str,
    pub file_name: &'a str,
    pub file_type: &'a str,
    pub blob_storage_path: &'a str,
}

#[derive(Clone)]
pub struct CreateFileVersionArgs<'a> {
    pub user_id: Uuid,
    pub file_id: Uuid,
    pub latest_version: Uuid,
    pub folder_id: Uuid,
    pub title: &'a str,
    pub description: &'a str,
    pub file_name: &'a str,
    pub file_type: &'a str,
    pub blob_storage_path: &'a str,
    pub version_number: i16,
}

#[cfg_attr(test, allow(dead_code))]
pub struct FileWithVersionRepo {}

#[cfg_attr(test, allow(dead_code))]
impl FileWithVersionRepo {
    pub async fn create(args: CreateFileArgs<'_>, pool: &PgPool) -> Result<FileWithVersion> {
        let version_id = Uuid::new_v4();

        let mut tx = pool.begin().await?;
        super::defer_all_constraints(&mut tx).await?;
        let file = FileRepo::create(args.user_id, version_id, &mut tx).await?;
        let file_version = super::FileVersionRepo::create(
            version_id,
            args.folder_id,
            file.id,
            args.title,
            args.description,
            args.file_name,
            args.file_type,
            args.blob_storage_path,
            args.user_id,
            1,
            "",
            &mut tx,
        )
        .await?;
        tx.commit().await?;

        Ok((file, file_version).into())
    }

    pub async fn create_version(
        args: CreateFileVersionArgs<'_>,
        pool: &PgPool,
    ) -> Result<FileWithVersion> {
        let mut tx = pool.begin().await?;
        let file_version = super::FileVersionRepo::create(
            Uuid::new_v4(),
            args.folder_id,
            args.file_id,
            args.title,
            args.description,
            args.file_name,
            args.file_type,
            args.blob_storage_path,
            args.user_id,
            args.version_number,
            "",
            &mut tx,
        )
        .await?;
        let file = FileRepo::update_latest_version(
            args.file_id,
            args.latest_version,
            file_version.id,
            &mut tx,
        )
        .await?;
        tx.commit().await?;

        Ok((file, file_version).into())
    }

    pub async fn find_by_folder(folder: Uuid, pool: &PgPool) -> Result<Vec<FileWithVersion>> {
        let files = sqlx::query_file_as!(FileWithVersion, "sql/files/find_by_folder.sql", folder)
            .fetch_all(pool)
            .await?;

        Ok(files)
    }

    pub async fn find_by_id(id: Uuid, pool: &PgPool) -> Result<FileWithVersion> {
        let file = sqlx::query_file_as!(FileWithVersion, "sql/files/find_by_id.sql", id)
            .fetch_one(pool)
            .await?;

        Ok(file)
    }

    pub async fn delete(id: Uuid, deleted_by: Uuid, pool: &PgPool) -> Result<FileWithVersion> {
        let file = sqlx::query_file_as!(FileWithVersion, "sql/files/delete.sql", id, deleted_by)
            .fetch_one(pool)
            .await?;

        Ok(file)
    }
}

#[allow(dead_code)]
pub struct FileWithVersionRepoFake {}

#[allow(dead_code)]
// Fake implementation for tests. If you want integration tests that exercise the database,
// see https://doc.rust-lang.org/rust-by-example/testing/integration_testing.html.
impl FileWithVersionRepoFake {
    pub async fn create(args: CreateFileArgs<'_>, _pool: &PgPool) -> Result<FileWithVersion> {
        let file = FileWithVersion {
            id: Uuid::new_v4(),
            title: args.title.into(),
            description: args.description.into(),
            folder: args.folder_id,
            file_name: args.file_name.into(),
            file_type: args.file_type.into(),
            blob_storage_path: args.blob_storage_path.into(),
            version: Uuid::new_v4(),
            version_number: 1,
            created_at: Utc::now(),
            modified_at: Utc::now(),
            deleted_at: None,
        };
        Ok(file)
    }

    pub async fn create_version(
        args: CreateFileVersionArgs<'_>,
        _pool: &PgPool,
    ) -> Result<FileWithVersion> {
        let file = FileWithVersion {
            id: args.file_id,
            title: args.title.into(),
            description: args.description.into(),
            folder: args.folder_id,
            file_name: args.file_name.into(),
            file_type: args.file_type.into(),
            blob_storage_path: args.blob_storage_path.into(),
            version: Uuid::new_v4(),
            version_number: args.version_number,
            created_at: Utc::now(),
            modified_at: Utc::now(),
            deleted_at: None,
        };
        Ok(file)
    }

    pub async fn find_by_folder(_folder: Uuid, _pool: &PgPool) -> Result<Vec<FileWithVersion>> {
        Ok(vec![])
    }

    pub async fn find_by_id(id: Uuid, _pool: &PgPool) -> Result<FileWithVersion> {
        let file = FileWithVersion {
            id,
            title: "fake file".into(),
            description: "fake file for tests".into(),
            folder: Uuid::parse_str("805eaaca-6c7d-40ac-bede-99b83ef838e4")?,
            file_name: "fake.txt".into(),
            file_type: "text/plain".into(),
            blob_storage_path: "http://localhost:10000/devstoreaccount1/files/fake".into(),
            version: Uuid::parse_str("393ec819-92f2-4bb3-bb1d-189d2ef8a30a")?,
            version_number: 1,
            created_at: Utc::now(),
            modified_at: Utc::now(),
            deleted_at: None,
        };
        Ok(file)
    }

    pub async fn delete(id: Uuid, _deleted_by: Uuid, _pool: &PgPool) -> Result<FileWithVersion> {
        let file = FileWithVersion {
            id,
            title: "fake file".into(),
            description: "fake file for tests".into(),
            folder: Uuid::parse_str("805eaaca-6c7d-40ac-bede-99b83ef838e4")?,
            file_name: "fake.txt".into(),
            file_type: "text/plain".into(),
            blob_storage_path: "http://localhost:10000/devstoreaccount1/files/fake".into(),
            version: Uuid::parse_str("393ec819-92f2-4bb3-bb1d-189d2ef8a30a")?,
            version_number: 1,
            created_at: Utc::now(),
            modified_at: Utc::now(),
            deleted_at: Some(Utc::now()),
        };
        Ok(file)
    }
}
