mod file_versions;
mod files;
mod folders;
mod teams;
mod users;
mod workspaces;

pub use file_versions::*;
pub use files::{
    CreateFileArgs, CreateFileVersionArgs, File, FileRepo, FileWithVersion, FileWithVersionRepo,
};
pub use folders::{Folder, FolderRepo};
pub use teams::TeamRepoImpl;
pub use users::{DbUser, UserRepoImpl};
pub use workspaces::{DbWorkspace, WorkspaceRepoImpl};

use anyhow::Result;
use sqlx::{Executor, Postgres, Transaction};

pub struct RepoFactory<'ex> {
    pub(crate) executor: Transaction<'ex, Postgres>,
}

impl<'ex> RepoFactory<'ex> {
    pub fn team<'r>(&'r mut self) -> TeamRepoImpl<'r, 'ex>
    where
        'ex: 'r,
    {
        TeamRepoImpl {
            executor: &mut self.executor,
        }
    }
    pub fn user<'r>(&'r mut self) -> UserRepoImpl<'r, 'ex>
    where
        'ex: 'r,
    {
        UserRepoImpl {
            executor: &mut self.executor,
        }
    }
    pub fn workspace<'r>(&'r mut self) -> WorkspaceRepoImpl<'r, 'ex>
    where
        'ex: 'r,
    {
        WorkspaceRepoImpl {
            executor: &mut self.executor,
        }
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
