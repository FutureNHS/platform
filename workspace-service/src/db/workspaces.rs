// sqlx::query_file_as!() causes spurious errors with this lint enabled
#![allow(clippy::suspicious_else_formatting)]

use crate::core::{
    team::TeamId,
    workspace::{Workspace, WorkspaceId, WorkspaceRepo},
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use sqlx::{types::Uuid, Postgres, Transaction};

#[derive(Clone)]
pub struct DbWorkspace {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub admins: Uuid,
    pub members: Uuid,
}

impl From<DbWorkspace> for Workspace {
    fn from(w: DbWorkspace) -> Self {
        Self {
            id: w.id.into(),
            title: w.title,
            description: w.description,
            admins: w.admins.into(),
            members: w.members.into(),
        }
    }
}

pub struct WorkspaceRepoImpl<'a, 'ex> {
    pub(crate) executor: &'a mut Transaction<'ex, Postgres>,
}

#[async_trait]
impl<'a, 'ex> WorkspaceRepo for WorkspaceRepoImpl<'a, 'ex> {
    async fn create(
        &mut self,
        title: &str,
        description: &str,
        admins_team_id: TeamId,
        members_team_id: TeamId,
    ) -> Result<Workspace> {
        let admins_team_id: Uuid = admins_team_id.into();
        let members_team_id: Uuid = members_team_id.into();
        let workspace = sqlx::query_file_as!(
            DbWorkspace,
            "sql/workspaces/create.sql",
            title,
            description,
            admins_team_id,
            members_team_id
        )
        .fetch_one(&mut *self.executor)
        .await
        .context("create workspace")?
        .into();

        Ok(workspace)
    }

    async fn find_all(&mut self) -> Result<Vec<Workspace>> {
        let workspaces: Vec<DbWorkspace> =
            sqlx::query_file_as!(DbWorkspace, "sql/workspaces/find_all.sql")
                .fetch_all(&mut *self.executor)
                .await
                .context("find all workspaces")?;

        Ok(workspaces.iter().cloned().map(Into::into).collect())
    }

    async fn find_by_id(&mut self, id: WorkspaceId) -> Result<Workspace> {
        let id: Uuid = id.into();
        let workspace = sqlx::query_file_as!(DbWorkspace, "sql/workspaces/find_by_id.sql", id)
            .fetch_one(&mut *self.executor)
            .await
            .context("find a workspace by id")?
            .into();

        Ok(workspace)
    }

    async fn update(
        &mut self,
        id: WorkspaceId,
        title: &str,
        description: &str,
    ) -> Result<Workspace> {
        let id: Uuid = id.into();
        let workspace = sqlx::query_file_as!(
            DbWorkspace,
            "sql/workspaces/update.sql",
            id,
            title,
            description
        )
        .fetch_one(&mut *self.executor)
        .await
        .context("update workspace")?
        .into();

        Ok(workspace)
    }

    async fn delete(&mut self, id: WorkspaceId) -> Result<Workspace> {
        let id: Uuid = id.into();
        let workspace = sqlx::query_file_as!(DbWorkspace, "sql/workspaces/delete.sql", id)
            .fetch_one(&mut *self.executor)
            .await
            .context("delete workspace")?
            .into();

        Ok(workspace)
    }

    // pub async fn get_user_role(workspace_id: Uuid, user_id: Uuid, pool: &PgPool) -> Result<Role> {
    //     match db::UserRepo::find_by_id(&user_id, pool).await? {
    //         Some(user) => {
    //             let workspace = WorkspaceRepo::find_by_id(workspace_id, pool).await?;
    //             let is_member = db::TeamRepo::is_member(workspace.members, user.id, pool).await?;
    //             let is_admin = db::TeamRepo::is_member(workspace.admins, user.id, pool).await?;
    //             if is_admin {
    //                 Ok(Role::Admin)
    //             } else if is_member {
    //                 Ok(Role::NonAdmin)
    //             } else {
    //                 Ok(Role::NonMember)
    //             }
    //         }
    //         None => Ok(Role::NonMember),
    //     }
    // }
}

// Fake implementation for tests. If you want integration tests that exercise the database,
// see https://doc.rust-lang.org/rust-by-example/testing/integration_testing.html.
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

//     pub async fn get_user_role(workspace_id: Uuid, user_id: Uuid, pool: &PgPool) -> Result<Role> {
//         match db::UserRepo::find_by_id(&user_id, pool).await? {
//             Some(user) => {
//                 let workspace = WorkspaceRepoFake::find_by_id(workspace_id, pool).await?;
//                 let is_member = db::TeamRepo::is_member(workspace.members, user.id, pool).await?;
//                 let is_admin = db::TeamRepo::is_member(workspace.admins, user.id, pool).await?;
//                 if is_admin {
//                     Ok(Role::Admin)
//                 } else if is_member {
//                     Ok(Role::NonAdmin)
//                 } else {
//                     Ok(Role::NonMember)
//                 }
//             }
//             None => Ok(Role::NonMember),
//         }
//     }

//     pub async fn is_admin(workspace_id: Uuid, user_id: Uuid, pool: &PgPool) -> Result<bool> {
//         match db::UserRepo::find_by_id(&user_id, pool).await? {
//             Some(user) => {
//                 let workspace = WorkspaceRepoFake::find_by_id(workspace_id, pool).await?;
//                 db::TeamRepo::is_member(workspace.admins, user.id, pool).await
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
//                 db::TeamRepo::add_member(workspace.admins, user_id, pool).await?;
//                 db::TeamRepo::add_member(workspace.members, user_id, pool).await?;
//             }
//             Role::NonAdmin => {
//                 db::TeamRepo::remove_member(workspace.admins, user_id, pool).await?;
//                 db::TeamRepo::add_member(workspace.members, user_id, pool).await?;
//             }
//             Role::NonMember => {
//                 db::TeamRepo::remove_member(workspace.admins, user_id, pool).await?;
//                 db::TeamRepo::remove_member(workspace.members, user_id, pool).await?;
//             }
//         }

//         Ok(workspace)
//     }
// }
