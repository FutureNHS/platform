// sqlx::query_file_as!() causes spurious errors with this lint enabled
#![allow(clippy::suspicious_else_formatting)]

use crate::services::{
    team::TeamId,
    user::UserId,
    workspace::{Role, Workspace, WorkspaceId, WorkspaceRepo},
    DB,
};
use anyhow::{Context, Result};
use sqlx::types::Uuid;

#[derive(Clone)]
pub struct DbWorkspace {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub admins: Uuid,
    pub members: Uuid,
}

impl From<DbWorkspace> for Workspace {
    fn from(_: DbWorkspace) -> Self {
        todo!()
    }
}

pub struct WorkspaceRepoImpl {}

#[async_trait::async_trait]
impl<'c, E> WorkspaceRepo<'c> for WorkspaceRepoImpl {
    async fn create(
        &self,
        title: &str,
        description: &str,
        admins_team_id: TeamId,
        members_team_id: TeamId,
        executor: DB<'c>,
    ) -> Result<Workspace> {
        let admins_team_id: Uuid = admins_team_id.into();
        let members_team_id: Uuid = members_team_id.into();
        let workspace = sqlx::query_file_as!(
            Workspace,
            "sql/workspaces/create.sql",
            title,
            description,
            admins_team_id,
            members_team_id
        )
        .fetch_one(&mut executor)
        .await
        .context("create workspace")?;

        Ok(workspace)
    }

    async fn find_all(&self, executor: DB<'c>) -> Result<Vec<Workspace>> {
        let workspaces = sqlx::query_file_as!(Workspace, "sql/workspaces/find_all.sql")
            .fetch_all(executor)
            .await
            .context("find all workspaces")?;

        Ok(workspaces)
    }

    async fn find_by_id(&self, id: WorkspaceId, executor: DB<'c>) -> Result<Workspace> {
        let workspace = sqlx::query_file_as!(Workspace, "sql/workspaces/find_by_id.sql", id)
            .fetch_one(executor)
            .await
            .context("find a workspace by id")?;

        Ok(workspace)
    }

    async fn update(
        &self,
        id: WorkspaceId,
        title: &str,
        description: &str,
        executor: DB<'c>,
    ) -> Result<Workspace> {
        let workspace = sqlx::query_file_as!(
            Workspace,
            "sql/workspaces/update.sql",
            id,
            title,
            description
        )
        .fetch_one(executor)
        .await
        .context("update workspace")?;

        Ok(workspace)
    }

    async fn delete(&self, id: WorkspaceId, executor: DB<'c>) -> Result<Workspace> {
        let workspace = sqlx::query_file_as!(Workspace, "sql/workspaces/delete.sql", id)
            .fetch_one(executor)
            .await
            .context("delete workspace")?;

        Ok(workspace)
    }
}

// // Fake implementation for tests. If you want integration tests that exercise the database,
// // see https://doc.rust-lang.org/rust-by-example/testing/integration_testing.html.
// #[cfg(test)]
// pub struct WorkspaceRepoFake {}

// #[cfg(test)]
// use std::collections::HashMap;
// #[cfg(test)]
// use std::sync::Mutex;

// #[cfg(test)]
// lazy_static::lazy_static! {
//     static ref WORKSPACES: Mutex<HashMap<Uuid, Workspace>> = Mutex::new(HashMap::new());
// }

// #[cfg(test)]
// impl WorkspaceRepoFake {
//     pub async fn create(title: &str, description: &str, _pool: &PgPool) -> Result<Workspace> {
//         let workspace = Workspace {
//             id: Uuid::new_v4(),
//             title: title.to_string(),
//             description: description.to_string(),
//             admins: Uuid::new_v4(),
//             members: Uuid::new_v4(),
//         };
//         let mut teams = WORKSPACES.lock().unwrap();
//         teams.insert(workspace.id, workspace.clone());
//         Ok(workspace)
//     }

//     pub async fn find_all(_pool: &PgPool) -> Result<Vec<Workspace>> {
//         Ok(vec![])
//     }

//     pub async fn find_by_id(id: Uuid, _pool: &PgPool) -> Result<Workspace> {
//         let teams = WORKSPACES.lock().unwrap();
//         Ok(teams.get(&id).unwrap().clone())
//     }

//     pub async fn update(
//         id: Uuid,
//         title: &str,
//         description: &str,
//         _pool: &PgPool,
//     ) -> Result<Workspace> {
//         let workspace = Workspace {
//             id,
//             title: title.to_string(),
//             description: description.to_string(),
//             admins: Uuid::new_v4(),
//             members: Uuid::new_v4(),
//         };
//         Ok(workspace)
//     }

//     pub async fn delete(id: Uuid, _pool: &PgPool) -> Result<Workspace> {
//         let workspace = Workspace {
//             id,
//             title: "fake deleted workspace".into(),
//             description: "fake deleted workspace for tests".into(),
//             admins: Uuid::new_v4(),
//             members: Uuid::new_v4(),
//         };
//         Ok(workspace)
//     }

//     pub async fn is_admin(workspace_id: Uuid, user_id: Uuid, pool: &PgPool) -> Result<bool> {
//         match user_repo.find_by_id(&user_id, pool).await? {
//             Some(user) => {
//                 let workspace = WorkspaceRepoFake::find_by_id(workspace_id, pool).await?;
//                 team_repo.is_member(workspace.admins, user.id, pool).await
//             }
//             None => Ok(false),
//         }
//     }

//     pub async fn change_workspace_membership(
//         workspace_id: Uuid,
//         user_id: Uuid,
//         new_role: Role,
//         pool: &PgPool,
//     ) -> Result<Workspace> {
//         let workspace = WorkspaceRepoFake::find_by_id(workspace_id, pool).await?;
//         match new_role {
//             Role::Admin => {
//                 team_repo.add_member(workspace.admins, user_id, pool).await?;
//                 team_repo.add_member(workspace.members, user_id, pool).await?;
//             }
//             Role::NonAdmin => {
//                 team_repo.remove_member(workspace.admins, user_id, pool).await?;
//                 team_repo.add_member(workspace.members, user_id, pool).await?;
//             }
//             Role::NonMember => {
//                 team_repo.remove_member(workspace.admins, user_id, pool).await?;
//                 team_repo.remove_member(workspace.members, user_id, pool).await?;
//             }
//         }

//         Ok(workspace)
//     }
// }
