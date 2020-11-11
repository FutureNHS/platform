use super::{azure, db, RequestingUser};
use fnhs_event_models::{Event, EventClient};
use sqlx::PgPool;
use std::sync::{
    mpsc::{sync_channel, Receiver},
    Arc,
};
use url::Url;
use uuid::Uuid;

/// Should explode if you actually try to use it.
pub fn mock_connection_pool() -> anyhow::Result<PgPool> {
    let database_url = "postgresql://COMPLETELY_BOGUS_DB_URL";
    let connection_pool = PgPool::connect_lazy(&database_url)?;
    Ok(connection_pool)
}

/// Returns an EventClient and a channel to see what was published.
pub fn mock_event_emitter() -> (Receiver<Event>, EventClient) {
    let (sender, receiver) = sync_channel(1000);
    (receiver, EventClient::with_publisher(Arc::new(sender)))
}

pub async fn mock_admin_requesting_user() -> anyhow::Result<RequestingUser> {
    let pool = mock_connection_pool()?;
    let requesting_user = db::UserRepoImpl::get_or_create(
        &Uuid::parse_str("feedface-0000-0000-0000-000000000000").unwrap(),
        "admin",
        "email_address",
        &pool,
    )
    .await?;
    Ok(RequestingUser {
        auth_id: requesting_user.auth_id,
    })
}

pub async fn mock_unprivileged_requesting_user() -> anyhow::Result<RequestingUser> {
    let pool = mock_connection_pool()?;
    let requesting_user = db::UserRepoImpl::get_or_create(
        &Uuid::parse_str("deadbeef-0000-0000-0000-000000000000").unwrap(),
        "member",
        "email_address",
        &pool,
    )
    .await?;
    Ok(RequestingUser {
        auth_id: requesting_user.auth_id,
    })
}

pub fn mock_azure_config() -> anyhow::Result<azure::Config> {
    azure::Config::new(
        "Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw=="
            .into(),
        Url::parse("http://localhost:10000/devstoreaccount1/upload")?,
        Url::parse("http://localhost:10000/devstoreaccount1/files")?,
    )
}
