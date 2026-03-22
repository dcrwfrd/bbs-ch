/// Rough wealth tier of a booster.
///
/// Placeholder — tiers and financial implications subject to change.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum WealthTier {
    Local,    // Community-level donor; small but consistent NIL contributions
    Regional, // Meaningful NIL budget; can anchor a position group
    Major,    // Program-defining money; expects influence in return
    Elite,    // Generational wealth; shapes program direction
}

/// What a booster prioritizes with their funding.
///
/// Placeholder — subject to change.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum BoosterFocus {
    Winning,      // Results-driven; patience tied directly to record
    Development,  // Invested in player growth; longer patience horizon
    Recruiting,   // Cares about landing top classes; NIL-forward
    Facilities,   // Wants to see the program's physical plant elevated
    Tradition,    // Old-school; resistant to culture changes
}

/// A named booster affiliated with a program.
///
/// A small number of key boosters (2–5) per school. Their collective
/// engagement level, funding, and mood feed into the broader booster
/// network summary used by the sim. Dissatisfied major boosters can
/// pull NIL funding, lobby the AD, or generate negative media stories.
///
/// Placeholder — fields and mechanics are subject to change.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Booster {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub school_id: u32,
    pub wealth_tier: WealthTier,
    pub focus: BoosterFocus,
    /// How tied their loyalty is to the coach vs. the program.
    /// High = follows the coach; low = stays with the school regardless.
    pub coach_loyalty: u8,
    /// Current mood toward the program. Affects NIL contributions and AD pressure.
    /// Placeholder scale: 1–100.
    pub satisfaction: u8,
    /// Annual NIL contribution ceiling at current satisfaction level.
    pub nil_contribution: u32,
}
