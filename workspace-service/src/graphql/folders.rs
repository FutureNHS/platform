use super::db;
use async_graphql::{Context, FieldResult, InputObject, Object, SimpleObject, ID};
use uuid::Uuid;

#[SimpleObject(desc = "A folder")]
pub struct Folder {
    #[field(desc = "The id of the folder")]
    id: ID,
    #[field(desc = "The title of the folder")]
    title: String,
    #[field(desc = "The description of the folder")]
    description: String,
    #[field(desc = "The workspace that this folder is in")]
    workspace: ID,
}

impl From<db::Folder> for Folder {
    fn from(d: db::Folder) -> Self {
        Self {
            id: d.id.into(),
            title: d.title,
            description: d.description,
            workspace: d.workspace.into(),
        }
    }
}

#[InputObject]
struct NewFolder {
    title: String,
    description: String,
    workspace: ID,
}

#[InputObject]
struct UpdateFolder {
    title: String,
    description: String,
}

#[derive(Default)]
pub struct FoldersQuery;

#[Object]
impl FoldersQuery {
    #[field(desc = "Get all Folders in a workspace")]
    async fn folders_by_workspace(
        &self,
        context: &Context<'_>,
        workspace: ID,
    ) -> FieldResult<Vec<Folder>> {
        let pool = context.data()?;
        let workspace = Uuid::parse_str(workspace.as_str())?;
        let folders = db::Folder::find_by_workspace(workspace, pool).await?;
        Ok(folders.into_iter().map(Into::into).collect())
    }

    #[field(desc = "Get folder by ID")]
    async fn folder(&self, context: &Context<'_>, id: ID) -> FieldResult<Folder> {
        self.get_folder(context, id).await
    }

    #[entity]
    async fn get_folder(&self, context: &Context<'_>, id: ID) -> FieldResult<Folder> {
        let pool = context.data()?;
        let id = Uuid::parse_str(id.as_str())?;
        let folder = db::Folder::find_by_id(id, pool).await?;
        Ok(folder.into())
    }
}

#[derive(Default)]
pub struct FoldersMutation;

#[Object]
impl FoldersMutation {
    #[field(desc = "Create a new folder (returns the created folder)")]
    async fn create_folder(&self, context: &Context<'_>, folder: NewFolder) -> FieldResult<Folder> {
        // TODO: Add event
        let pool = context.data()?;
        let workspace = Uuid::parse_str(folder.workspace.as_str())?;
        let folder = db::Folder::create(folder.title, folder.description, workspace, pool).await?;
        Ok(folder.into())
    }

    #[field(desc = "Update folder (returns updated folder")]
    async fn update_folder(
        &self,
        context: &Context<'_>,
        id: ID,
        folder: UpdateFolder,
    ) -> FieldResult<Folder> {
        // TODO: Add event
        let pool = context.data()?;
        let folder = db::Folder::update(
            Uuid::parse_str(id.as_str())?,
            folder.title,
            folder.description,
            pool,
        )
        .await?;

        Ok(folder.into())
    }

    #[field(desc = "Delete folder (returns deleted folder")]
    async fn delete_folder(&self, context: &Context<'_>, id: ID) -> FieldResult<Folder> {
        // TODO: Add event
        let pool = context.data()?;
        let folder = db::Folder::delete(Uuid::parse_str(id.as_str())?, pool).await?;

        Ok(folder.into())
    }
}
