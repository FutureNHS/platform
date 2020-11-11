use super::{
    user::{User, UserId},
    DB,
};
use anyhow::Result;
use derive_more::{Display, From, Into};
use uuid::Uuid;

#[derive(From, Into, Display, Copy, Clone)]
pub struct TeamId(Uuid);

pub struct Team {
    pub id: TeamId,
    pub title: String,
}

#[async_trait::async_trait]
pub trait TeamRepo {
    async fn create<'c>(title: &str, executor: &DB<'c>) -> Result<Team>;

    async fn members<'c>(id: TeamId, executor: &DB<'c>) -> Result<Vec<User>>;

    async fn members_difference<'c>(
        team_a_id: TeamId,
        team_b_id: TeamId,
        executor: &DB<'c>,
    ) -> Result<Vec<User>>;

    async fn is_member<'c>(team_id: TeamId, user_id: UserId, executor: &DB<'c>) -> Result<bool>;

    async fn add_member<'c>(team_id: TeamId, user_id: UserId, executor: &DB<'c>) -> Result<()>;

    async fn remove_member<'c>(team_id: TeamId, user_id: UserId, executor: &DB<'c>) -> Result<()>;
}
