// sqlx::query_file_as!() causes spurious errors with this lint enabled
#![allow(clippy::suspicious_else_formatting)]

use crate::db::User;
use anyhow::{Context, Result};
use sqlx::{types::Uuid, Executor, Postgres, Transaction};

#[derive(Clone)]
pub struct Team {
    pub id: Uuid,
    pub title: String,
}

// enum Either{
//     A(Transaction)
//     B(PgPool)
// }
// impl Executor for Either {}

#[cfg_attr(test, allow(dead_code))]
pub struct TeamRepo<'a, 'ex> {
    pub(crate) executor: &'a mut Transaction<'ex, Postgres>,
}

impl<'a, 'ex> TeamRepo<'a, 'ex> {
    pub async fn create(&mut self, title: &str) -> Result<Team> {
        let q = sqlx::query_file_as!(Team, "sql/teams/create.sql", title);
        let group = q
            .fetch_one(&mut *self.executor)
            .await
            .context("create team")?;

        Ok(group)
    }

    pub async fn members<'c, E>(id: Uuid, executor: E) -> Result<Vec<User>>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let users = sqlx::query_file_as!(User, "sql/teams/members.sql", id)
            .fetch_all(executor)
            .await
            .context("get team members")?;

        Ok(users)
    }

    pub async fn members_difference<'c, E>(
        team_a_id: Uuid,
        team_b_id: Uuid,
        executor: E,
    ) -> Result<Vec<User>>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let users = sqlx::query_file_as!(
            User,
            "sql/teams/members_difference.sql",
            team_a_id,
            team_b_id
        )
        .fetch_all(executor)
        .await
        .context("get members of team A that aren't in team B")?;

        Ok(users)
    }

    pub async fn is_member<'c, E>(team_id: Uuid, user_id: Uuid, executor: E) -> Result<bool>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let found = sqlx::query_file!("sql/teams/is_member.sql", team_id, user_id)
            .fetch_optional(executor)
            .await
            .context("is user a member of team")?;

        Ok(found.is_some())
    }

    pub async fn add_member<'c, E>(team_id: Uuid, user_id: Uuid, executor: E) -> Result<()>
    where
        E: Executor<'c, Database = Postgres>,
    {
        sqlx::query_file!("sql/teams/add_member.sql", team_id, user_id)
            .execute(executor)
            .await
            .context("add member to team")?;

        Ok(())
    }

    pub async fn remove_member<'c, E>(team_id: Uuid, user_id: Uuid, executor: E) -> Result<()>
    where
        E: Executor<'c, Database = Postgres>,
    {
        sqlx::query_file!("sql/teams/remove_member.sql", team_id, user_id)
            .execute(executor)
            .await
            .context("remove member from team")?;

        Ok(())
    }
}
