use tracing::subscriber::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{
    prelude::__tracing_subscriber_SubscriberExt, registry::Registry, EnvFilter,
};

pub fn configure_tracing() -> impl Subscriber + Send + Sync {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("oves".into(), std::io::stdout);
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}
