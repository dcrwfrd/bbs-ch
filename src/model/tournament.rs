/// What kind of tournament this is — drives scheduling and bid logic.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TournamentKind {
    /// Multi-team event held early in the season (e.g., Battle 4 Atlantis).
    InSeason,
    /// Conference championship tournament — one per conference.
    ConferenceTourney,
    /// The national postseason championship (the big one).
    NationalChampionship,
    /// Consolation / secondary postseason field (e.g., NIT-style).
    SecondaryPostseason,
}

/// Bracket elimination format.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum BracketFormat {
    SingleElimination,
    DoubleElimination,
    /// All teams play each other once; best record wins.
    RoundRobin,
}

/// A tournament in the game world — conference tourneys, in-season events,
/// and the national championship all use this model.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tournament {
    /// Runtime-only. Assigned sequentially by the loader.
    #[serde(skip)]
    pub id: u32,

    pub name: String,
    /// Short identifier shown in brackets/scores. e.g., "NCAAT", "ACC-T".
    pub abbreviation: String,

    /// Path to the tournament logo asset, relative to the `data/` directory.
    /// e.g., `"logos/ncaat.png"`. `None` if no logo is configured.
    #[serde(default)]
    pub logo: Option<String>,

    pub kind: TournamentKind,
    pub bracket_format: BracketFormat,

    /// Number of teams in the field. Common values: 4, 8, 16, 32, 64, 68.
    pub field_size: u32,

    /// City/arena description shown in the UI. e.g., "Indianapolis, IN".
    pub host_site: String,

    /// Set for `ConferenceTourney` — links to `Conference.abbreviation`.
    /// `None` for all other tournament kinds.
    pub conference: Option<String>,

    /// Number of automatic qualifier bids (conference champions).
    pub automatic_bids: u32,

    /// Number of at-large bids selected by committee.
    pub at_large_bids: u32,
}
