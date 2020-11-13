// sqlx::query_file_as!() causes spurious errors with this lint enabled
#![allow(clippy::suspicious_else_formatting)]

use anyhow::Result;
use sqlx::{types::Uuid, PgPool};

#[derive(Clone)]
pub struct Folder {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub role_required: String,
    pub workspace: Uuid,
}

pub struct FolderRepo {}

impl FolderRepo {
    pub async fn create(
        title: &str,
        description: &str,
        role_required: &str,
        workspace: Uuid,
        pool: &PgPool,
    ) -> Result<Folder> {
        let folder = sqlx::query_file_as!(
            Folder,
            "sql/folders/create.sql",
            title,
            description,
            role_required,
            workspace,
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

    pub async fn update(
        id: Uuid,
        title: &str,
        description: &str,
        role_required: &str,
        pool: &PgPool,
    ) -> Result<Folder> {
        let folder = sqlx::query_file_as!(
            Folder,
            "sql/folders/update.sql",
            id,
            title,
            description,
            role_required,
        )
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

// #[cfg(test)]
// pub struct FolderRepoFake {}

// #[cfg(test)]
// impl FolderRepoFake {
//     pub async fn create(
//         title: &str,
//         description: &str,
//         role_required: &str,
//         workspace: Uuid,
//         _pool: &PgPool,
//     ) -> Result<Folder> {
//         let folder = Folder {
//             id: Uuid::new_v4(),
//             title: title.to_string(),
//             workspace,
//             description: description.to_string(),
//             role_required: role_required.to_string(),
//         };
//         Ok(folder)
//     }

//     pub async fn find_by_workspace(_workspace: Uuid, _pool: &PgPool) -> Result<Vec<Folder>> {
//         Ok(Vec::new())
//     }

//     pub async fn find_by_id(id: Uuid, _pool: &PgPool) -> Result<Folder> {
//         let folder = Folder {
//             id,
//             title: "fake folder".into(),
//             workspace: Uuid::new_v4(),
//             description: "fake folder for testing".into(),
//             role_required: "PLATFORM_MEMBER".into(),
//         };
//         Ok(folder)
//     }

//     pub async fn update(
//         id: Uuid,
//         title: &str,
//         description: &str,
//         role_required: &str,
//         _pool: &PgPool,
//     ) -> Result<Folder> {
//         let folder = Folder {
//             id,
//             title: title.to_string(),
//             workspace: Uuid::new_v4(),
//             description: description.to_string(),
//             role_required: role_required.to_string(),
//         };
//         Ok(folder)
//     }

//     pub async fn delete(id: Uuid, _pool: &PgPool) -> Result<Folder> {
//         let folder = Folder {
//             id,
//             title: "fake folder".into(),
//             workspace: Uuid::new_v4(),
//             description: "fake folder for testing".into(),
//             role_required: "PLATFORM_MEMBER".into(),
//         };
//         Ok(folder)
//     }
// }
