/// Coaching philosophy / system.
///
/// Placeholder — will tie into the tactics layer. Subject to change.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CoachingStyle {
    UpTempo,
    HalfCourt,
    DefenseFirst,
    DevelopmentFocused,
    RecruiterFirst,
}

/// Assistant coaching specialization.
///
/// Placeholder — subject to change.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CoachSpec {
    OffensiveCoordinator,
    DefensiveCoordinator,
    RecruitingCoordinator,
    PlayerDevelopment,
}

/// Hidden personality dimensions that drive coach behavior and AI decisions.
///
/// For the player's coach, these start as archetype values and shift with
/// outcomes. For NPC coaches, they drive decisions (take the job offer?
/// go public with player criticism?). Scale: 1–20.
///
/// Placeholder — dimensions and scale are subject to change.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoachPersonality {
    pub composure: u8,
    pub ambition: u8,
    pub loyalty: u8,
    pub media_presence: u8,
}

/// Role-specific state for a coach.
///
/// Common fields live on `Coach`. Data that only applies to a specific
/// role lives here in the variant.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CoachRole {
    Head {
        /// Years remaining on current contract.
        contract_years: u8,
        /// Pressure from AD. Drives firing events and media narrative.
        hot_seat: u8,
        /// National profile. Affects recruiting interest floor independent
        /// of program prestige.
        reputation: u8,
        /// Coaching system. Placeholder for future tactics integration.
        style: CoachingStyle,
        /// ID of the coach who developed this coach. Enables coaching trees.
        mentor_id: Option<u32>,
    },
    Assistant {
        spec: CoachSpec,
        /// Likelihood to follow head coach to a new program vs. staying.
        loyalty: u8,
        /// Whether this assistant is a viable future head coach candidate.
        hc_potential: u8,
    },
}

/// A coach — the player's avatar, a rival head coach, or a staff assistant.
///
/// The player controls exactly one `Coach` with `role: CoachRole::Head { .. }`.
/// All rival head coaches share this same type — the game engine distinguishes
/// the player by coach ID, not by a separate type.
///
/// NCAA limits: 3 assistant coaches per program (DOBO and GAs are outside
/// this cap and can be modeled as additional `Assistant` records if needed).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Coach {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub school_id: u32,
    pub salary: u32,
    pub personality: CoachPersonality,
    pub role: CoachRole,
}
