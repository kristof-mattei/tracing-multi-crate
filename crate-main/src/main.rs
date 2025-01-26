use std::env;

use tracing::{error, info};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{filter::EnvFilter, util::SubscriberInitExt, Layer, Registry};

fn init_tracing() {
    let registry = Registry::default();

    let mut layers = vec![];

    // crate_a max level INFO
    let filter = EnvFilter::builder().parse("crate_a=INFO").unwrap();
    layers.push(tracing_subscriber::fmt::layer().with_filter(filter).boxed());

    // crate_b max level TRACE
    let filter = EnvFilter::builder().parse("crate_b=TRACE").unwrap();
    layers.push(tracing_subscriber::fmt::layer().with_filter(filter).boxed());

    // and for ourselves, ERROR
    let filter = EnvFilter::builder()
        .parse(format!(
            "{}=ERROR",
            env!("CARGO_PKG_NAME").replace('-', "_")
        ))
        .unwrap();

    layers.push(tracing_subscriber::fmt::layer().with_filter(filter).boxed());

    registry.with(layers).init();
}

fn main() {
    init_tracing();

    crate_a::work();
    crate_b::work();

    info!("I don't get printed");
    error!("I do!");
}
