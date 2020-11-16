use crate::core::{
    folder::{Folder, FolderId, FolderService},
    user::AuthId,
    workspace::WorkspaceId,
    RepoCreator,
};
use anyhow::Result;
use async_trait::async_trait;

#[derive(Clone)]
pub struct FolderServiceImpl {}

#[async_trait]
impl<'a, 'b> FolderService<'a, 'b> for FolderServiceImpl {
    async fn find_workspace_folders<T>(
        &self,
        repo_factory: &'a mut T,
        workspace_id: WorkspaceId,
    ) -> Result<Vec<Folder>>
    where
        T: RepoCreator<'b> + Send,
        'b: 'a,
    {
        let folders = repo_factory
            .folder()
            .find_by_workspace(workspace_id)
            .await?;
        let folders = folders.into_iter().map(Into::into).collect();

        Ok(folders)
    }
    async fn find_folder_by_id<T>(
        &self,
        repo_factory: &'a mut T,
        folder_id: FolderId,
    ) -> Result<Folder>
    where
        T: RepoCreator<'b> + Send,
        'b: 'a,
    {
        let folder = repo_factory.folder().find_by_id(folder_id).await?;

        Ok(folder)
    }

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
        T: RepoCreator<'b> + Send,
        'b: 'a,
    {
        let _user = repo_factory
            .user()
            .find_by_auth_id(requesting_user)
            .await?
            .ok_or_else(|| anyhow::anyhow!("user not found"))?;

        let folder = repo_factory
            .folder()
            .create(&title, &description, &role_required, workspace_id)
            .await?;

        // event_client
        //     .publish_events(&[Event::new(
        //         folder.id.clone(),
        //         FolderCreatedData {
        //             folder_id: folder.id.clone().into(),
        //             workspace_id: folder.workspace.clone().into(),
        //             user_id: user.id.to_string(),
        //             title: folder.title.clone(),
        //             description: folder.description.clone(),
        //             role_required: folder.role_required.to_string(),
        //         },
        //     )])
        //     .await?;
        Ok(folder)
    }

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
        T: RepoCreator<'b> + Send,
        'b: 'a,
    {
        let _user = repo_factory
            .user()
            .find_by_auth_id(requesting_user)
            .await?
            .ok_or_else(|| anyhow::anyhow!("user not found"))?;

        let folder = repo_factory
            .folder()
            .update(folder_id, title, description, role_required)
            .await?;

        Ok(folder)
    }

    async fn delete_folder<T>(
        &self,
        repo_factory: &'a mut T,
        folder_id: FolderId,
        requesting_user: AuthId,
    ) -> Result<Folder>
    where
        T: RepoCreator<'b> + Send,
        'b: 'a,
    {
        let _user = repo_factory
            .user()
            .find_by_auth_id(requesting_user)
            .await?
            .ok_or_else(|| anyhow::anyhow!("user not found"))?;

        let folder = repo_factory.folder().delete(folder_id).await?;

        // event_client
        //     .publish_events(&[Event::new(
        //         id,
        //         FolderDeletedData {
        //             folder_id: folder.id.to_string(),
        //             user_id: user.id.to_string(),
        //             workspace_id: folder.workspace.to_string(),
        //         },
        //     )])
        //     .await?;

        Ok(folder)
    }
}
