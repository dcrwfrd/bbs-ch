/// A season-specific team — one school's roster for one year.
///
/// Distinct from `School`, which holds the persistent institutional identity.
/// A new `Team` is assembled each season as players exhaust eligibility,
/// enter the transfer portal, or graduate.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Team {
    pub id: u32,
    pub school_id: u32,
    pub season: u32, // e.g., 2025 for the 2024–25 season

    /// IDs of players on this season's roster.
    pub roster: Vec<u32>,
}
