// sqlx::query_file_as!() causes spurious errors with this lint enabled
#![allow(clippy::suspicious_else_formatting)]

use crate::core::user::{AuthId, User, UserId, UserRepo};
use anyhow::Result;
use async_trait::async_trait;
use sqlx::{types::Uuid, Postgres, Transaction};

#[derive(Clone)]
pub struct DbUser {
    pub id: Uuid,
    pub auth_id: Uuid,
    pub name: String,
    pub is_platform_admin: bool,
    pub email_address: String,
}

impl From<DbUser> for User {
    fn from(u: DbUser) -> Self {
        Self {
            id: u.id.into(),
            auth_id: u.auth_id.into(),
            name: u.name,
            is_platform_admin: u.is_platform_admin,
            email_address: u.email_address,
        }
    }
}

pub struct UserRepoImpl<'a, 'ex> {
    pub(crate) executor: &'a mut Transaction<'ex, Postgres>,
}

#[async_trait]
impl<'a, 'ex> UserRepo for UserRepoImpl<'a, 'ex> {
    async fn find_by_auth_id(&mut self, auth_id: AuthId) -> Result<Option<User>> {
        let auth_id: Uuid = auth_id.into();
        let user = sqlx::query_file_as!(DbUser, "sql/users/find_by_auth_id.sql", auth_id)
            .fetch_optional(&mut *self.executor)
            .await?
            .map(Into::into);

        Ok(user)
    }

    async fn find_by_id(&mut self, id: UserId) -> Result<Option<User>> {
        let id: Uuid = id.into();
        let user = sqlx::query_file_as!(DbUser, "sql/users/find_by_id.sql", id)
            .fetch_optional(&mut *self.executor)
            .await?
            .map(Into::into);

        Ok(user)
    }

    async fn get_or_create(
        &mut self,
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
        .fetch_one(&mut *self.executor)
        .await?
        .into();

        Ok(user)
    }

    async fn update(&mut self, auth_id: AuthId, is_platform_admin: bool) -> Result<User> {
        let auth_id: Uuid = auth_id.into();
        let user = sqlx::query_file_as!(DbUser, "sql/users/update.sql", auth_id, is_platform_admin)
            .fetch_one(&mut *self.executor)
            .await?
            .into();

        Ok(user)
    }
}
