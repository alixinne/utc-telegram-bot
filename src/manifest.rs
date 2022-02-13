use std::{collections::HashMap, path::Path};

use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ManifestError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}

#[derive(Default, Debug, Clone, Deserialize)]
#[cfg_attr(feature = "renderer", derive(serde::Serialize))]
pub struct Manifest {
    pub images: HashMap<String, ImageManifest>,
}

impl Manifest {
    pub fn hash(&self, filename: &str) -> Option<&str> {
        self.images.get(filename).map(|entry| entry.hash.as_str())
    }

    pub fn load(file: impl AsRef<Path>) -> Result<Self, ManifestError> {
        let f = std::fs::File::open(file.as_ref())?;
        let manifest = serde_json::from_reader(f)?;

        debug!(
            "loaded manifest from {}: {:?}",
            file.as_ref().display(),
            manifest
        );

        Ok(manifest)
    }

    #[cfg(feature = "renderer")]
    pub fn write(&self, file: impl AsRef<Path>) -> Result<(), ManifestError> {
        debug!(
            "writing manifest at {}: {:?}",
            file.as_ref().display(),
            self
        );

        let f = std::fs::File::create(file)?;
        serde_json::to_writer_pretty(f, self)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "renderer", derive(serde::Serialize))]
pub struct ImageManifest {
    pub hash: String,
}

impl ImageManifest {
    #[cfg(feature = "renderer")]
    pub fn new(data: &[u8]) -> Self {
        use sha2::{Digest, Sha256};

        let mut hash = Sha256::new();
        hash.update(data);

        Self {
            hash: hex::encode(hash.finalize()),
        }
    }
}
