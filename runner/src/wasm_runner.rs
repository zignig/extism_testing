// Extism is not async, need to wrap in a task

use std::sync::Arc;

use anyhow::Result;
use extism::{Manifest, Plugin, Wasm};
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

    pub fn run_plugin(&self, input: &str) -> Result<String, extism::Error> {
        let mut plugin = Plugin::new(&self.manifest, [], true)?;
        let res = plugin.call::<&str, &str>("count_vowels", input)?;
        Ok(res.to_string())
    }

    // pub async fn sub_test(&mut self) -> Result<()> {
    //     let rx = self.rx.clone();
    //     tokio::task::spawn_blocking(move || {
    //         println!("Spawn blocking");
    //         while let Some(val) = rx.blocking_recv() {
    //             println!("{:#?}", val);
    //         }
    //     })
    //     .await?;
    //     // self.handle = Some(handle);
    //     Ok(())
    // }

    pub async fn test(&self,rx: mpsc::Receiver<&str>) {
        // let (tx, mut rx) = mpsc::channel(10);

        let  r = tokio::task::spawn_blocking(move || {
            while let Some(val) = rx.blocking_recv(){ 
                println!("{:#?}",val)
            }
        }).await.unwrap();

    }
}
