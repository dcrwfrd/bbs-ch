/// Athletic director personality dimensions.
///
/// Placeholder — dimensions and scale are subject to change.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AdPersonality {
    pub patience: u8,    // Tolerance for down seasons before firing
    pub ambition: u8,    // Appetite for program elevation (spending, hires)
    pub loyalty: u8,     // Tendency to back the coach publicly vs. distancing
    pub publicity: u8,   // Media engagement; affects narrative amplification
}

/// The athletic director at a school.
///
/// Holds the power to extend, renegotiate, or terminate the head coach's
/// contract. Their personality drives how quickly hot seat pressure builds
/// and how aggressively they fund NIL, facilities, and assistant salaries.
///
/// Placeholder — fields and mechanics are subject to change.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AD {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub school_id: u32,
    pub salary: u32,
    pub tenure_years: u8,  // Years in the role; longer tenure = more stable
    pub personality: AdPersonality,
}
