use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::model::{conference::Conference, school::School};

/// Loads static world configuration from the `data/` directory.
///
/// Config files define the game world: schools, conferences, settings.
/// Read-only at runtime, always JSON — this loader is not swappable.
/// On a new game, the engine reads from here to build the initial save.
pub struct DataLoader {
    root: PathBuf,
}

impl DataLoader {
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self { root: root.as_ref().to_path_buf() }
    }

    pub fn load_schools(&self) -> Result<Vec<School>> {
        self.load("schools.json")
    }

    pub fn load_conferences(&self) -> Result<Vec<Conference>> {
        self.load("conferences.json")
    }

    fn load<T: serde::de::DeserializeOwned>(&self, filename: &str) -> Result<Vec<T>> {
        let path = self.root.join(filename);
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config: {}", path.display()))?;
        serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse config: {}", path.display()))
    }
}
