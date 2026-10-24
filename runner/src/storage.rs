use std::{env, path::PathBuf};

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

    pub async fn list_tags(&self) -> Result<()> {
        let mut tags = self.blobs.tags().list().await?;
        while let Some(tag) = tags.next().await {
            let tag = tag?.name;
            println!("{}", tag);
        }
        Ok(())
    }

    pub async fn install(&self, path: impl AsRef<std::path::Path>) -> Result<()> {
        let absolute = env::current_dir()?.join(&path);
        println!("upload {:#?} = {:#?}", path.as_ref(), absolute);
        let tag = self.blobs.add_path(&absolute).await?;
        let tag_name = if let Some(file_name) = path.as_ref().file_name() {
            format!("{:?}", file_name)
        } else {
            format!("{:}", path.as_ref().display().to_string())
        };
        self.blobs.tags().set(tag_name, tag.hash).await?;

        Ok(())
    }
}
