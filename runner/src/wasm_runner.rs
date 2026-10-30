// Extism is not async, need to wrap in a task

use std::sync::Arc;

use anyhow::Result;
use tokio::{sync::mpsc, task::JoinHandle};

pub struct PluginRunner<'a> {
    manifest: Manifest,
    // handle: Option<JoinHandle<()>>,

}

impl<'a> PluginRunner<'a> {
    pub fn new(path: &str) -> Self {
        // Convert to bytes
        let wasm = Wasm::file(path);
        let manifest = Manifest::new([wasm]);

        Self {
            manifest,
            // handle: None,
            // rx,
        }
    }

    pub async fn test(&self,rx: mpsc::Receiver<&str>) {
        // let (tx, mut rx) = mpsc::channel(10);

        let  r = tokio::task::spawn_blocking(move || {
            while let Some(val) = rx.blocking_recv(){ 
                println!("{:#?}",val)
            }
        }).await.unwrap();

    }
}
