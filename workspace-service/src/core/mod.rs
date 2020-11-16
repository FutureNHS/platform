use self::{folder::FolderRepo, team::TeamRepo, user::UserRepo, workspace::WorkspaceRepo};

pub mod file;
pub mod folder;
pub mod team;
pub mod user;
pub mod workspace;

#[cfg_attr(test, mockall::automock)]
pub trait RepoCreator<'a> {
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
