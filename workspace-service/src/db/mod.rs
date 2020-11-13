mod file_versions;
mod files;
mod folders;
mod teams;
mod users;
mod workspaces;

use crate::core::{team::TeamRepo, user::UserRepo, workspace::WorkspaceRepo, RepoCreator};
use anyhow::Result;
pub use file_versions::*;
pub use files::{
    CreateFileArgs, CreateFileVersionArgs, File, FileRepo, FileWithVersion, FileWithVersionRepo,
};
pub use folders::{Folder, FolderRepo};
use sqlx::{Executor, Postgres, Transaction};
pub use teams::TeamRepoImpl;
pub use users::{DbUser, UserRepoImpl};
pub use workspaces::{DbWorkspace, WorkspaceRepoImpl};

pub struct RepoFactory<'ex> {
    executor: Transaction<'ex, Postgres>,
}

impl<'ex> RepoFactory<'ex> {
    pub fn new(executor: Transaction<'ex, Postgres>) -> Self {
        Self { executor }
    }

    pub async fn commit(self) -> Result<()> {
        self.executor.commit().await?;
        Ok(())
    }
}

impl<'ex> RepoCreator<'ex> for RepoFactory<'ex> {
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
