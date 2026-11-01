use anyhow::Result;
use clap::Parser;
use std::collections::HashMap;
use std::{path::PathBuf, str::FromStr, sync::Arc};

mod engine;
mod storage;
mod webrouter;
mod workloads;

use webrouter::EasyRouter;

use wash_runtime::{
    engine::Engine,
    host::{HostApi, HostBuilder, http::Ingress},
    plugin::wasi_config::DynamicConfig,
    types::{Component, LocalResources, Workload, WorkloadStartRequest},
};

use tracing_subscriber::{
    filter::{LevelFilter, Targets},
    prelude::*,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to wasm
    input: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Some logging setup
    let mut filter = Targets::new();
    filter = filter
        .with_target(env!("CARGO_PKG_NAME"), LevelFilter::DEBUG)
        .with_target("wash_runtime", LevelFilter::INFO);

    let fmt_layer = tracing_subscriber::fmt::layer().compact().without_time();

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .init();

    let args = Args::parse();
    println!("{:#?}", &args);

    let blobs = storage::Storage::new(PathBuf::from_str("data")?).await?;

    // print out the exiting tags
    blobs.list_tag().await?;

    // Create the engine with pooling enabled
    let engine = Engine::builder().with_pooling_allocator(true).build()?;
    // Configure HTTP handler and plugins
    // let http_handler = Ingress::new(DevRouter::default(), "0.0.0.0:8080".parse()?).await?;
    let http_handler = Ingress::new(EasyRouter::default(), "0.0.0.0:8080".parse()?).await?;

    let config_plugin = DynamicConfig::default();

    // Build and start the host
    let host = HostBuilder::new()
        .with_engine(engine)
        .with_friendly_name("my-custom-host")
        .with_http_handler(Arc::new(http_handler))
        .with_plugin(Arc::new(config_plugin))?
        .build()?;
    let host = host.start().await?;
    println!("Host started: {}", host.friendly_name());

    let t = host.wit_world();
    // dangerous as it offers everything. ( tesing )
    let r = t.exports.clone().into_iter().collect();
    // println!("{:#?}", &r);

    // Start a workload
    if let Some(path) = args.input {
        let data = std::fs::read(&path)?;
        blobs.save(path).await?;
        let mut res = LocalResources::default();

        res.config = HashMap::from([("host".to_string(), "localhost:8080".to_string())]);

        let comp = Component {
            name: "bob".to_string(),
            bytes: data.into(),
            digest: None,
            local_resources: res,
            pool_size: 0,
            ..Default::default()
        };

        let req = WorkloadStartRequest {
            workload_id: uuid::Uuid::new_v4().to_string(),
            workload: Workload {
                namespace: "test".to_string(),
                name: "test-workload".to_string(),
                annotations: HashMap::new(),
                service: None,
                components: vec![comp],
                host_interfaces: r,
                volumes: vec![],
            },
        };
        let wr = host.workload_start(req).await?;
        println!("{:#?}", &wr);
    } else {
        println!("Load and run a default one.");
    }

    tokio::signal::ctrl_c().await?;
    Ok(())
}
