/// Conference tier classification.
///
/// Used as a fixed additive input to school prestige calculations.
/// Avoids the feedback loop of live school prestige ↔ conference prestige.
/// Changes only during realignment events — never recalculated automatically.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ConferenceTier {
    PowerConference, // Major TV deals, automatic prestige floor for members
    MidMajor,        // Credible programs, occasional tournament runs
    LowMajor,        // Smaller programs, rare national exposure
    Independent,     // No conference affiliation
}

/// The component scores that feed into a program's overall prestige.
///
/// Prestige is never stored or exposed as a single flat value — it is
/// calculated from these components at season end using weighted sums.
/// Storing components separately makes them auditable ("what's dragging
/// this program's prestige down?") and directly editable in Commissioner mode.
///
/// All fields: 0–99 scale. Weights are applied at calculation time and are
/// subject to change during balancing.
///
/// `coaching_staff` updates immediately when staff changes (hire/fire),
/// not just at season end. All other components update at season end only —
/// prestige is then fixed for the full following season.
///
/// Placeholder — component list and weights subject to change.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProgramPrestige {
    /// Long-term program arc. Blends pre-game authored value with
    /// accumulated in-game success. Moves slowly — never resets.
    pub cumulative_success: u8,

    /// Rolling 3–5 season window of in-game results: win percentage,
    /// conference titles, tournament depth reached.
    /// Recalculated each season end from SeasonRecord history.
    pub recent_success: u8,

    /// Practice facility, weight room, arena quality, player amenities.
    pub facilities: u8,

    /// NIL collective strength and staff salary capacity.
    pub resources: u8,

    /// Arena atmosphere and reputation as a hostile road environment.
    /// A small but real factor — infamous gyms carry genuine recruiting value.
    pub atmosphere: u8,

    /// Derived from head coach reputation + aggregate assistant quality.
    /// Live value — updates on hire/fire events, not just season end.
    pub coaching_staff: u8,

    /// City size, fan base depth, media market footprint.
    /// Mostly fixed; shifts only with sustained multi-season changes.
    pub market: u8,

    /// Derived from MediaNarrative (tone + national_profile) at season end.
    /// Can be moved mid-season by media interactions — a bad press conference
    /// or a viral moment affects this component immediately via MediaNarrative,
    /// with the prestige impact baking in at the next season-end calculation.
    pub media_perception: u8,

    /// Academic reputation and NCAA APR standing.
    pub academics: u8,
}
