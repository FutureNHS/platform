use self::{folder::FolderRepo, team::TeamRepo, user::UserRepo, workspace::WorkspaceRepo};

pub mod file;
pub mod folder;
pub mod team;
pub mod user;
pub mod workspace;

/// `RepoFactoryImpl` holds a `Transaction<'a>` and hands out short-lived [Whatever]Repos.
/// These `Repo`s hold an `&'r mut Transaction` to make DB queries with, and then let go of
/// it as soon as they go out of scope.
#[cfg_attr(test, mockall::automock)]
pub trait RepoFactory<'a> {
    fn folder<'r>(&'r mut self) -> Box<dyn FolderRepo + Send + 'r>
    where
        'a: 'r;

    fn team<'r>(&'r mut self) -> Box<dyn TeamRepo + Send + 'r>
    where
        'a: 'r;

    fn user<'r>(&'r mut self) -> Box<dyn UserRepo + Send + 'r>
    where
        'a: 'r;

    fn workspace<'r>(&'r mut self) -> Box<dyn WorkspaceRepo + Send + 'r>
    where
        'a: 'r;
}
