use std::{sync::Arc};

use anyhow::Result;
use clap::Parser;

mod wasm_runner;
use wasm_runner::PluginRunner;

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

    let runner = Arc::new(PluginRunner::new("dist/plugin.wasm"));
    let res = runner.run_plugin(input);
    println!("{:#?}", &res);
    Ok(())
}
