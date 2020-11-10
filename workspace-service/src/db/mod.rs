mod file_versions;
mod files;
mod folders;
mod teams;
mod users;
mod workspaces;

pub use file_versions::*;
pub use workspaces::Role;

#[cfg(not(test))]
pub use files::FileWithVersionRepo;
#[cfg(test)]
pub use files::FileWithVersionRepoFake as FileWithVersionRepo;
pub use files::{CreateFileArgs, CreateFileVersionArgs, File, FileRepo, FileWithVersion};

pub use folders::Folder;
#[cfg(not(test))]
pub use folders::FolderRepo;
#[cfg(test)]
pub use folders::FolderRepoFake as FolderRepo;

pub use teams::Team;
#[cfg(not(test))]
pub use teams::TeamRepo;
#[cfg(test)]
pub use teams::TeamRepoFake as TeamRepo;

pub use workspaces::Workspace;
#[cfg(not(test))]
pub use workspaces::WorkspaceRepo;
#[cfg(test)]
pub use workspaces::WorkspaceRepoFake as WorkspaceRepo;

use anyhow::Result;
use sqlx::{Executor, Postgres};

async fn defer_all_constraints<'c, E>(executor: E) -> Result<()>
where
    E: Executor<'c, Database = Postgres>,
{
    sqlx::query!("SET CONSTRAINTS ALL DEFERRED;")
        .execute(executor)
        .await?;
    Ok(())
}
