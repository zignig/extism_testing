use std::path::PathBuf;

use anyhow::{anyhow,Result};
use iroh_blobs::store::fs::FsStore;
use n0_future::StreamExt;

#[derive(Debug)]
pub struct Storage {
    blobs: FsStore,
}


impl Storage {
    // Create a new one
    pub async fn new(path: PathBuf) -> Result<Self> {
        let mut full_path = path.clone();
        full_path.push("blobs");

        let blobs = FsStore::load(full_path).await?;
        Ok(Self { blobs })
    }

    // List all the tags that we have
    pub async fn list_tag(&self) -> Result<()> { 
        let mut tags = self.blobs.tags().list().await?;
        while let Some(tag) = tags.next().await {
            let tag = tag?.name;
            println!("tag = {}", tag);
        }
        Ok(())
    }

    // Given a tag get the bytes
    pub async fn get_item(&self,name: String) -> Result<&[u8]> {
        let tag = self.blobs.tags().get(name).await?;
        if let Some(tag) = tag { 
            println!("{:#?}",&tag);
        }
        Err(anyhow!("bork"))
    }

    pub async fn save(&self, path: PathBuf) -> Result<()> {
        let abs_path = std::path::absolute(&path)?;
        let t = self.blobs.add_path(&abs_path).await?;
        let ps = format!("preboot::{}", path.display());
        println!("{:#?} {:#?}", &t, &ps);
        self.blobs.tags().set(ps, t.hash_and_format()).await?;
        self.blobs.tags().delete(t.name).await?;
        Ok(())
    }
}
