use super::{db::User, RequestingUser};
use async_graphql::{Context, FieldResult, InputObject, Object, ID};
use sqlx::PgPool;
use uuid::Uuid;

/// A user
#[Object]
impl User {
    /// The id of the user
    async fn id(&self) -> ID {
        self.id.into()
    }
    /// The auth id of the user
    async fn auth_id(&self) -> ID {
        self.auth_id.into()
    }
    /// The name of the user
    async fn name(&self) -> String {
        self.name.clone()
    }
    /// The email of the user
    async fn email_address(&self) -> String {
        self.email_address.clone()
    }
    /// If true, user has full platform access
    async fn is_platform_admin(&self) -> bool {
        self.is_platform_admin
    }
}

#[derive(InputObject)]
pub struct NewUser {
    pub auth_id: ID,
    pub name: String,
    pub email_address: String,
}

#[derive(InputObject)]
pub struct UpdateUser {
    pub auth_id: ID,
    pub is_platform_admin: bool,
}

#[derive(Default)]
pub struct UsersMutation;

#[Object]
impl UsersMutation {
    /// Get or Create a new user (returns the user)
    async fn get_or_create_user(
        &self,
        context: &Context<'_>,
        new_user: NewUser,
    ) -> FieldResult<User> {
        let pool: &PgPool = context.data()?;
        let auth_id = Uuid::parse_str(&new_user.auth_id)?;

        Ok(User::get_or_create(&auth_id, &new_user.name, &new_user.email_address, pool).await?)
    }

    /// Update a user (returns the user)
    async fn update_user(
        &self,
        context: &Context<'_>,
        update_user: UpdateUser,
    ) -> FieldResult<User> {
        let pool = context.data()?;

        let requesting_user = context.data::<RequestingUser>()?;
        update_user_impl(pool, requesting_user, update_user).await
    }
}

async fn update_user_impl(
    pool: &PgPool,
    requesting_user: &RequestingUser,
    update_user: UpdateUser,
) -> FieldResult<User> {
    let requesting_user = User::find_by_auth_id(&requesting_user.auth_id, pool).await?;
    if !requesting_user.is_platform_admin {
        return Err(anyhow::anyhow!(
            "User with auth_id {} is not a platform admin.",
            requesting_user.auth_id
        )
        .into());
    }

    let auth_id = Uuid::parse_str(&update_user.auth_id)?;
    Ok(User::update(&auth_id, update_user.is_platform_admin, pool).await?)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::graphql::test_mocks::*;

    #[async_std::test]
    async fn update_user_succeds_if_admin() -> anyhow::Result<()> {
        let pool = mock_connection_pool()?;
        update_user_impl(
            &pool,
            &mock_admin_requesting_user(),
            UpdateUser {
                auth_id: Uuid::new_v4().into(),
                is_platform_admin: true,
            },
        )
        .await
        .unwrap();

        Ok(())
    }

    #[async_std::test]
    async fn update_user_fails_if_not_admin() -> anyhow::Result<()> {
        let pool = mock_connection_pool()?;

        let user = mock_unprivileged_requesting_user();

        let result = update_user_impl(
            &pool,
            &user,
            UpdateUser {
                auth_id: user.auth_id.into(),
                is_platform_admin: true,
            },
        )
        .await;

        assert_eq!(
            result.err().unwrap().message,
            "User with auth_id deadbeef-0000-0000-0000-000000000000 is not a platform admin."
        );

        Ok(())
    }
}
