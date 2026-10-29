use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncSeekExt, AsyncWriteExt, SeekFrom};

pub struct FileStorage {
    pub file_path: PathBuf,
}

impl FileStorage {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self { file_path: path.as_ref().to_path_buf() }
    }

    pub async fn preallocate(&self, total_size: u64) -> Result<()> {
        if let Some(parent) = self.file_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(false)
            .open(&self.file_path)
            .await
            .context("Échec d'ouverture du fichier OpenDM")?;

        file.set_len(total_size).await?;
        Ok(())
    }

    pub async fn write_at(file: &mut File, offset: u64, data: &[u8]) -> Result<()> {
        file.seek(SeekFrom::Start(offset)).await?;
        file.write_all(data).await?;
        Ok(())
    }
}
