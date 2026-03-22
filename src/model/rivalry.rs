/// A moment that caused a measurable shift in rivalry intensity between two programs.
/// Events are appended over time and feed into the intensity recalculation at season end.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum RivalryEventKind {
    /// Significant upset win that humiliated the rival.
    BigUpset,
    /// Blowout loss that damaged the program's pride.
    BlowoutLoss,
    /// The two programs met in the Final Four or championship.
    FinalFourMeeting,
    /// Public trash talk or controversy between coaching staffs or players.
    CoachTrashTalk,
    /// A heated battle for the same high-profile recruit.
    RecruitingWar,
    /// A long winning streak against this rival was ended.
    RecentStreakBroken,
    /// Freeform narrative event (e.g., from a mod or scripted story moment).
    Custom(String),
}

/// A single logged event that affected rivalry intensity in a given season.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RivalryEvent {
    pub kind: RivalryEventKind,
    /// The season year this event occurred (e.g., 2026).
    pub season: u32,
    /// How much this event shifted intensity. Positive = more intense, negative = cooling.
    /// Typically in the range –0.15 to +0.25.
    pub intensity_delta: f32,
}

/// Dynamic state of a directed rivalry relationship: school A's rivalry toward school B.
///
/// This is **save state**, not config — it lives in the player's save file, not `data/`.
/// The school's initial rivals are declared in `School.rivals` / `School.primary_rival`,
/// but intensity and history are tracked here and evolve over time.
///
/// The relationship is directed: A's `RivalryRecord` toward B is separate from
/// B's `RivalryRecord` toward A. A can have a more intense rivalry with B than
/// B has with A (e.g., a smaller school that obsesses over a bigger rival).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RivalryRecord {
    /// Abbreviation of the school this record is *toward*. Matches `School.abbreviation`.
    pub rival_abbreviation: String,

    /// Current rivalry intensity from this school's perspective.
    /// `0.0` = negligible, `1.0` = maximum intensity.
    /// Affects player motivation, home-crowd atmosphere boost, recruit perception,
    /// and post-game media narrative weight when facing this opponent.
    pub intensity: f32,

    /// Whether this school declared the rival as its `primary_rival`.
    /// Derived from `School.primary_rival` at load time — not independently editable.
    pub is_primary: bool,

    /// Head-to-head games played between these two programs (all-time or tracked window).
    pub games_played: u32,
    /// Wins by the *owning* school in this matchup.
    pub wins: u32,
    /// Losses by the *owning* school in this matchup.
    pub losses: u32,

    /// Geographic proximity factor. Pre-computed from zip codes at load time and
    /// cached here. `0.0` = far apart, `1.0` = same city.
    /// **Not yet implemented** — always `0.0` until zip-distance logic is added.
    pub proximity_factor: f32,

    /// Chronological log of events that have shifted this rivalry's intensity.
    pub events: Vec<RivalryEvent>,
}

impl RivalryRecord {
    /// Creates a fresh rivalry record toward the given school with no history.
    /// `intensity` is seeded at a default based on whether it is a primary rival.
    pub fn new(rival_abbreviation: String, is_primary: bool) -> Self {
        let intensity = if is_primary { 0.7 } else { 0.4 };
        Self {
            rival_abbreviation,
            intensity,
            is_primary,
            games_played: 0,
            wins: 0,
            losses: 0,
            proximity_factor: 0.0, // placeholder until zip-distance logic is implemented
            events: Vec::new(),
        }
    }
}
