use anyhow::Result;
use async_trait::async_trait;
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

#[async_trait]
pub trait UserRepo {
    async fn find_by_auth_id(&mut self, auth_id: AuthId) -> Result<Option<User>>;

    async fn find_by_id(&mut self, id: UserId) -> Result<Option<User>>;

    async fn get_or_create(
        &mut self,
        auth_id: AuthId,
        name: &str,
        email_address: &str,
    ) -> Result<User>;

    async fn update(&mut self, auth_id: AuthId, is_platform_admin: bool) -> Result<User>;
}
