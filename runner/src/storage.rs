use std::path::PathBuf;

use anyhow::Result;
use iroh_blobs::store::fs::FsStore;
use n0_future::StreamExt;

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

    pub async fn save(&self, path: PathBuf) -> Result<()> {
        let mut tags = self.blobs.tags().list().await?;
        while let Some(tag) = tags.next().await {
            let tag = tag?.name;
            println!("tag = {}", tag);
        }
        let abs_path = std::path::absolute(&path)?;
        let t = self.blobs.add_path(&abs_path).await?;
        let ps = format!("component::{}", path.display());
        println!("{:#?} {:#?}", &t, &ps);
        self.blobs.tags().set(ps, t.hash_and_format()).await?;
        Ok(())
    }
}
