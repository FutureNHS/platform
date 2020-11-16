use super::{user::AuthId, workspace::WorkspaceId, RepoFactory};
use anyhow::Result;
use async_trait::async_trait;
use derive_more::{Display, From, Into};
use uuid::Uuid;

#[derive(From, Into, Default, Display, Copy, Clone, Debug, PartialEq)]
pub struct FolderId(Uuid);

#[derive(Default)]
pub struct Folder {
    pub id: FolderId,
    pub title: String,
    pub description: String,
    pub role_required: String,
    pub workspace: WorkspaceId,
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait FolderRepo {
    async fn create(
        &mut self,
        title: &str,
        description: &str,
        role_required: &str,
        workspace: WorkspaceId,
    ) -> Result<Folder>;

    async fn find_by_workspace(&mut self, workspace: WorkspaceId) -> Result<Vec<Folder>>;

    async fn find_by_id(&mut self, id: FolderId) -> Result<Folder>;

    async fn update(
        &mut self,
        id: FolderId,
        title: &str,
        description: &str,
        role_required: &str,
    ) -> Result<Folder>;

    async fn delete(&mut self, id: FolderId) -> Result<Folder>;
}

#[async_trait]
pub trait FolderService<'a, 'b> {
    async fn find_workspace_folders<T>(
        &self,
        repo_factory: &'a mut T,
        workspace_id: WorkspaceId,
    ) -> Result<Vec<Folder>>
    where
        T: RepoFactory<'b> + Send,
        'b: 'a;

    async fn find_folder_by_id<T>(
        &self,
        repo_factory: &'a mut T,
        folder_id: FolderId,
    ) -> Result<Folder>
    where
        T: RepoFactory<'b> + Send,
        'b: 'a;

    async fn create_folder<T>(
        &self,
        repo_factory: &'a mut T,
        title: &str,
        description: &str,
        role_required: &str,
        workspace_id: WorkspaceId,
        requesting_user: AuthId,
    ) -> Result<Folder>
    where
        T: RepoFactory<'b> + Send,
        'b: 'a;

    async fn update_folder<T>(
        &self,
        repo_factory: &'a mut T,
        folder_id: FolderId,
        title: &str,
        description: &str,
        role_required: &str,
        requesting_user: AuthId,
    ) -> Result<Folder>
    where
        T: RepoFactory<'b> + Send,
        'b: 'a;

    async fn delete_folder<T>(
        &self,
        repo_factory: &'a mut T,
        folder_id: FolderId,
        requesting_user: AuthId,
    ) -> Result<Folder>
    where
        T: RepoFactory<'b> + Send,
        'b: 'a;
}
