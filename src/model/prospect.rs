use crate::model::player::{Class, PlayerPersonality, PlayerSkills, PlayerTrait, Position};

/// How interested a prospect is in a given program.
/// Ordered lowest to highest — comparisons rely on this ordering.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub enum InterestLevel {
    None,
    Aware,       // Knows the program exists; no active engagement
    Interested,  // Engaged; following the program
    Considering, // On the real list; taking the process seriously
    TopSchool,   // Among the final few
    Leader,      // Current favorite
}

/// Where a prospect stands in their commitment decision.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CommitmentStatus {
    Uncommitted,
    /// Verbal — public but not binding. Can decommit.
    SoftCommit { program_id: u32 },
    /// Firm verbal — unlikely to flip but not yet signed.
    Committed { program_id: u32 },
    /// National Letter of Intent signed. Binding.
    Signed { program_id: u32 },
}

/// A prospect's active relationship with one specific program.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SchoolInterest {
    pub program_id: u32,
    pub level: InterestLevel,
    pub has_offer: bool,
    pub official_visit_taken: bool,
}

/// A recruit who has not yet enrolled at a college program.
///
/// Shares the personality and skills systems with `Player`, but carries
/// recruiting-specific state. When a prospect signs and enrolls, they
/// become a `Player`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Prospect {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub position: Position,
    pub class: Class, // Freshman for HS prospects; varies for transfers

    // Physical
    pub height_in: u8,
    pub weight_lb: u16,

    // Recruiting profile
    pub star_rating: u8,            // 1–5
    pub nat_rank: Option<u32>,      // None if unranked
    pub pos_rank: Option<u32>,      // None if unranked

    // Scouted ability (may carry noise — true skills revealed over time)
    pub skills: PlayerSkills,
    /// Long-term ceiling. Gap between current skills and potential
    /// represents development upside.
    pub potential: u8, // 1–99

    // Personality
    pub personality: PlayerPersonality,
    pub traits: Vec<PlayerTrait>,

    // Recruiting state
    pub commit_status: CommitmentStatus,
    /// Programs the prospect is actively tracking.
    pub top_schools: Vec<SchoolInterest>,
    /// NCAA allows 5 official visits total.
    pub off_visits_rem: u8,

    // Background
    pub hometown: String,
    pub home_state: String,
    pub nil_target: u32,
}
