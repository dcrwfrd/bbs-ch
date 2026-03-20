/// The physical venue where a school plays home games.
/// Belongs to the school, not a specific season's team.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Arena {
    pub name: String,
    pub capacity: u32,
    /// 1–99. Affects home-court advantage in the sim and recruit visit impressions.
    pub atmosphere: u8,
}

/// A college institution — the persistent entity that spans all seasons.
///
/// Distinct from `Team`, which represents a specific season's roster.
/// The school owns everything that doesn't change year to year: its name,
/// identity, facilities, arena, prestige, and conference membership.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct School {
    pub id: u32,
    pub name: String,         // e.g., "Lakewood University"
    pub short_name: String,   // e.g., "Lakewood"
    pub nickname: String,     // e.g., "Wolves"
    pub abbreviation: String, // e.g., "LWU"
    pub city: String,
    pub state: String,
    pub conference_id: u32,

    // Visual identity (RGB)
    pub primary_color: [u8; 3],
    pub secondary_color: [u8; 3],

    pub arena: Arena,

    // Institutional ratings (1–99)
    /// How blue-blood the program is perceived nationally.
    /// Affects recruit interest floors and media coverage volume.
    pub prestige: u8,
    /// Practice facilities, film rooms, weight room, player amenities.
    /// Affects recruit visit impressions and player development rate.
    pub facilities: u8,
    /// Affects academically-minded recruits and graduate transfer interest.
    pub academic_reputation: u8,

    /// Annual NIL collective budget available for the current coaching staff.
    pub nil_budget: u32,
}
