use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{de::DeserializeOwned, Serialize};

use crate::model::{
    ad::AD,
    booster::Booster,
    coach::Coach,
    game_state::GameState,
    media::{MediaFigure, MediaNarrative, NewsStory, PreseasonExpectations},
    player::Player,
    prospect::Prospect,
    team::Team,
};
use super::Storage;

/// JSON-backed save storage.
///
/// Each save is a directory under `saves/<save_name>/`. Files:
///   game_state.json, players.json, teams.json, prospects.json,
///   coaches.json, ads.json, boosters.json, media.json,
///   preseason.json, news.json, narratives.json
///
/// Swap this for `sqlite::SqliteStorage` when querying and scale require it.
/// Game logic only ever touches `&dyn Storage`.
pub struct JsonStorage {
    root: PathBuf,
}

impl JsonStorage {
    /// Creates a `JsonStorage` rooted at `root`, creating the directory
    /// if it does not already exist.
    pub fn new(root: impl AsRef<Path>) -> Result<Self> {
        let root = root.as_ref().to_path_buf();
        std::fs::create_dir_all(&root)
            .with_context(|| format!("Failed to create save directory: {}", root.display()))?;
        Ok(Self { root })
    }

    /// Reads a JSON file and deserializes it as `Vec<T>`.
    /// Returns an empty Vec if the file does not exist (new game).
    fn load<T: DeserializeOwned>(&self, filename: &str) -> Result<Vec<T>> {
        let path = self.root.join(filename);
        if !path.exists() {
            return Ok(Vec::new());
        }
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read {}", path.display()))?;
        serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse {}", path.display()))
    }

    /// Serializes `data` as pretty-printed JSON and writes it to `filename`.
    fn save<T: Serialize>(&self, filename: &str, data: &[T]) -> Result<()> {
        let path = self.root.join(filename);
        let content = serde_json::to_string_pretty(data).context("Failed to serialize")?;
        std::fs::write(&path, content)
            .with_context(|| format!("Failed to write {}", path.display()))
    }

    /// Reads a single JSON record. Returns `None` if the file does not exist.
    fn load_one<T: DeserializeOwned>(&self, filename: &str) -> Result<Option<T>> {
        let path = self.root.join(filename);
        if !path.exists() {
            return Ok(None);
        }
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read {}", path.display()))?;
        serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse {}", path.display()))
            .map(Some)
    }

    /// Serializes a single record as pretty-printed JSON.
    fn save_one<T: Serialize>(&self, filename: &str, data: &T) -> Result<()> {
        let path = self.root.join(filename);
        let content = serde_json::to_string_pretty(data).context("Failed to serialize")?;
        std::fs::write(&path, content)
            .with_context(|| format!("Failed to write {}", path.display()))
    }
}

impl Storage for JsonStorage {
    fn load_game_state(&self) -> Result<Option<GameState>> { self.load_one("game_state.json") }
    fn save_game_state(&self, state: &GameState) -> Result<()> { self.save_one("game_state.json", state) }

    fn load_players(&self) -> Result<Vec<Player>> { self.load("players.json") }
    fn save_players(&self, v: &[Player]) -> Result<()> { self.save("players.json", v) }

    fn load_teams(&self) -> Result<Vec<Team>> { self.load("teams.json") }
    fn save_teams(&self, v: &[Team]) -> Result<()> { self.save("teams.json", v) }

    fn load_prospects(&self) -> Result<Vec<Prospect>> { self.load("prospects.json") }
    fn save_prospects(&self, v: &[Prospect]) -> Result<()> { self.save("prospects.json", v) }

    fn load_coaches(&self) -> Result<Vec<Coach>> { self.load("coaches.json") }
    fn save_coaches(&self, v: &[Coach]) -> Result<()> { self.save("coaches.json", v) }

    fn load_ads(&self) -> Result<Vec<AD>> { self.load("ads.json") }
    fn save_ads(&self, v: &[AD]) -> Result<()> { self.save("ads.json", v) }

    fn load_boosters(&self) -> Result<Vec<Booster>> { self.load("boosters.json") }
    fn save_boosters(&self, v: &[Booster]) -> Result<()> { self.save("boosters.json", v) }

    fn load_media_figures(&self) -> Result<Vec<MediaFigure>> { self.load("media.json") }
    fn save_media_figures(&self, v: &[MediaFigure]) -> Result<()> { self.save("media.json", v) }

    fn load_preseason_expectations(&self) -> Result<Vec<PreseasonExpectations>> { self.load("preseason.json") }
    fn save_preseason_expectations(&self, v: &[PreseasonExpectations]) -> Result<()> { self.save("preseason.json", v) }

    fn load_news(&self) -> Result<Vec<NewsStory>> { self.load("news.json") }
    fn save_news(&self, v: &[NewsStory]) -> Result<()> { self.save("news.json", v) }

    fn load_narratives(&self) -> Result<Vec<MediaNarrative>> { self.load("narratives.json") }
    fn save_narratives(&self, v: &[MediaNarrative]) -> Result<()> { self.save("narratives.json", v) }
}
