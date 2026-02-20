use std::{io, net::TcpListener};

use devbox::{configuration, startup::run};
use sqlx::PgPool;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    // Redirect all `log`'s events to our subscriber
    LogTracer::init().expect("Failed to set logger");
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("zero2prod".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    set_global_default(subscriber).expect("Failed to set subscriber");

    let configuration = configuration::get_config().expect("Error while generating config");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    let connection_pool =
        PgPool::connect_lazy(&configuration.database_settings.connection_string())
            .expect("Failed to connect to Postgres.");
    run(listener, connection_pool)?.await
}
