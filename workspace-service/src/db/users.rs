// sqlx::query_file_as!() causes spurious errors with this lint enabled
#![allow(clippy::suspicious_else_formatting)]

use crate::services::user::{AuthId, User, UserId, UserRepo};
use anyhow::Result;
use sqlx::{types::Uuid, PgPool};
use std::sync::Arc;

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

#[cfg_attr(test, allow(dead_code))]
pub struct UserRepoImpl {
    pool: Arc<PgPool>,
}

#[cfg_attr(test, allow(dead_code))]
#[async_trait::async_trait]
impl UserRepo for UserRepoImpl {
    async fn find_by_auth_id(&self, auth_id: AuthId) -> Result<Option<User>> {
        let auth_id: Uuid = auth_id.into();
        let user = sqlx::query_file_as!(DbUser, "sql/users/find_by_auth_id.sql", auth_id)
            .fetch_optional(&*self.pool)
            .await?
            .into();

        Ok(user)
    }

    async fn find_by_id(&self, id: UserId) -> Result<Option<User>> {
        let id: Uuid = id.into();
        let user = sqlx::query_file_as!(DbUser, "sql/users/find_by_id.sql", id)
            .fetch_optional(&*self.pool)
            .await?
            .into();

        Ok(user)
    }

    async fn get_or_create(
        &self,
        auth_id: AuthId,
        name: &str,
        email_address: &str,
    ) -> Result<User> {
        let auth_id: Uuid = auth_id.into();
        let user = sqlx::query_file_as!(
            DbUser,
            "sql/users/get_or_create.sql",
            auth_id,
            name,
            email_address
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(user)
    }

    async fn update(&self, auth_id: AuthId, is_platform_admin: bool) -> Result<User> {
        let auth_id: Uuid = auth_id.into();
        let user = sqlx::query_file_as!(DbUser, "sql/users/update.sql", auth_id, is_platform_admin)
            .fetch_one(&*self.pool)
            .await?
            .into();

        Ok(user)
    }
}

#[cfg(test)]
pub struct UserRepoFake {}

#[cfg(test)]
use std::sync::Mutex;
#[cfg(test)]
use std::{collections::HashMap, sync::Arc};

#[cfg(test)]
lazy_static::lazy_static! {
    static ref USERS_BY_ID: Mutex<HashMap<Uuid, User>> = Mutex::new(HashMap::new());
    static ref USERS_BY_AUTH_ID: Mutex<HashMap<Uuid, User>> = Mutex::new(HashMap::new());
}

// Fake implementation for tests. If you want integration tests that exercise the database,
// see https://doc.rust-lang.org/rust-by-example/testing/integration_testing.html.
#[cfg(test)]
impl UserRepoFake {
    pub async fn find_by_auth_id(auth_id: &Uuid, _pool: impl Sized) -> Result<Option<User>> {
        let users = USERS_BY_AUTH_ID.lock().unwrap();
        Ok(users.get(auth_id).cloned())
    }

    pub async fn find_by_id(id: &Uuid, _pool: &PgPool) -> Result<Option<User>> {
        let users = USERS_BY_ID.lock().unwrap();
        Ok(users.get(id).cloned())
    }

    pub async fn get_or_create(
        auth_id: &Uuid,
        name: &str,
        email_address: &str,
        pool: impl Sized,
    ) -> Result<User> {
        const ADMIN_AUTH_ID: &str = "feedface-0000-0000-0000-000000000000";
        let user = if let Ok(Some(user)) = UserRepoFake::find_by_auth_id(auth_id, pool).await {
            user
        } else {
            let user = User {
                id: Uuid::new_v4(),
                auth_id: *auth_id,
                name: name.to_string(),
                is_platform_admin: auth_id.to_string() == ADMIN_AUTH_ID,
                email_address: email_address.to_string(),
            };
            let mut users = USERS_BY_ID.lock().unwrap();
            users.insert(user.id, user.clone());
            let mut users = USERS_BY_AUTH_ID.lock().unwrap();
            users.insert(user.auth_id, user.clone());
            user
        };

        Ok(user)
    }

    pub async fn update(
        auth_id: &Uuid,
        is_platform_admin: bool,
        _pool: impl Sized,
    ) -> Result<User> {
        let mut users = USERS_BY_AUTH_ID.lock().unwrap();
        let user = users.get_mut(auth_id).unwrap();
        user.is_platform_admin = is_platform_admin;
        Ok(user.clone())
    }
}
