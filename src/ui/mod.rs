pub mod content;
pub mod league_builder;
pub mod main_menu;
pub mod new_game;
pub mod sidebar;
pub mod theme;
pub mod ticker;
pub mod top_bar;

pub use league_builder::PrestigeField;
pub use new_game::{NewGameState, WizardStep};

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use iced::widget::image::Handle as ImageHandle;

use bbs_ch::model::{game_state::GameState, league::League, school::School};
use iced::Color;

// ---------------------------------------------------------------------------
// Color helpers
// ---------------------------------------------------------------------------

/// Parses a `"#rrggbb"` hex string into an iced `Color`.
/// Returns a neutral gray on malformed input.
pub fn hex_to_color(hex: &str) -> Color {
    let h = hex.trim_start_matches('#');
    if h.len() != 6 {
        return Color { r: 0.3, g: 0.3, b: 0.35, a: 1.0 };
    }
    let r = u8::from_str_radix(&h[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&h[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&h[4..6], 16).unwrap_or(0);
    Color::from_rgb8(r, g, b)
}

/// Blends `base` toward `tint` by `amount` (0.0 = pure base, 1.0 = pure tint).
fn blend(base: Color, tint: Color, amount: f32) -> Color {
    let a = amount.clamp(0.0, 1.0);
    Color {
        r: base.r * (1.0 - a) + tint.r * a,
        g: base.g * (1.0 - a) + tint.g * a,
        b: base.b * (1.0 - a) + tint.b * a,
        a: 1.0,
    }
}

// ---------------------------------------------------------------------------
// Team theme
// ---------------------------------------------------------------------------

/// Derived surface colors for the in-game UI, tinted toward the selected
/// school's primary and secondary colors. Used by top_bar, sidebar, and ticker.
#[derive(Debug, Clone, Copy)]
pub struct TeamTheme {
    pub bar_bg: Color,
    pub hub_bg: Color,
    pub panel_bg: Color,
    pub sidebar_bg: Color,
    pub ticker_bg: Color,
    /// Raw primary color — used for the color swatch and accents.
    pub primary: Color,
    /// Raw secondary color — available for future accent use.
    #[allow(dead_code)]
    pub secondary: Color,
}

impl TeamTheme {
    /// Builds a theme tinted toward the school's primary and secondary colors.
    pub fn from_school(school: &School) -> Self {
        let primary = hex_to_color(&school.primary_color);
        let secondary = hex_to_color(&school.secondary_color);
        Self::from_colors(primary, secondary)
    }

    pub fn from_colors(primary: Color, secondary: Color) -> Self {
        // Very dark base that receives the tint — keeps the UI legible at all
        // team color values (even bright yellows or whites).
        let dark = Color { r: 0.06, g: 0.06, b: 0.09, a: 1.0 };
        Self {
            bar_bg:     blend(dark, primary, 0.20),
            hub_bg:     blend(dark, primary, 0.32),
            panel_bg:   blend(dark, primary, 0.24),
            sidebar_bg: blend(dark, secondary, 0.22),
            ticker_bg:  blend(dark, primary, 0.15),
            primary,
            secondary,
        }
    }

    /// Neutral dark theme for Observer mode or when no school is selected.
    pub fn observer() -> Self {
        Self {
            bar_bg:     Color { r: 0.10, g: 0.10, b: 0.13, a: 1.0 },
            hub_bg:     Color { r: 0.16, g: 0.16, b: 0.22, a: 1.0 },
            panel_bg:   Color { r: 0.12, g: 0.12, b: 0.17, a: 1.0 },
            sidebar_bg: Color { r: 0.10, g: 0.10, b: 0.14, a: 1.0 },
            ticker_bg:  Color { r: 0.08, g: 0.08, b: 0.11, a: 1.0 },
            primary:    Color { r: 0.3,  g: 0.3,  b: 0.35, a: 1.0 },
            secondary:  Color { r: 0.2,  g: 0.2,  b: 0.25, a: 1.0 },
        }
    }
}

// ---------------------------------------------------------------------------
/// Which top-bar nav item is currently active.
/// Determines sidebar placement (team → left, league → right) and content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NavItem {
    // Team-centered
    Roster,
    Recruiting,
    Schedule,
    Tactics,
    Staff,
    // League-centered
    Standings,
    Rankings,
    Scores,
    Portal,
    LeagueNews,
}

impl NavItem {
    pub fn is_team(&self) -> bool {
        matches!(self, Self::Roster | Self::Recruiting | Self::Schedule | Self::Tactics | Self::Staff)
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Roster      => "Roster",
            Self::Recruiting  => "Recruiting",
            Self::Schedule    => "Schedule",
            Self::Tactics     => "Tactics",
            Self::Staff       => "Staff",
            Self::Standings   => "Standings",
            Self::Rankings    => "Rankings",
            Self::Scores      => "Scores",
            Self::Portal      => "Portal",
            Self::LeagueNews  => "League News",
        }
    }

    /// Sub-navigation items shown in the sidebar when this nav is active.
    pub fn sub_items(&self) -> &'static [&'static str] {
        match self {
            Self::Roster     => &["By Position", "Depth Chart", "Stats", "Transfers"],
            Self::Recruiting => &["Board", "Pipeline", "Offers", "Visits", "Portal"],
            Self::Schedule   => &["Upcoming", "Results", "Full Schedule"],
            Self::Tactics    => &["Offense", "Defense", "Practice", "Rotations"],
            Self::Staff      => &["Coaching Staff", "Support Staff", "Contracts"],
            Self::Standings  => &["Conference", "Overall", "RPI"],
            Self::Rankings   => &["AP Poll", "Coaches Poll", "NET"],
            Self::Scores     => &["Today", "This Week", "By Conference"],
            Self::Portal     => &["Available", "My Offers", "Committed"],
            Self::LeagueNews => &["Headlines", "Program News", "Recruiting News"],
        }
    }
}

/// The physical scale factor that corresponds to the "100%" display label.
/// Calibrated so the UI feels right at normal desktop DPI.
pub const BASE_SCALE: f32 = 1.15;

/// Percentage steps relative to `BASE_SCALE` shown in the UI.
/// "100%" means BASE_SCALE; steps are clean round numbers for readability.
pub const SCALE_STEPS_PCT: &[u32] = &[70, 80, 90, 100, 110, 125, 150];

/// Default index into `SCALE_STEPS_PCT` (the "100%" entry).
pub const DEFAULT_SCALE_IDX: usize = 3;

/// Returns the raw iced scale factor for a given step index.
pub fn scale_factor(idx: usize) -> f32 {
    BASE_SCALE * (SCALE_STEPS_PCT[idx] as f32 / 100.0)
}

/// User-configurable in-game settings (does not include UI scale, which is app-level).
#[derive(Debug, Clone)]
pub struct Settings {
    pub ticker_visible: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self { ticker_visible: true }
    }
}

/// All state for an active game session beyond the serialized `GameState`.
pub struct InGameState {
    pub game: GameState,
    pub league: League,

    /// The currently active top-bar nav item. `None` = no section open.
    pub active_nav: Option<NavItem>,
    /// Index into `active_nav.sub_items()` for the active sidebar sub-item.
    pub active_sub: Option<usize>,

    // Expand / popup open flags
    pub schedule_open: bool,
    pub advance_open: bool,
    pub game_menu_open: bool,

    pub settings: Settings,
}

impl InGameState {
    pub fn new(game: GameState, league: League) -> Self {
        Self {
            game,
            league,
            active_nav: None,
            active_sub: None,
            schedule_open: false,
            advance_open: false,
            game_menu_open: false,
            settings: Settings::default(),
        }
    }
}

// ---------------------------------------------------------------------------
// Logo resolution
// ---------------------------------------------------------------------------

/// Looks up a logo file for a school, preferring SVG over PNG and the full
/// name over the abbreviation. Returns the first path that exists on disk.
///
/// Convention: `data/logos/{name}.svg` → `{abbreviation}.svg`
///             → `{name}.png`          → `{abbreviation}.png`
pub fn find_logo(name: &str, abbreviation: &str) -> Option<PathBuf> {
    let base = Path::new("data/logos");
    for stem in [name, abbreviation] {
        for ext in ["svg", "png"] {
            let path = base.join(format!("{stem}.{ext}"));
            if path.exists() {
                return Some(path);
            }
        }
    }
    None
}

// ---------------------------------------------------------------------------
// League Builder
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeagueBuilderSection {
    League,
    Conferences,
    Tournaments,
    Teams,
    Rosters,
}

impl LeagueBuilderSection {
    pub fn label(&self) -> &'static str {
        match self {
            Self::League      => "League",
            Self::Conferences => "Conferences",
            Self::Tournaments => "Tournaments",
            Self::Teams       => "Teams",
            Self::Rosters     => "Rosters",
        }
    }
}

pub const LEAGUE_BUILDER_SECTIONS: &[LeagueBuilderSection] = &[
    LeagueBuilderSection::League,
    LeagueBuilderSection::Conferences,
    LeagueBuilderSection::Tournaments,
    LeagueBuilderSection::Teams,
    LeagueBuilderSection::Rosters,
];

// ---------------------------------------------------------------------------
// File Picker
// ---------------------------------------------------------------------------

/// Which asset import triggered the file picker, so the confirm handler
/// knows what message to emit with the chosen path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilePickerContext {
    SchoolLogo(u32),
    SchoolCourt(u32),
    ConfLogo(u32),
    TourneyLogo(u32),
    ExportLeague,
    ImportLeague,
}

/// The file extension filter to apply in the picker UI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilePickerFilter {
    /// Show only `.svg` and `.png` files.
    Image,
    /// Show only `.json` files.
    Json,
    /// No filter — show all files.
    All,
}

impl FilePickerFilter {
    pub fn accepts(&self, name: &str) -> bool {
        let lower = name.to_lowercase();
        match self {
            Self::Image => lower.ends_with(".svg") || lower.ends_with(".png"),
            Self::Json  => lower.ends_with(".json"),
            Self::All   => true,
        }
    }
}

/// Whether the picker is opening a file or saving one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilePickerMode {
    Open,
    /// Save mode: the user can also type a filename.
    Save,
}

/// All state for the iced-native file browser overlay.
pub struct FilePickerState {
    pub context: FilePickerContext,
    pub mode: FilePickerMode,
    pub filter: FilePickerFilter,
    /// The directory currently displayed.
    pub current_dir: PathBuf,
    /// Sorted directory entries: `(is_dir, name)`.
    pub entries: Vec<(bool, String)>,
    /// The filename typed by the user (save mode only).
    pub save_name: String,
    /// The file currently highlighted/previewed (open mode) or selected (save mode).
    pub selected_file: Option<PathBuf>,
    /// Whether the file browser is in grid view (`true`) or list view (`false`).
    pub grid_view: bool,
}

impl FilePickerState {
    pub fn open(context: FilePickerContext, filter: FilePickerFilter, start_dir: PathBuf) -> Self {
        let mut s = Self {
            context,
            mode: FilePickerMode::Open,
            filter,
            current_dir: start_dir,
            entries: Vec::new(),
            save_name: String::new(),
            selected_file: None,
            grid_view: false,
        };
        s.refresh();
        s
    }

    pub fn save(context: FilePickerContext, filter: FilePickerFilter, start_dir: PathBuf, default_name: &str) -> Self {
        let mut s = Self {
            context,
            mode: FilePickerMode::Save,
            filter,
            current_dir: start_dir,
            entries: Vec::new(),
            save_name: default_name.to_string(),
            selected_file: None,
            grid_view: false,
        };
        s.refresh();
        s
    }

    /// Returns true if the selected file is an image (SVG or PNG).
    pub fn selected_is_image(&self) -> bool {
        self.selected_file.as_ref().is_some_and(|p| {
            let n = p.to_string_lossy().to_lowercase();
            n.ends_with(".svg") || n.ends_with(".png")
        })
    }

    /// Re-reads `current_dir` and rebuilds `entries`. Clears any selection.
    pub fn refresh(&mut self) {
        self.entries.clear();
        self.selected_file = None;
        let Ok(rd) = std::fs::read_dir(&self.current_dir) else { return };
        let mut dirs: Vec<String> = Vec::new();
        let mut files: Vec<String> = Vec::new();
        for entry in rd.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.starts_with('.') { continue; }
            let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
            if is_dir {
                dirs.push(name);
            } else if self.filter.accepts(&name) {
                files.push(name);
            }
        }
        dirs.sort();
        files.sort();
        for d in dirs  { self.entries.push((true,  d)); }
        for f in files { self.entries.push((false, f)); }
    }
}

// ---------------------------------------------------------------------------

/// Which rival slot a searchable picker is filling in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RivalPickerContext {
    /// The user is choosing (or replacing) the primary rival.
    Primary,
    /// The user is adding a school to the general rivals list.
    AddRival,
}

pub struct LeagueBuilderState {
    pub active_section: LeagueBuilderSection,
    pub league: League,

    // Selection state
    pub selected_conf_id: Option<u32>,
    pub selected_tourney_id: Option<u32>,
    pub selected_team_id: Option<u32>,

    /// Which rival picker is currently open, if any.
    pub rival_picker: Option<RivalPickerContext>,
    /// Current search text inside the rival picker.
    pub rival_search: String,

    /// The iced-native file browser overlay, open when `Some`.
    pub file_picker: Option<FilePickerState>,

    /// In-memory cache of logo image handles, keyed by the full on-disk path.
    ///
    /// Populated on logo import so that re-imported logos hot-reload immediately.
    /// Stores `Handle` (not raw bytes) so the same Arc-backed handle is returned
    /// on every render frame — iced keys its GPU texture cache by Arc pointer, so
    /// a stable handle means zero re-uploads between frames.
    pub logo_cache: HashMap<PathBuf, ImageHandle>,

    /// Session-persistent thumbnail cache for the file picker.
    ///
    /// Keyed by absolute path. Stores `Handle` objects (which wrap an `Arc`
    /// internally) rather than raw bytes. Cloning a `Handle` is O(1) — it just
    /// increments a reference count — and iced uses the Arc pointer as the GPU
    /// texture cache key, so each image is decoded and uploaded at most once per
    /// session regardless of how many times the picker is opened or the
    /// directory is revisited.
    pub thumbnail_cache: HashMap<PathBuf, ImageHandle>,

    /// True when the league data has been modified since last save.
    pub dirty: bool,
}

impl LeagueBuilderState {

    pub fn new(league: League) -> Self {
        Self {
            active_section: LeagueBuilderSection::League,
            league,
            selected_conf_id: None,
            selected_tourney_id: None,
            selected_team_id: None,
            rival_picker: None,
            rival_search: String::new(),
            file_picker: None,
            logo_cache: HashMap::new(),
            thumbnail_cache: HashMap::new(),
            dirty: false,
        }
    }
}
