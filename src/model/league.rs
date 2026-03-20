/// The emotional tone of a piece of media coverage.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MediaTone {
    Glowing,   // "Best team in the country"
    Positive,
    Neutral,
    Skeptical,
    Critical,
    Hostile,   // Hot take / pile-on territory
}

/// A generated news story that exists in the game world.
///
/// Produced by the sim engine in response to events: wins, upsets,
/// transfers, coaching decisions, rankings movements. Affects program
/// reputation and player morale over time.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NewsStory {
    pub id: u32,
    pub headline: String,
    pub tone: MediaTone,
    /// Programs involved (multiple for rivalry/upset stories).
    pub program_ids: Vec<u32>,
    /// Players specifically named in the story.
    pub player_idx: Vec<u32>,
    /// The sim week this story was generated.
    pub week: u32,
}

/// The current media narrative around a program.
///
/// A rolling summary of how the press perceives a program right now —
/// distinct from historical prestige. A blue-blood having a down year
/// can carry a negative narrative despite high prestige.
///
/// Affects: recruit confidence during visits, transfer portal interest,
/// fan expectations, coach hot-seat pressure.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MediaNarrative {
    pub program_id: u32,
    pub tone: MediaTone,
    /// -1000 to 1000. Volume of national media attention.
    pub national_profile: i16,
    /// Most recent story driving the current narrative.
    pub latest_story_id: Option<u32>,
}
