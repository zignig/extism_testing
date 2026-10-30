use std::{path::{Path, PathBuf}, sync::Arc};

use anyhow::Result;
use clap::Parser;

mod storage;
// use storage::Storage;

mod engine;

use std::collections::HashMap;

use wash_runtime::{
    engine::Engine, host::{HostApi, HostBuilder}, plugin::{wasi_config::RuntimeConfig, wasi_http::HttpServer}, types::{Component, LocalResources, Workload, WorkloadStartRequest},
};

use tracing_subscriber::filter::LevelFilter;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to wasm
    input: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO) // Explicitly set maximum verbosity
        .init();

    let args = Args::parse();
    println!("{:#?}", &args);

    // Create a Wasmtime engine
    let engine = Engine::builder().build()?;

    // Configure plugins
    let http_plugin = HttpServer::new("0.0.0.0:8080".parse()?);
    let runtime_config_plugin = RuntimeConfig::default();

    // Build and start the host
    let host = HostBuilder::new()
        .with_engine(engine)
        .with_plugin(Arc::new(http_plugin))?
        .with_plugin(Arc::new(runtime_config_plugin))?
        .build()?;

    let host = host.start().await?;

    // Start a workload
    if let Some(path) = args.input {
        let data = std::fs::read(path)?;
        let comp = Component{
            bytes: data.into(),
            local_resources: LocalResources::default(),
            pool_size: 0,
            max_invocations: 0
        };
        let req = WorkloadStartRequest {
            workload: Workload {
                namespace: "test".to_string(),
                name: "test-workload".to_string(),
                annotations: HashMap::new(),
                service: None,
                components: vec![comp],
                host_interfaces: vec![],
                volumes: vec![],
            },
        };
        host.workload_start(req).await?;
    }

    
    tokio::signal::ctrl_c().await?;
    Ok(())
}
