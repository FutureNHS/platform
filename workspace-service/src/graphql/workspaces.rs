use crate::db;
use async_graphql::{Context, FieldResult, InputObject, Object, SimpleObject, ID};
use uuid::Uuid;

#[SimpleObject(desc = "A workspace")]
pub struct Workspace {
    #[field(desc = "The id of the workspace")]
    id: ID,
    #[field(desc = "The title of the workspace")]
    title: String,
    #[field(desc = "The description of the workspace")]
    description: String,
}

impl From<db::Workspace> for Workspace {
    fn from(d: db::Workspace) -> Self {
        Self {
            id: d.id.into(),
            title: d.title,
            description: d.description,
        }
    }
}

#[InputObject]
struct NewWorkspace {
    title: String,
    description: String,
}

#[InputObject]
struct UpdateWorkspace {
    title: String,
    description: String,
}

#[derive(Default)]
pub struct WorkspacesQuery;

#[Object]
impl WorkspacesQuery {
    #[field(desc = "Get all Workspaces")]
    async fn workspaces(&self, context: &Context<'_>) -> FieldResult<Vec<Workspace>> {
        let pool = context.data()?;
        let workspaces = db::Workspace::find_all(pool).await?;
        Ok(workspaces.into_iter().map(Into::into).collect())
    }

    #[field(desc = "Get workspace by ID")]
    async fn workspace(&self, context: &Context<'_>, id: ID) -> FieldResult<Workspace> {
        self.get_workspace(context, id).await
    }

    #[entity]
    async fn get_workspace(&self, context: &Context<'_>, id: ID) -> FieldResult<Workspace> {
        let pool = context.data()?;
        let id = Uuid::parse_str(id.as_str())?;
        let workspace = db::Workspace::find_by_id(id, pool).await?;
        Ok(workspace.into())
    }
}

#[derive(Default)]
pub struct WorkspacesMutation;

#[Object]
impl WorkspacesMutation {
    #[field(desc = "Create a new workspace (returns the created workspace)")]
    async fn create_workspace(
        &self,
        context: &Context<'_>,
        workspace: NewWorkspace,
    ) -> FieldResult<Workspace> {
        // TODO: Add event
        let pool = context.data()?;
        let workspace = db::Workspace::create(workspace.title, workspace.description, pool).await?;
        Ok(workspace.into())
    }

    #[field(desc = "Update workspace (returns updated workspace")]
    async fn update_workspace(
        &self,
        context: &Context<'_>,
        id: ID,
        workspace: UpdateWorkspace,
    ) -> FieldResult<Workspace> {
        // TODO: Add event
        let pool = context.data()?;
        let workspace = db::Workspace::update(
            Uuid::parse_str(id.as_str())?,
            workspace.title,
            workspace.description,
            pool,
        )
        .await?;

        Ok(workspace.into())
    }

    #[field(desc = "Delete workspace(returns deleted workspace")]
    async fn delete_workspace(&self, context: &Context<'_>, id: ID) -> FieldResult<Workspace> {
        // TODO: Add event
        let pool = context.data()?;
        let workspace = db::Workspace::delete(Uuid::parse_str(id.as_str())?, pool).await?;

        Ok(workspace.into())
    }
}
