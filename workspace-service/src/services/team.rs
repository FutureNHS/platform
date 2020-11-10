use super::user::{User, UserId};
use anyhow::Result;
use derive_more::{Display, From, Into};
use sqlx::{Executor, Postgres};
use uuid::Uuid;

#[derive(From, Into, Display, Copy, Clone)]
pub struct TeamId(Uuid);

pub struct Team {
    pub id: TeamId,
    pub title: String,
}

#[async_trait::async_trait]
pub trait TeamRepo {
    async fn create<'c, E>(&self, title: &str, executor: E) -> Result<Team>
    where
        E: Executor<'c, Database = Postgres>;

    async fn members<'c, E>(&self, id: TeamId, executor: E) -> Result<Vec<User>>
    where
        E: Executor<'c, Database = Postgres>;

    async fn members_difference<'c, E>(
        &self,
        team_a_id: TeamId,
        team_b_id: TeamId,
        executor: E,
    ) -> Result<Vec<User>>
    where
        E: Executor<'c, Database = Postgres>;

    async fn is_member<'c, E>(&self, team_id: TeamId, user_id: UserId, executor: E) -> Result<bool>
    where
        E: Executor<'c, Database = Postgres>;

    async fn add_member<'c, E>(&self, team_id: TeamId, user_id: UserId, executor: E) -> Result<()>
    where
        E: Executor<'c, Database = Postgres>;

    async fn remove_member<'c, E>(
        &self,
        team_id: TeamId,
        user_id: UserId,
        executor: E,
    ) -> Result<()>
    where
        E: Executor<'c, Database = Postgres>;
}
