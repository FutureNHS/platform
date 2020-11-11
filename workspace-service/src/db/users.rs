// sqlx::query_file_as!() causes spurious errors with this lint enabled
#![allow(clippy::suspicious_else_formatting)]

use crate::services::user::{AuthId, User, UserId, UserRepo};
use anyhow::Result;
use sqlx::{types::Uuid, Executor, Postgres};

#[derive(Clone)]
pub struct DbUser {
    pub id: Uuid,
    pub auth_id: Uuid,
    pub name: String,
    pub is_platform_admin: bool,
    pub email_address: String,
}

impl From<DbUser> for User {
    fn from(_: DbUser) -> Self {
        todo!()
    }
}

pub struct UserRepoImpl {}

#[async_trait::async_trait]
impl UserRepo for UserRepoImpl {
    async fn find_by_auth_id<'c, E>(auth_id: AuthId, executor: E) -> Result<Option<User>>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let auth_id: Uuid = auth_id.into();
        let user = sqlx::query_file_as!(DbUser, "sql/users/find_by_auth_id.sql", auth_id)
            .fetch_optional(executor)
            .await?
            .into();

        Ok(user)
    }

    async fn find_by_id<'c, E>(id: UserId, executor: E) -> Result<Option<User>>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let id: Uuid = id.into();
        let user = sqlx::query_file_as!(DbUser, "sql/users/find_by_id.sql", id)
            .fetch_optional(executor)
            .await?
            .into();

        Ok(user)
    }

    async fn get_or_create<'c, E>(
        auth_id: AuthId,
        name: &str,
        email_address: &str,
        executor: E,
    ) -> Result<User>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let auth_id: Uuid = auth_id.into();
        let user = sqlx::query_file_as!(
            DbUser,
            "sql/users/get_or_create.sql",
            auth_id,
            name,
            email_address
        )
        .fetch_one(executor)
        .await?;

        Ok(user)
    }

    async fn update<'c, E>(auth_id: AuthId, is_platform_admin: bool, executor: E) -> Result<User>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let auth_id: Uuid = auth_id.into();
        let user = sqlx::query_file_as!(DbUser, "sql/users/update.sql", auth_id, is_platform_admin)
            .fetch_one(executor)
            .await?
            .into();

        Ok(user)
    }
}
