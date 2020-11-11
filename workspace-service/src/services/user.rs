use super::DB;
use anyhow::Result;
use derive_more::{Display, From, Into};
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
    async fn find_by_auth_id<'c>(auth_id: AuthId, executor: &DB<'c>) -> Result<Option<User>>;

    async fn find_by_id<'c>(id: UserId, executor: &DB<'c>) -> Result<Option<User>>;

    async fn get_or_create<'c>(
        auth_id: AuthId,
        name: &str,
        email_address: &str,
        executor: &DB<'c>,
    ) -> Result<User>;

    async fn update<'c>(
        auth_id: AuthId,
        is_platform_admin: bool,
        executor: &DB<'c>,
    ) -> Result<User>;
}
