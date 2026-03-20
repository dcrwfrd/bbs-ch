use anyhow::Result;

use crate::model::{
    league::{MediaNarrative, NewsStory},
    player::Player,
    prospect::Prospect,
    team::Team,
};

pub mod json;

/// The permanent seam between game logic and persistence.
///
/// All save-state I/O goes through this trait. `json::JsonStorage` is the current
/// implementation. When SQLite is introduced, it implements this same trait
/// and the swap requires no changes to game logic.
///
/// Schools and conferences are config, not save state — they are loaded
/// once at startup by `crate::data::DataLoader` and are not part of this trait.
pub trait Storage {
    fn load_players(&self) -> Result<Vec<Player>>;
    fn save_players(&self, players: &[Player]) -> Result<()>;

    fn load_teams(&self) -> Result<Vec<Team>>;
    fn save_teams(&self, teams: &[Team]) -> Result<()>;

    fn load_prospects(&self) -> Result<Vec<Prospect>>;
    fn save_prospects(&self, prospects: &[Prospect]) -> Result<()>;

    fn load_news(&self) -> Result<Vec<NewsStory>>;
    fn save_news(&self, stories: &[NewsStory]) -> Result<()>;

    fn load_narratives(&self) -> Result<Vec<MediaNarrative>>;
    fn save_narratives(&self, narratives: &[MediaNarrative]) -> Result<()>;
}
