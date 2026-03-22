use crate::model::prestige::ConferenceTier;

/// Broad geographic grouping used for scheduling flavor and
/// recruit hometown proximity calculations.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Region {
    Northeast,
    MidAtlantic,
    Southeast,
    Midwest,
    Plains,
    Southwest,
    West,
}

/// A collection of programs that share a conference schedule and
/// automatic tournament bid pathway.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Conference {
    /// Runtime-only. Assigned by `DataLoader` after loading (alphabetical by name).
    /// Not stored in JSON — use `abbreviation` to identify conferences in data files.
    #[serde(skip)]
    pub id: u32,

    pub name: String,
    /// Short identifier. e.g., "ACC", "B1G". Used as the cross-reference key
    /// in data files — `schools.json` references conferences by abbreviation.
    pub abbreviation: String,
    pub region: Region,
    /// Tier classification. Used as a fixed prestige input for member schools.
    /// Changes only during realignment — never auto-recalculated.
    pub tier: ConferenceTier,

    /// Path to the conference logo asset, relative to the `data/` directory.
    /// e.g., `"logos/acc.png"`. `None` if no logo is configured.
    #[serde(default)]
    pub logo: Option<String>,

    /// School abbreviations of member programs. Matches `School.abbreviation`.
    /// Human-readable cross-reference — resolved to `member_ids` by `DataLoader`.
    pub members: Vec<String>,

    /// Runtime-only. Resolved from `members` by `DataLoader`.
    #[serde(skip)]
    pub member_ids: Vec<u32>,
}
