use super::{azure, db};
use async_graphql::{Context, FieldResult, Object, ID};
use fnhs_event_models::{Event, EventClient, EventPublisher as _, FileDownloadedData};
use sqlx::PgPool;
use url::Url;
use uuid::Uuid;

#[derive(Default)]
pub struct FileDownloadUrlsMutation;

#[Object]
impl FileDownloadUrlsMutation {
    /// Get a file download URL
    async fn file_download_url(&self, context: &Context<'_>, id: ID) -> FieldResult<Url> {
        let pool = context.data()?;
        let config = context.data()?;
        let event_client = context.data()?;
        let requesting_user = context.data()?;

        file_download_url(id, pool, config, event_client, requesting_user).await
    }
}

async fn file_download_url(
    id: ID,
    pool: &PgPool,
    config: &azure::Config,
    event_client: &EventClient,
    requesting_user: &super::RequestingUser,
) -> FieldResult<Url> {
    let user = db::UserRepoImpl::find_by_auth_id(&requesting_user.auth_id, pool)
        .await?
        .ok_or_else(|| anyhow::anyhow!("user not found"))?;
    let id = Uuid::parse_str(&id)?;
    let file = db::FileWithVersionRepo::find_by_id(id, pool).await?;
    let folder = db::FolderRepo::find_by_id(file.folder, pool).await?;

    event_client
        .publish_events(&[Event::new(
            id.to_string(),
            FileDownloadedData {
                file_id: file.id.to_string(),
                user_id: user.id.to_string(),
                version_number: file.version_number.into(),
                version_id: file.version.to_string(),
                workspace_id: folder.workspace.to_string(),
            },
        )])
        .await?;

    Ok(azure::create_download_sas(
        config,
        &file.blob_storage_path.parse()?,
    )?)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::graphql::test_mocks::*;
    use fnhs_event_models::EventData;
    #[async_std::test]
    async fn file_download_url_emits_event() -> anyhow::Result<()> {
        let pool = mock_connection_pool()?;
        let azure_config = mock_azure_config()?;

        let requesting_user = mock_unprivileged_requesting_user().await?;

        let (events, event_client) = mock_event_emitter();

        file_download_url(
            "1f20fa4c-543c-45b4-93bb-a6e21a8e4de5".into(),
            &pool,
            &azure_config,
            &event_client,
            &requesting_user,
        )
        .await
        .unwrap();

        assert!(events
            .try_iter()
            .any(|e| matches!(e.data, EventData::FileDownloaded(_))));

        Ok(())
    }
}
