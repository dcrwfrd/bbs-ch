/// Positionless labels used for player description and recruiting.
///
/// Traditional 5-position roles (PG/SG/SF/PF/C) belong to the tactics layer,
/// not to player identity. These reflect how modern college basketball
/// actually categorizes and recruits players.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Position {
    Guard,  // Primary ball-handlers and perimeter shooters
    Wing,   // Versatile perimeter players, 3-and-D types
    Big,    // Frontcourt players, rim presence
    Combo,  // Genuinely positionless; can credibly play multiple roles
}

/// Year of eligibility.
/// Graduate transfers are distinct from seniors — they carry immediate
/// eligibility and a different recruiting profile (portal vs. high school).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Class {
    Freshman,
    Sophomore,
    Junior,
    Senior,
    GradTransfer,
}

/// On-court skill ratings on a 1–99 scale.
///
/// Placeholder — field set and weights are subject to change.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlayerSkills {
    // Shooting
    pub inside: u8,
    pub midrange: u8,
    pub three_point: u8,
    pub free_throw: u8,

    // Playmaking
    pub passing: u8,
    pub ball_handling: u8,
    pub decision_making: u8,

    // Defense
    pub perimeter_def: u8,
    pub interior_def: u8,
    pub help_def_iq: u8,

    // Athleticism
    pub speed: u8,
    pub strength: u8,
    pub vertical: u8,

    // Feel for the game
    pub basketball_iq: u8,
    pub rebounding: u8,
    pub clutch: u8,
}

/// Hidden personality dimensions that drive simulation behavior.
///
/// Never shown directly to the player — surfaces through observable
/// signals: morale swings, performance patterns, interaction outcomes,
/// and the visible `PlayerTrait` list. Scale: 1–20.
///
/// Placeholder — dimensions and scale are subject to change.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlayerPersonality {
    pub leadership: u8,
    pub professionalism: u8,
    pub coachability: u8,
    pub resilience: u8,
    pub ambition: u8,
    pub ego: u8,
}

/// Visible personality surface — 1–3 traits per player.
///
/// Derived from the hidden `PlayerPersonality` values at generation time.
/// Each trait has mechanical implications in the sim and interaction systems.
/// Note: `trait` is a reserved keyword in Rust, so this type is `PlayerTrait`.
///
/// Placeholder — trait list is subject to change.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PlayerTrait {
    // --- Presence & leadership ---
    NaturalLeader,
    QuietLeader,
    Hothead,
    Mentor,
    Catalyst,
    Cancerous,
    Introvert,

    // --- Work ethic & development ---
    Grinder,
    Coasting,
    GymRat,
    LateBloom,
    EarlyPeak,

    // --- Mental & pressure ---
    Clutch,
    Fragile,
    Mercurial,
    Streaky,
    FlatTrackBully,

    // --- Team dynamics ---
    TeamFirst,
    BallHog,
    LoneWolf,
    LockerRoom,

    // --- Media & public profile ---
    MediaDarling,
    MediaShy,
    Polarizing,

    // --- Life & circumstance ---
    Homesick,
    Motivated,
    FamilyFirst,
    NilHunter,
    Academic,
    AtRisk,
    InjuryProne,
    IronMan,
}

/// A player on an active roster.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Player {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub position: Position,
    pub class: Class,

    pub height_in: u8,
    pub weight_lb: u16,

    pub skills: PlayerSkills,
    pub personality: PlayerPersonality,
    pub traits: Vec<PlayerTrait>,

    pub morale: i8,

    pub hometown: String,
    pub home_state: String,
    pub nil_value: u32,
}
