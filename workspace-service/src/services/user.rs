use anyhow::Result;
use derive_more::{Display, From, Into};
use sqlx::{Executor, Postgres};
use uuid::Uuid;

#[derive(From, Into, Display, Copy, Clone)]
pub struct AuthId(Uuid);

#[derive(From, Into, Display, Copy, Clone)]
pub struct UserId(Uuid);

pub struct User {
    pub id: UserId,
    pub auth_id: AuthId,
    pub name: String,
    pub is_platform_admin: bool,
    pub email_address: String,
}

#[async_trait::async_trait]
pub trait UserRepo {
    async fn find_by_auth_id<'c, E>(auth_id: AuthId, executor: E) -> Result<Option<User>>
    where
        E: Executor<'c, Database = Postgres>;

    async fn find_by_id<'c, E>(id: UserId, executor: E) -> Result<Option<User>>
    where
        E: Executor<'c, Database = Postgres>;

    async fn get_or_create<'c, E>(
        auth_id: AuthId,
        name: &str,
        email_address: &str,
        executor: E,
    ) -> Result<User>
    where
        E: Executor<'c, Database = Postgres>;

    async fn update<'c, E>(auth_id: AuthId, is_platform_admin: bool, executor: E) -> Result<User>
    where
        E: Executor<'c, Database = Postgres>;
}
