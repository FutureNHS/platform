// sqlx::query_file_as!() causes spurious errors with this lint enabled
#![allow(clippy::suspicious_else_formatting)]

use super::users::DbUser;
use crate::core::{
    team::{Team, TeamId, TeamRepo},
    user::{User, UserId},
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use sqlx::{types::Uuid, Postgres, Transaction};

#[derive(Clone)]
pub struct DbTeam {
    pub id: Uuid,
    pub title: String,
}

impl From<DbTeam> for Team {
    fn from(t: DbTeam) -> Self {
        Self {
            id: t.id.into(),
            title: t.title,
        }
    }
}

pub struct TeamRepoImpl<'a, 'ex> {
    pub(crate) executor: &'a mut Transaction<'ex, Postgres>,
}

#[async_trait]
impl<'a, 'ex> TeamRepo for TeamRepoImpl<'a, 'ex> {
    async fn create(&mut self, title: &str) -> Result<Team> {
        let team = sqlx::query_file_as!(DbTeam, "sql/teams/create.sql", title)
            .fetch_one(&mut *self.executor)
            .await
            .context("create team")?
            .into();

        Ok(team)
    }

    async fn members(&mut self, id: TeamId) -> Result<Vec<User>> {
        let id: Uuid = id.into();

        let users: Vec<DbUser> = sqlx::query_file_as!(DbUser, "sql/teams/members.sql", id)
            .fetch_all(&mut *self.executor)
            .await
            .context("get team members")?;

        Ok(users.iter().cloned().map(Into::into).collect())
    }

    async fn members_difference(
        &mut self,
        team_a_id: TeamId,
        team_b_id: TeamId,
    ) -> Result<Vec<User>> {
        let team_a_id: Uuid = team_a_id.into();
        let team_b_id: Uuid = team_b_id.into();
        let users: Vec<DbUser> = sqlx::query_file_as!(
            DbUser,
            "sql/teams/members_difference.sql",
            team_a_id,
            team_b_id
        )
        .fetch_all(&mut *self.executor)
        .await
        .context("get members of team A that aren't in team B")?;

        Ok(users.iter().cloned().map(Into::into).collect())
    }

    async fn is_member(&mut self, team_id: TeamId, user_id: UserId) -> Result<bool> {
        let team_id: Uuid = team_id.into();
        let user_id: Uuid = user_id.into();
        let found = sqlx::query_file!("sql/teams/is_member.sql", team_id, user_id)
            .fetch_optional(&mut *self.executor)
            .await
            .context("is user a member of team")?;

        Ok(found.is_some())
    }

    async fn add_member(&mut self, team_id: TeamId, user_id: UserId) -> Result<()> {
        let team_id: Uuid = team_id.into();
        let user_id: Uuid = user_id.into();
        sqlx::query_file!("sql/teams/add_member.sql", team_id, user_id)
            .execute(&mut *self.executor)
            .await
            .context("add member to team")?;

        Ok(())
    }

    async fn remove_member(&mut self, team_id: TeamId, user_id: UserId) -> Result<()> {
        let team_id: Uuid = team_id.into();
        let user_id: Uuid = user_id.into();
        sqlx::query_file!("sql/teams/remove_member.sql", team_id, user_id)
            .execute(&mut *self.executor)
            .await
            .context("remove member from team")?;

        Ok(())
    }
}
