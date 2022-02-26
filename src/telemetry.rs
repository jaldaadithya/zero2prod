use std::io::stdout;

use tracing::{Subscriber, subscriber::set_global_default};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry,prelude::__tracing_subscriber_SubscriberExt, fmt::MakeWriter};

pub fn get_subscriber<Sink>(name:String, env_filter_level:String, sink : Sink) -> impl Subscriber+Sync+Send
where Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static{
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter_level));
    let formatting_layer = BunyanFormattingLayer::new(name,sink);
    let subscriber = Registry::default().with(env_filter)
    .with(JsonStorageLayer)
    .with(formatting_layer);
    subscriber
}

pub fn init_subscriber(subscriber: impl Subscriber+Sync+Send) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}