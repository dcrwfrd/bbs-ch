use crate::model::prestige::ProgramPrestige;

/// The physical venue where a school plays home games.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HomeCourt {
    pub name: String,
    pub capacity: u32,
    /// 1–99. Atmosphere/hostility rating. Affects home-court advantage
    /// in the sim and recruit visit impressions.
    pub atmosphere: u8,
}

/// A college institution — the persistent entity that spans all seasons.
///
/// Distinct from `Team`, which represents a specific season's roster.
/// The school owns everything that doesn't change year to year: its name,
/// identity, facilities, home court, prestige, and conference membership.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct School {
    /// Runtime-only. Assigned by `DataLoader` after loading (alphabetical by name).
    /// Not stored in JSON — use `abbreviation` to identify schools in data files.
    #[serde(skip)]
    pub id: u32,

    pub name: String,
    pub nickname: String,
    /// Short identifier. e.g., "LSU", "DUKE". Used as the cross-reference key
    /// in data files — `conferences.json` lists members by abbreviation.
    pub abbreviation: String,
    /// If true, the abbreviation is preferred in common discourse over the
    /// full name (e.g., "LSU" rather than "Louisiana State").
    pub prefer_abbreviation: bool,

    /// US zip code. City and state are extrapolated from this as needed.
    pub zip_code: String,

    /// Conference abbreviation. Matches `Conference.abbreviation` in conferences.json.
    /// Human-readable cross-reference — resolved to `conference_id` by `DataLoader`.
    pub conference: String,

    /// Runtime-only. Resolved from `conference` by `DataLoader`.
    #[serde(skip)]
    pub conference_id: u32,

    /// Primary brand color as a hex string. e.g., `"#003087"`.
    pub primary_color: String,
    /// Secondary brand color as a hex string. e.g., `"#ffffff"`.
    pub secondary_color: String,

    pub home_court: HomeCourt,

    /// Component-based prestige. Recalculated at season end.
    /// See `ProgramPrestige` for the full breakdown.
    pub prestige: ProgramPrestige,

    /// Total undergraduate enrollment. Affects market size, fan base
    /// calculations, and recruiting reach.
    #[serde(default)]
    pub enrollment: u32,

    /// Annual NIL collective budget. Raw spendable value — feeds into
    /// `prestige.resources` at calculation time but stored separately
    /// as it is directly spent and adjusted during recruiting.
    pub nil_budget: u32,

    /// The one school this program considers its defining rival — the most
    /// intense, identity-shaping matchup. This relationship is **mutual and
    /// enforced**: if school A declares school B as its primary rival, B must
    /// also declare A. The loader will hard-error if this invariant is broken.
    ///
    /// Games between two schools who share a primary-rival relationship get
    /// special "rivalry game" treatment with amplified effects on momentum,
    /// crowd atmosphere, and recruiting impact.
    ///
    /// Uses `School.abbreviation` as the cross-reference key.
    #[serde(default)]
    pub primary_rival: Option<String>,

    /// Additional rivals beyond the primary rival. This list is **directed**:
    /// school A can consider school B a rival without B reciprocating.
    /// Unilateral rivalry still affects A's players' motivation and fan response
    /// when facing B, but the "rivalry game" banner only appears if both teams
    /// list each other. Uses `School.abbreviation` as the cross-reference key.
    ///
    /// Rivalries in this list are dynamic — intensity drifts over time based on
    /// games played, geographic proximity, and in-game events.
    #[serde(default)]
    pub rivals: Vec<String>,
}
