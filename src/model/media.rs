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
/// **Direct mutation:** This record is not only updated at season end —
/// it is a live value that can be shifted immediately by media interactions.
/// A poor coach press conference, a player going off-script, or a viral
/// controversy moves `tone` and `national_profile` in real time. Those
/// changes cascade into recruiting confidence, player morale, AD patience,
/// and booster satisfaction during the current season. The prestige
/// calculation then reads this state at season end.
///
/// Affects: recruit confidence during visits, transfer portal interest,
/// fan expectations, coach hot-seat pressure, booster satisfaction.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MediaNarrative {
    pub program_id: u32,
    pub tone: MediaTone,
    /// -1000 to 1000. Volume of national media attention.
    /// Positive = high visibility (good or bad). Drives story amplification.
    pub national_profile: i16,
    /// Most recent story driving the current narrative.
    pub latest_story_id: Option<u32>,
}

/// The platform a media figure operates on.
///
/// Placeholder — subject to change.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MediaPlatform {
    Television,
    Newspaper,
    Podcast,
    SocialMedia,
    Blog,
}

/// Geographic or topical scope of coverage.
///
/// Placeholder — subject to change.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CoverageScope {
    National,        // Covers the sport broadly; all programs in view
    Conference(u32), // Beat reporter for a specific conference
    School(u32),     // Dedicated beat for one program
}

/// A media figure who covers college basketball in the game world.
///
/// Media figures generate and amplify `NewsStory` records. Building or
/// damaging relationships with them affects narrative tone over time.
/// Per-program bias is tracked separately as a relationship record
/// (not stored here) so it can shift dynamically.
///
/// Placeholder — fields and mechanics are subject to change.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MediaFigure {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub platform: MediaPlatform,
    pub scope: CoverageScope,
    /// 1–99. How much reach this figure has. Drives how widely their
    /// stories spread and how much they move program narrative scores.
    pub reach: u8,
    /// Tendency toward hot takes and controversy vs. measured analysis.
    /// Placeholder scale: 1–20.
    pub sensationalism: u8,
}

/// Preseason poll position and media expectations for a program's upcoming season.
///
/// Generated at the start of each season during `SeasonPhase::Preseason`.
/// Sets the expectations bar the program is measured against all year.
///
/// High hype raises recruit visit confidence early but creates a fragile
/// narrative — underperforming a top-5 preseason ranking damages `MediaNarrative`
/// more than losing as an unranked team. Overachieving as a dark horse has
/// the opposite effect.
///
/// Coach and player responses during preseason media day (interviews, press
/// conferences) can shift the opening `MediaNarrative.tone` before a game
/// is played. A bold guarantee or a dismissive response to a rival becomes
/// bulletin board material and sets the season's media temperature.
///
/// Placeholder — fields and mechanics are subject to change.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PreseasonExpectations {
    pub program_id: u32,
    pub season: u32,

    /// Position in the preseason AP/Coaches poll. None if unranked.
    pub poll_position: Option<u32>,
    /// Where the media predicts this program will finish in its conference.
    /// 1 = projected first place.
    pub conf_predicted_finish: u8,
    /// Media's projected regular season win total.
    pub win_total_prediction: u8,

    /// Aggregate expectation score distilled from poll position, predicted
    /// finish, and media coverage volume. Drives early-season morale modifier
    /// and recruit visit confidence. Placeholder scale: 0–99.
    pub hype_level: u8,

    /// The narrative tone the season opens with, before any games are played.
    /// Shifted by preseason media interactions before it locks in.
    pub opening_tone: MediaTone,
}
