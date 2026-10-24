use std::{path::PathBuf, str::FromStr, sync::Arc};

use anyhow::Result;
use clap::Parser;

mod wasm_runner;
use wasm_runner::PluginRunner;

mod storage;
use storage::Storage;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet (Positional argument)
    input: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let input = if let Some(val) = &args.input {
        val.as_str()
    } else {
        "aeiou"
    };

    let storage = Storage::new(PathBuf::from_str("data")?).await?;
    // println!("{:#?}",&storage);
    let _ = storage.list_tags().await;
    // storage.install("./dist/plugin.wasm").await?;


    let runner = Arc::new(PluginRunner::new("dist/plugin.wasm"));
    let res = runner.run_plugin(input);
    println!("{:#?}", &res);


    tokio::signal::ctrl_c().await?;
    Ok(())
}

