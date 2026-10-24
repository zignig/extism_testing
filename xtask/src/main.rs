/// Task builder
/// make the plugin 

use std::path::PathBuf;
use std::process::Command;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    action: Option<String>
}

fn main() {
    let args = Args::parse();
    println!("{:#?}",args);

    let status = Command::new("cargo")
        .args([
            "build",
            "--package",
            "plugin",
            "--target",
            "wasm32-unknown-unknown",
            "--release",
        ])
        .status()
        .expect("Failed to run cargo build");

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }

    // From "target\wasm32-unknown-unknown\release\plugin.wasm"
    // 2. Perform your post-build tasks here
    println!("Doing post-build work (e.g., copying binaries)...");

    let sections = vec!["target","wasm32-unknown-unknown","release","plugin.wasm"];
    let source: PathBuf = sections.iter().collect();

    // make the dir
    let _ = std::fs::create_dir("dist");
    let dsections =  vec!["dist","plugin.wasm"];
    let dest: PathBuf = dsections.iter().collect();

    println!("{:?} => {:?}",source,dest);
    
    std::fs::copy(
        source,
        dest
    )
    .unwrap();
}
