use anyhow::{anyhow, Result};
use fnhs_event_models::EventClient;
use opentelemetry::{
    api::{trace::b3_propagator::B3Encoding, B3Propagator, Provider},
    global, sdk,
    sdk::BatchSpanProcessor,
};
use sqlx::PgPool;
use structopt::StructOpt;
use tracing_subscriber::{layer::SubscriberExt, Registry};
use workspace_service::{azure, config::Config};

#[async_std::main]
async fn main() -> Result<()> {
    let config = Config::from_args();

    if config.selfcheck_only {
        println!("--selfcheck-only is set. Exiting...");
        return Ok(());
    }

    if config.generate_schema_only {
        println!("{}", workspace_service::generate_graphql_schema().await?);
        return Ok(());
    }

    let provider = if let Some(instrumentation_key) = config.instrumentation_key {
        let exporter = opentelemetry_application_insights::Exporter::new(instrumentation_key);
        let batch_exporter = BatchSpanProcessor::builder(
            exporter,
            async_std::task::spawn,
            async_std::stream::interval,
        )
        .build();
        sdk::Provider::builder()
            .with_batch_exporter(batch_exporter)
            .build()
    } else {
        let exporter = opentelemetry::exporter::trace::stdout::Builder::default().init();
        opentelemetry::sdk::Provider::builder()
            .with_simple_exporter(exporter)
            .build()
    };

    let tracer = provider.get_tracer("workspace-service");
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = Registry::default().with(telemetry);
    tracing::subscriber::set_global_default(subscriber)?;

    let propagator = B3Propagator::with_encoding(B3Encoding::SingleAndMultiHeader);
    global::set_http_text_propagator(propagator);

    let connection_pool = PgPool::connect(config.database_url.expect("required").as_str()).await?;
    sqlx::migrate!("./migrations").run(&connection_pool).await?;

    let event_client = if let (Some(topic_endpoint), Some(topic_key)) =
        (config.eventgrid_topic_endpoint, config.eventgrid_topic_key)
    {
        EventClient::new(
            topic_endpoint
                .host_str()
                .ok_or_else(|| anyhow!("EVENTGRID_TOPIC_ENDPOINT does not contain host name"))?
                .to_owned(),
            topic_key,
        )
    } else {
        EventClient::default()
    };

    let app = workspace_service::create_app(
        connection_pool,
        event_client,
        azure::Config::new(
            config.file_storage_access_key.expect("required"),
            config.upload_container_url.expect("required"),
            config.files_container_url.expect("required"),
        )?,
    )
    .await?;
    app.listen("0.0.0.0:3030").await?;

    unreachable!()
}
