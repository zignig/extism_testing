// Some logging setup
use tracing_subscriber::{
    filter::{LevelFilter, Targets},
    prelude::*,
};

pub fn start_logging() {
    let mut filter = Targets::new();
    filter = filter
        .with_target(env!("CARGO_PKG_NAME"), LevelFilter::DEBUG)
        .with_target("wash_runtime", LevelFilter::INFO);

    let fmt_layer = tracing_subscriber::fmt::layer().compact();//.without_time();

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .init();
}
