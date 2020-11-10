use anyhow::Result;
use async_trait::async_trait;
use derive_more::{Display, From, Into};
use uuid::Uuid;

pub struct Workspace {
    pub id: WorkspaceId,
    pub title: String,
    pub description: String,
    pub admins: TeamId,
    pub members: TeamId,
}

#[derive(From, Into, Display)]
pub struct WorkspaceId(Uuid);

#[derive(From, Into, Display)]
pub struct TeamId(Uuid);

#[async_trait]
pub trait WorkspaceService {
    async fn create(
        &self,
        title: &str,
        description: &str,
        requesting_user: AuthId,
    ) -> Result<Workspace>;
}

#[derive(Clone)]
pub struct WorkspaceServiceImpl {}

impl WorkspaceServiceImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl WorkspaceService for WorkspaceServiceImpl {
    async fn create(
        &self,
        title: &str,
        description: &str,
        requesting_user: AuthId,
    ) -> Result<Workspace> {
        let user = db::UserRepo::find_by_auth_id(&requesting_user, pool)
            .await?
            .ok_or_else(|| anyhow::anyhow!("user not found"))?;
        if !user.is_platform_admin {
            return Err(anyhow::anyhow!(
                "User with auth_id {} does not have permission to create a workspace.",
                requesting_user.auth_id,
            )
            .into());
        }

        let workspace: Workspace = WorkspaceRepo::create(title, description, pool)
            .await?
            .into();

        event_client
            .publish_events(&[Event::new(
                workspace.id.clone(),
                WorkspaceCreatedData {
                    workspace_id: workspace.id.clone().into(),
                    // TODO: Fill this in when we have users in the db.
                    user_id: "".into(),
                    title: workspace.title.clone(),
                },
            )])
            .await?;

        Ok(workspace)
    }
}
