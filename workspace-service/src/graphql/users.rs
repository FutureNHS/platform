use super::RequestingUser;
use crate::{
    core::{user, RepoCreator},
    db::RepoFactory,
};
use async_graphql::{Context, FieldResult, InputObject, Object, SimpleObject, ID};
use sqlx::PgPool;
use uuid::Uuid;

/// A user
#[derive(SimpleObject)]
pub struct User {
    /// The id of the user
    pub id: ID,
    /// The auth id of the user
    pub auth_id: ID,
    /// The name of the user
    pub name: String,
    /// The email of the user
    pub email_address: String,
    /// If true, user has full platform access
    pub is_platform_admin: bool,
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

impl From<user::User> for User {
    fn from(user: user::User) -> Self {
        Self {
            id: user.id.into(),
            name: user.name,
            auth_id: user.auth_id.into(),
            email_address: user.email_address,
            is_platform_admin: user.is_platform_admin,
        }
    }
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
        let auth_id = Uuid::parse_str(&new_user.auth_id)?.into();

        let mut repos = RepoFactory::new(pool.begin().await?);
        let user = repos
            .user()
            .get_or_create(auth_id, &new_user.name, &new_user.email_address)
            .await?
            .into();
        Ok(user)
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
    let mut repos = RepoFactory::new(pool.begin().await?);
    let mut user_repo = repos.user();
    let requesting_user = user_repo
        .find_by_auth_id(requesting_user.auth_id.into())
        .await?
        .ok_or_else(|| anyhow::anyhow!("user not found"))?;
    if !requesting_user.is_platform_admin {
        return Err(anyhow::anyhow!(
            "User with auth_id {} is not a platform admin.",
            requesting_user.auth_id
        )
        .into());
    }

    let auth_id = Uuid::parse_str(&update_user.auth_id)?.into();
    let user = user_repo
        .update(auth_id, update_user.is_platform_admin)
        .await?
        .into();
    Ok(user)
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::graphql::test_mocks::*;

//     #[async_std::test]
//     async fn update_user_succeeds_if_admin() -> anyhow::Result<()> {
//         let pool = mock_connection_pool()?;
//         let requesting_user = mock_admin_requesting_user().await?;

//         update_user_impl(
//             &pool,
//             &requesting_user,
//             UpdateUser {
//                 auth_id: requesting_user.auth_id.into(),
//                 is_platform_admin: true,
//             },
//         )
//         .await
//         .unwrap();

//         Ok(())
//     }

//     #[async_std::test]
//     async fn update_user_fails_if_not_admin() -> anyhow::Result<()> {
//         let pool = mock_connection_pool()?;

//         let user = mock_unprivileged_requesting_user().await?;

//         let result = update_user_impl(
//             &pool,
//             &user,
//             UpdateUser {
//                 auth_id: user.auth_id.into(),
//                 is_platform_admin: true,
//             },
//         )
//         .await;

//         assert_eq!(
//             result.err().unwrap().message,
//             "User with auth_id deadbeef-0000-0000-0000-000000000000 is not a platform admin."
//         );

//         Ok(())
//     }
// }
