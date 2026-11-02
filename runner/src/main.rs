//! A wasm cloud attached to an Iroh node.

use anyhow::Result;
use clap::Parser;
// use wash_runtime::types::Service;
use std::collections::HashMap;
use std::{path::PathBuf, str::FromStr, sync::Arc};

mod engine;
mod storage;
mod trace;
mod webrouter;
mod workloads;

use webrouter::EasyRouter;

// The wash stuff
use wash_runtime::{
    engine::Engine,
    host::{HostApi, HostBuilder, http::Ingress},
    plugin::wasi_config::DynamicConfig,
    types::{Component, LocalResources, Workload, WorkloadStartRequest},
};

// Local tracing
use tracing::info;

use crate::workloads::Dispatcher;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to wasm
    input: Option<PathBuf>,
    is_service: Option<bool>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Setup and start some logging.
    trace::start_logging();

    // Get the args and print them
    let args = Args::parse();
    println!("{:#?}", &args);

    let blobs = storage::Storage::new(PathBuf::from_str("data")?).await?;

    // Create the engine with pooling enabled
    let engine = Engine::builder().with_pooling_allocator(true).build()?;
    let http_handler = Ingress::new(EasyRouter::default(), "0.0.0.0:8080".parse()?).await?;
    let config_plugin = DynamicConfig::default();

    // Build and start the host
    let host = HostBuilder::new()
        .with_engine(engine)
        .with_friendly_name("my-custom-host")
        .with_http_handler(Arc::new(http_handler))
        .with_plugin(Arc::new(config_plugin))?
        .build()?;

    // Go
    let host = host.start().await?;
    println!("Host started: {}", host.friendly_name());

    // Load the default items
    let item_list = blobs.list_tags("preboot:").await?;
    for name in item_list.iter() {
        info!("load me {}", &name);
        // if let Some(item) = blobs.get_item(name.clone()).await? {
        //     info!("have the blob {}",name);
        // }
    }

    let ww = host.wit_world();
    // dangerous as it offers everything. ( tesing )
    let r = ww.exports.clone().into_iter().collect();

    let dispatcher = Dispatcher::new(ww);

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
