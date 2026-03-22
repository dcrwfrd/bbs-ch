use anyhow::Result;

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

pub mod db;
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
    /// Returns `None` if no game state has been saved yet (fresh save directory).
    fn load_game_state(&self) -> Result<Option<GameState>>;
    fn save_game_state(&self, state: &GameState) -> Result<()>;

    fn load_players(&self) -> Result<Vec<Player>>;
    fn save_players(&self, players: &[Player]) -> Result<()>;

    fn load_teams(&self) -> Result<Vec<Team>>;
    fn save_teams(&self, teams: &[Team]) -> Result<()>;

    fn load_prospects(&self) -> Result<Vec<Prospect>>;
    fn save_prospects(&self, prospects: &[Prospect]) -> Result<()>;

    fn load_coaches(&self) -> Result<Vec<Coach>>;
    fn save_coaches(&self, coaches: &[Coach]) -> Result<()>;

    fn load_ads(&self) -> Result<Vec<AD>>;
    fn save_ads(&self, ads: &[AD]) -> Result<()>;

    fn load_boosters(&self) -> Result<Vec<Booster>>;
    fn save_boosters(&self, boosters: &[Booster]) -> Result<()>;

    fn load_media_figures(&self) -> Result<Vec<MediaFigure>>;
    fn save_media_figures(&self, figures: &[MediaFigure]) -> Result<()>;

    fn load_preseason_expectations(&self) -> Result<Vec<PreseasonExpectations>>;
    fn save_preseason_expectations(&self, expectations: &[PreseasonExpectations]) -> Result<()>;

    fn load_news(&self) -> Result<Vec<NewsStory>>;
    fn save_news(&self, stories: &[NewsStory]) -> Result<()>;

    fn load_narratives(&self) -> Result<Vec<MediaNarrative>>;
    fn save_narratives(&self, narratives: &[MediaNarrative]) -> Result<()>;
}
