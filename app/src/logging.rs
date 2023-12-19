use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{
    prelude::__tracing_subscriber_SubscriberExt, registry::Registry, EnvFilter,
};

pub fn configure_tracing() -> impl Subscriber + Send + Sync {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("oves".into(), std::io::stdout);
    return Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
}
