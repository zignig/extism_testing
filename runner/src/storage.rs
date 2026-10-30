use std::{env, path::PathBuf};

use anyhow::Result;
use iroh_blobs::store::fs::FsStore;
use n0_future::StreamExt;
use wash_runtime::plugin::HostPlugin;

#[derive(Debug)]
pub struct Storage {
    blobs: FsStore,
}

impl Storage {
    pub async fn new(path: PathBuf) -> Result<Self> {
        let mut full_path = path.clone();
        full_path.push("blobs");

        let blobs = FsStore::load(full_path).await?;
        Ok(Self { blobs })
    }
}


