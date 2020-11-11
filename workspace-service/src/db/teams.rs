// sqlx::query_file_as!() causes spurious errors with this lint enabled
#![allow(clippy::suspicious_else_formatting)]

use crate::services::{
    team::{Team, TeamId, TeamRepo},
    user::{User, UserId},
};
use anyhow::{Context, Result};
use sqlx::{types::Uuid, Executor, Postgres};

use super::DbUser;

#[derive(Clone)]
pub struct DbTeam {
    pub id: Uuid,
    pub title: String,
}

impl From<DbTeam> for Team {
    fn from(_: DbTeam) -> Self {
        todo!()
    }
}

pub struct TeamRepoImpl {}

#[async_trait::async_trait]
impl TeamRepo for TeamRepoImpl {
    async fn create<'c, E>(title: &str, executor: E) -> Result<Team>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let group = sqlx::query_file_as!(DbTeam, "sql/teams/create.sql", title)
            .fetch_one(executor)
            .await
            .context("create team")?
            .into();

        Ok(group)
    }

    async fn members<'c, E>(id: TeamId, executor: E) -> Result<Vec<User>>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let id: Uuid = id.into();

        let users: Vec<DbUser> = sqlx::query_file_as!(DbUser, "sql/teams/members.sql", id)
            .fetch_all(executor)
            .await
            .context("get team members")?;

        Ok(users.iter().cloned().map(Into::into).collect())
    }

    async fn members_difference<'c, E>(
        team_a_id: TeamId,
        team_b_id: TeamId,
        executor: E,
    ) -> Result<Vec<User>>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let team_a_id: Uuid = team_a_id.into();
        let team_b_id: Uuid = team_b_id.into();
        let users: Vec<DbUser> = sqlx::query_file_as!(
            DbUser,
            "sql/teams/members_difference.sql",
            team_a_id,
            team_b_id
        )
        .fetch_all(executor)
        .await
        .context("get members of team A that aren't in team B")?;

        Ok(users.iter().cloned().map(Into::into).collect())
    }

    async fn is_member<'c, E>(team_id: TeamId, user_id: UserId, executor: E) -> Result<bool>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let team_id: Uuid = team_id.into();
        let user_id: Uuid = user_id.into();
        let found = sqlx::query_file!("sql/teams/is_member.sql", team_id, user_id)
            .fetch_optional(executor)
            .await
            .context("is user a member of team")?;

        Ok(found.is_some())
    }

    async fn add_member<'c, E>(team_id: TeamId, user_id: UserId, executor: E) -> Result<()>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let team_id: Uuid = team_id.into();
        let user_id: Uuid = user_id.into();
        sqlx::query_file!("sql/teams/add_member.sql", team_id, user_id)
            .execute(executor)
            .await
            .context("add member to team")?;

        Ok(())
    }

    async fn remove_member<'c, E>(team_id: TeamId, user_id: UserId, executor: E) -> Result<()>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let team_id: Uuid = team_id.into();
        let user_id: Uuid = user_id.into();
        sqlx::query_file!("sql/teams/remove_member.sql", team_id, user_id)
            .execute(executor)
            .await
            .context("remove member from team")?;

        Ok(())
    }
}
