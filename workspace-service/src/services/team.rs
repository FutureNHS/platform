use super::user::{User, UserId};
use anyhow::Result;
use async_trait::async_trait;
use derive_more::{Display, From, Into};
use uuid::Uuid;

#[derive(From, Into, Display, Copy, Clone, Debug, PartialEq)]
pub struct TeamId(Uuid);

pub struct Team {
    pub id: TeamId,
    pub title: String,
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait TeamRepo {
    async fn create(&mut self, title: &str) -> Result<Team>;

    async fn members(&mut self, id: TeamId) -> Result<Vec<User>>;

    async fn members_difference(
        &mut self,
        team_a_id: TeamId,
        team_b_id: TeamId,
    ) -> Result<Vec<User>>;

    async fn is_member(&mut self, team_id: TeamId, user_id: UserId) -> Result<bool>;

    async fn add_member(&mut self, team_id: TeamId, user_id: UserId) -> Result<()>;

    async fn remove_member(&mut self, team_id: TeamId, user_id: UserId) -> Result<()>;
}
