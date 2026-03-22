use std::fmt;

pub const MIN_START_SEASON: u32 = 1950;
pub const MAX_START_SEASON: u32 = 2050;
pub const DEFAULT_START_SEASON: u32 = 2027;

/// A calendar date in the game world. The primary unit of in-game time.
///
/// `Copy` because it's a 6-byte value passed freely throughout the UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GameDate {
    pub year: u32,
    pub month: u8,
    pub day: u8,
}

impl GameDate {
    pub fn new(year: u32, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    /// Starting date for a new game: April 1st of the season year.
    /// `season` is the year the season ends (2027 = 2026–27). April puts
    /// us at the start of the post-tournament offseason / spring portal window.
    pub fn season_open(season: u32) -> Self {
        Self { year: season, month: 4, day: 1 }
    }

    /// Returns this date advanced by one calendar day.
    pub fn advance_one_day(self) -> Self {
        let days_in_month: u8 = match self.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => if self.year % 4 == 0 { 29 } else { 28 },
            _ => 30,
        };
        if self.day < days_in_month {
            Self { day: self.day + 1, ..self }
        } else if self.month < 12 {
            Self { month: self.month + 1, day: 1, ..self }
        } else {
            Self { year: self.year + 1, month: 1, day: 1 }
        }
    }

    /// Returns this date advanced by `n` calendar days.
    pub fn advance_by(self, n: u32) -> Self {
        (0..n).fold(self, |d, _| d.advance_one_day())
    }
}

impl fmt::Display for GameDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let month = match self.month {
            1 => "Jan",  2 => "Feb",  3 => "Mar",
            4 => "Apr",  5 => "May",  6 => "Jun",
            7 => "Jul",  8 => "Aug",  9 => "Sep",
            10 => "Oct", 11 => "Nov", 12 => "Dec",
            _ => "???",
        };
        write!(f, "{} {}, {}", month, self.day, self.year)
    }
}

/// Where in the annual calendar the current season sits.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SeasonPhase {
    SpringPortalWindow,
    SummerContact,
    FallPortalWindow,
    EarlySigningPeriod,
    SigningDay,
    Preseason,
    RegularSeason,
    ConferenceTournament,
    NcaaTournament,
    Offseason,
}

impl SeasonPhase {
    pub fn label(&self) -> &'static str {
        match self {
            Self::SpringPortalWindow  => "Spring Portal",
            Self::SummerContact       => "Summer Contact",
            Self::FallPortalWindow    => "Fall Portal",
            Self::EarlySigningPeriod  => "Early Signing",
            Self::SigningDay          => "Signing Day",
            Self::Preseason           => "Preseason",
            Self::RegularSeason       => "Regular Season",
            Self::ConferenceTournament => "Conf. Tournament",
            Self::NcaaTournament      => "NCAA Tournament",
            Self::Offseason           => "Offseason",
        }
    }
}

/// Determines what actions the player can take and how the sim behaves.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SaveMode {
    Normal,
    Commissioner,
    Observer,
}

/// The saved session context for one dynasty run.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GameState {
    pub save_name: String,
    pub save_mode: SaveMode,
    /// The season year in progress (year the season ends — 2027 = 2026–27).
    pub season: u32,
    pub phase: SeasonPhase,
    /// The current in-game calendar date. Advances day-by-day via player action.
    pub current_date: GameDate,
    /// The `Coach` record the player controls. `None` in Observer mode.
    pub player_coach_id: Option<u32>,
    /// The `School` the player is currently coaching at. `None` in Observer mode.
    pub player_school_id: Option<u32>,
}
