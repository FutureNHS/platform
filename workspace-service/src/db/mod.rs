mod file_versions;
mod files;
mod folders;
mod teams;
mod users;
mod workspaces;

use crate::core::{
    folder::FolderRepo, team::TeamRepo, user::UserRepo, workspace::WorkspaceRepo, RepoFactory,
};
use anyhow::Result;
pub use file_versions::*;
pub use files::{
    CreateFileArgs, CreateFileVersionArgs, File, FileRepo, FileWithVersion, FileWithVersionRepo,
};
pub use folders::FolderRepoImpl;
use sqlx::{Executor, Postgres, Transaction};
pub use teams::TeamRepoImpl;
pub use users::UserRepoImpl;
pub use workspaces::{DbWorkspace, WorkspaceRepoImpl};

pub struct RepoFactoryImpl<'ex> {
    executor: Transaction<'ex, Postgres>,
}

impl<'ex> RepoFactoryImpl<'ex> {
    pub fn new(executor: Transaction<'ex, Postgres>) -> Self {
        Self { executor }
    }

    pub async fn commit(self) -> Result<()> {
        self.executor.commit().await?;
        Ok(())
    }
}

impl<'ex> RepoFactory<'ex> for RepoFactoryImpl<'ex> {
    fn folder<'r>(&'r mut self) -> Box<dyn FolderRepo + Send + 'r>
    where
        'ex: 'r,
    {
        Box::new(FolderRepoImpl {
            executor: &mut self.executor,
        })
    }

    fn team<'r>(&'r mut self) -> Box<dyn TeamRepo + Send + 'r>
    where
        'ex: 'r,
    {
        Box::new(TeamRepoImpl {
            executor: &mut self.executor,
        })
    }

    fn user<'r>(&'r mut self) -> Box<dyn UserRepo + Send + 'r>
    where
        'ex: 'r,
    {
        Box::new(UserRepoImpl {
            executor: &mut self.executor,
        })
    }

    fn workspace<'r>(&'r mut self) -> Box<dyn WorkspaceRepo + Send + 'r>
    where
        'ex: 'r,
    {
        Box::new(WorkspaceRepoImpl {
            executor: &mut self.executor,
        })
    }
}

async fn defer_all_constraints<'c, E>(executor: E) -> Result<()>
where
    E: Executor<'c, Database = Postgres>,
{
    sqlx::query!("SET CONSTRAINTS ALL DEFERRED;")
        .execute(executor)
        .await?;
    Ok(())
}
