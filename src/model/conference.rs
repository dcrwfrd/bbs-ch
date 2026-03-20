/// Broad geographic grouping used for scheduling flavor and
/// recruit hometown proximity calculations.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Region {
    Northeast,
    MidAtlantic,
    Southeast,
    Midwest,
    Plains,
    Southwest,
    West,
}

/// A collection of programs that share a conference schedule and
/// automatic tournament bid pathway.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Conference {
    pub id: u32,
    pub name: String,        // e.g., "Midlands Athletic Conference"
    pub abbreviation: String, // e.g., "MAC"
    pub region: Region,
    /// 1–99. Affects strength-of-schedule perception, recruiting pipeline
    /// quality, and media visibility of member programs.
    pub prestige: u8,
    /// IDs of member programs.
    pub member_ids: Vec<u32>,
}
