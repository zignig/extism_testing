use std::path::PathBuf;

use anyhow::{Result, anyhow};
use iroh_blobs::store::fs::FsStore;
use n0_future::StreamExt;
use tracing::error;

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

    // List all the tags that we have under this prefix
    pub async fn list_tags(&self, prefix: &str) -> Result<Vec<String>> {
        let mut tags = self.blobs.tags().list_prefix(prefix).await?;
        let mut tag_list = Vec::new();
        while let Some(tag) = tags.next().await {
            let tag = tag?.name;
            let tag_name = format!("{}", tag);
            tag_list.push(tag_name);
        }
        Ok(tag_list)
    }

    // Given a tag get the bytes
    pub async fn get_item(&self, name: String) -> Result<Option<&[u8]>> {
        let tag = self.blobs.tags().get(&name).await?;
        let name = name.as_str();
        if let Some(tag) = tag {
            error!("{:#?}", &tag);
            // get the blob
            return Ok(None);
        }
        Err(anyhow!("bork on getting {}", name))
    }

    // Take a blog and put it into storage
    pub async fn save(&self, path: PathBuf) -> Result<()> {
        let abs_path = std::path::absolute(&path)?;
        let t = self.blobs.add_path(&abs_path).await?;

        let ps = if let Some(file_name) = path.file_name().and_then(|os_str| os_str.to_str()) {
            format!("preboot::{}", file_name)
        } else {
            format!("preboot::{}", path.display())
        };
        println!("{:#?} {:#?}", &t, &ps);
        self.blobs.tags().set(ps, t.hash_and_format()).await?;
        // Delete the auto tag
        self.blobs.tags().delete(t.name).await?;
        Ok(())
    }
}
