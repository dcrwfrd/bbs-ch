use std::collections::HashMap;
use std::path::PathBuf;

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::widget::container::Style as CStyle;
use iced::widget::image::{FilterMethod, Handle as ImageHandle};
use iced::widget::Id as WidgetId;
use iced::{Background, Border, Color, Element, Fill, FillPortion, Task};

use super::theme;

use bbs_ch::model::conference::Conference;
use bbs_ch::model::school::School;
use bbs_ch::model::tournament::Tournament;

use super::{
    find_logo, hex_to_color, FilePickerContext, FilePickerFilter, FilePickerMode,
    FilePickerState, LeagueBuilderSection, LeagueBuilderState, RivalPickerContext,
    LEAGUE_BUILDER_SECTIONS,
};
use crate::Message;

// ---------------------------------------------------------------------------
// Which prestige component is being edited (lives here, used only by LB).
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrestigeField {
    CumulativeSuccess,
    RecentSuccess,
    Facilities,
    Resources,
    Atmosphere,
    CoachingStaff,
    Market,
    MediaPerception,
    Academics,
}

// ---------------------------------------------------------------------------
// Update — all Lb* message handling extracted from main.rs
// ---------------------------------------------------------------------------

/// Handles every `Message::Lb*` variant (and `LbExportLeague`/`LbImportLeague`).
///
/// `last_dir` is `App::last_asset_dir` — passed mutably so the file picker
/// can update it without the handler needing access to the full `App`.
pub fn update(
    s: &mut LeagueBuilderState,
    last_dir: &mut Option<PathBuf>,
    last_grid_view: &mut bool,
    msg: Message,
) -> Task<Message> {
    match msg {
        // ---------------------------------------------------------------
        // Navigation
        // ---------------------------------------------------------------
        Message::LeagueBuilderNav(section) => {
            s.active_section = section;
        }

        // ---------------------------------------------------------------
        // Conference editor
        // ---------------------------------------------------------------
        Message::LbConferenceSelect(id) => { s.selected_conf_id = Some(id); }
        Message::LbConfName(v) => {
            if let Some(id) = s.selected_conf_id {
                if let Some(c) = s.league.conference_by_id_mut(id) { c.name = v; s.dirty = true; }
            }
        }
        Message::LbConfAbbr(v) => {
            if let Some(id) = s.selected_conf_id {
                if let Some(c) = s.league.conference_by_id_mut(id) { c.abbreviation = v; s.dirty = true; }
            }
        }
        // Region/tier are enums; dropdowns TBD.
        Message::LbConfRegion(_) | Message::LbConfTier(_) => {}

        // ---------------------------------------------------------------
        // Tournament editor
        // ---------------------------------------------------------------
        Message::LbTournamentSelect(id) => { s.selected_tourney_id = Some(id); }
        Message::LbTourneyName(v) => {
            if let Some(id) = s.selected_tourney_id {
                if let Some(t) = s.league.tournament_by_id_mut(id) { t.name = v; s.dirty = true; }
            }
        }
        Message::LbTourneyAbbr(v) => {
            if let Some(id) = s.selected_tourney_id {
                if let Some(t) = s.league.tournament_by_id_mut(id) { t.abbreviation = v; s.dirty = true; }
            }
        }
        Message::LbTourneyFieldSize(v) => {
            if let (Some(id), Ok(n)) = (s.selected_tourney_id, v.parse::<u32>()) {
                if let Some(t) = s.league.tournament_by_id_mut(id) { t.field_size = n; s.dirty = true; }
            }
        }
        Message::LbTourneyHostSite(v) => {
            if let Some(id) = s.selected_tourney_id {
                if let Some(t) = s.league.tournament_by_id_mut(id) { t.host_site = v; s.dirty = true; }
            }
        }
        Message::LbTourneyAutoBids(v) => {
            if let (Some(id), Ok(n)) = (s.selected_tourney_id, v.parse::<u32>()) {
                if let Some(t) = s.league.tournament_by_id_mut(id) { t.automatic_bids = n; s.dirty = true; }
            }
        }
        Message::LbTourneyAtLargeBids(v) => {
            if let (Some(id), Ok(n)) = (s.selected_tourney_id, v.parse::<u32>()) {
                if let Some(t) = s.league.tournament_by_id_mut(id) { t.at_large_bids = n; s.dirty = true; }
            }
        }
        // Enum fields — dropdowns TBD.
        Message::LbTourneyKind(_) | Message::LbTourneyFormat(_) => {}

        // ---------------------------------------------------------------
        // Team editor
        // ---------------------------------------------------------------
        Message::LbTeamSelect(id) => { s.selected_team_id = Some(id); }
        Message::LbTeamName(v) => {
            if let Some(id) = s.selected_team_id {
                if let Some(sc) = s.league.school_by_id_mut(id) { sc.name = v; s.dirty = true; }
            }
        }
        Message::LbTeamNickname(v) => {
            if let Some(id) = s.selected_team_id {
                if let Some(sc) = s.league.school_by_id_mut(id) { sc.nickname = v; s.dirty = true; }
            }
        }
        Message::LbTeamAbbr(v) => {
            if let Some(id) = s.selected_team_id {
                if let Some(sc) = s.league.school_by_id_mut(id) { sc.abbreviation = v; s.dirty = true; }
            }
        }
        Message::LbTeamPreferAbbr(v) => {
            if let Some(id) = s.selected_team_id {
                if let Some(sc) = s.league.school_by_id_mut(id) { sc.prefer_abbreviation = v; s.dirty = true; }
            }
        }
        Message::LbTeamZip(v) => {
            if let Some(id) = s.selected_team_id {
                if let Some(sc) = s.league.school_by_id_mut(id) { sc.zip_code = v; s.dirty = true; }
            }
        }
        Message::LbTeamConference(v) => {
            if let Some(id) = s.selected_team_id {
                if let Some(sc) = s.league.school_by_id_mut(id) { sc.conference = v; s.dirty = true; }
            }
        }
        Message::LbTeamPrimaryColor(v) => {
            if let Some(id) = s.selected_team_id {
                if let Some(sc) = s.league.school_by_id_mut(id) { sc.primary_color = v; s.dirty = true; }
            }
        }
        Message::LbTeamSecondaryColor(v) => {
            if let Some(id) = s.selected_team_id {
                if let Some(sc) = s.league.school_by_id_mut(id) { sc.secondary_color = v; s.dirty = true; }
            }
        }
        Message::LbTeamCourtName(v) => {
            if let Some(id) = s.selected_team_id {
                if let Some(sc) = s.league.school_by_id_mut(id) { sc.home_court.name = v; s.dirty = true; }
            }
        }
        Message::LbTeamCourtCapacity(v) => {
            if let (Some(id), Ok(n)) = (s.selected_team_id, v.parse::<u32>()) {
                if let Some(sc) = s.league.school_by_id_mut(id) { sc.home_court.capacity = n; s.dirty = true; }
            }
        }
        Message::LbTeamCourtAtmosphere(v) => {
            if let (Some(id), Ok(n)) = (s.selected_team_id, v.parse::<u8>()) {
                if let Some(sc) = s.league.school_by_id_mut(id) {
                    sc.home_court.atmosphere = n.min(99);
                    s.dirty = true;
                }
            }
        }
        Message::LbTeamEnrollment(v) => {
            if let (Some(id), Ok(n)) = (s.selected_team_id, v.parse::<u32>()) {
                if let Some(sc) = s.league.school_by_id_mut(id) { sc.enrollment = n; s.dirty = true; }
            }
        }
        Message::LbTeamNilBudget(v) => {
            if let (Some(id), Ok(n)) = (s.selected_team_id, v.parse::<u32>()) {
                if let Some(sc) = s.league.school_by_id_mut(id) { sc.nil_budget = n; s.dirty = true; }
            }
        }
        Message::LbTeamPrestige(field, v) => {
            if let (Some(id), Ok(n)) = (s.selected_team_id, v.parse::<u8>()) {
                let n = n.min(99);
                if let Some(sc) = s.league.school_by_id_mut(id) {
                    let p = &mut sc.prestige;
                    match field {
                        PrestigeField::CumulativeSuccess => p.cumulative_success = n,
                        PrestigeField::RecentSuccess     => p.recent_success = n,
                        PrestigeField::Facilities        => p.facilities = n,
                        PrestigeField::Resources         => p.resources = n,
                        PrestigeField::Atmosphere        => p.atmosphere = n,
                        PrestigeField::CoachingStaff     => p.coaching_staff = n,
                        PrestigeField::Market            => p.market = n,
                        PrestigeField::MediaPerception   => p.media_perception = n,
                        PrestigeField::Academics         => p.academics = n,
                    }
                    s.dirty = true;
                }
            }
        }

        // ---------------------------------------------------------------
        // Add / remove items
        // ---------------------------------------------------------------
        Message::LbAddConference => {
            let new_id = s.league.conferences.iter().map(|c| c.id).max().unwrap_or(0) + 1;
            s.league.conferences.push(Conference {
                id: new_id,
                name: "New Conference".to_string(),
                abbreviation: "NEW".to_string(),
                region: bbs_ch::model::conference::Region::Midwest,
                tier: bbs_ch::model::prestige::ConferenceTier::LowMajor,
                members: vec![],
                member_ids: vec![],
                logo: None,
            });
            s.selected_conf_id = Some(new_id);
            s.dirty = true;
        }
        Message::LbRemoveConference(id) => {
            s.league.conferences.retain(|c| c.id != id);
            if s.selected_conf_id == Some(id) { s.selected_conf_id = None; }
            s.dirty = true;
        }
        Message::LbAddTournament => {
            let new_id = s.league.tournaments.iter().map(|t| t.id).max().unwrap_or(0) + 1;
            s.league.tournaments.push(Tournament {
                id: new_id,
                name: "New Tournament".to_string(),
                abbreviation: "NEW".to_string(),
                logo: None,
                kind: bbs_ch::model::tournament::TournamentKind::InSeason,
                bracket_format: bbs_ch::model::tournament::BracketFormat::SingleElimination,
                field_size: 8,
                host_site: String::new(),
                conference: None,
                automatic_bids: 0,
                at_large_bids: 8,
            });
            s.selected_tourney_id = Some(new_id);
            s.dirty = true;
        }
        Message::LbRemoveTournament(id) => {
            s.league.tournaments.retain(|t| t.id != id);
            if s.selected_tourney_id == Some(id) { s.selected_tourney_id = None; }
            s.dirty = true;
        }
        Message::LbAddTeam => {
            let new_id = s.league.schools.iter().map(|sc| sc.id).max().unwrap_or(0) + 1;
            s.league.schools.push(School {
                id: new_id,
                name: "New School".to_string(),
                nickname: "Wildcats".to_string(),
                abbreviation: "NEW".to_string(),
                prefer_abbreviation: false,
                zip_code: String::new(),
                conference: String::new(),
                conference_id: 0,
                primary_color: "#333366".to_string(),
                secondary_color: "#ffffff".to_string(),
                home_court: bbs_ch::model::school::HomeCourt {
                    name: "Home Arena".to_string(),
                    capacity: 5000,
                    atmosphere: 50,
                },
                prestige: bbs_ch::model::prestige::ProgramPrestige {
                    cumulative_success: 50,
                    recent_success: 50,
                    facilities: 50,
                    resources: 50,
                    atmosphere: 50,
                    coaching_staff: 50,
                    market: 50,
                    media_perception: 50,
                    academics: 50,
                },
                enrollment: 0,
                nil_budget: 1_000_000,
                primary_rival: None,
                rivals: Vec::new(),
            });
            s.selected_team_id = Some(new_id);
            s.dirty = true;
        }
        Message::LbRemoveTeam(id) => {
            s.league.schools.retain(|sc| sc.id != id);
            if s.selected_team_id == Some(id) { s.selected_team_id = None; }
            s.dirty = true;
        }

        // ---------------------------------------------------------------
        // File picker
        // ---------------------------------------------------------------
        Message::LbFilePickerOpen(ctx) => {
            let start = last_dir.clone()
                .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
            let mut picker = match &ctx {
                FilePickerContext::SchoolLogo(_)
                | FilePickerContext::ConfLogo(_)
                | FilePickerContext::TourneyLogo(_)
                | FilePickerContext::SchoolCourt(_) =>
                    FilePickerState::open(ctx, FilePickerFilter::Image, start),
                FilePickerContext::ImportLeague =>
                    FilePickerState::open(ctx, FilePickerFilter::Json, start),
                FilePickerContext::ExportLeague =>
                    FilePickerState::save(ctx, FilePickerFilter::Json, start, "my_league.json"),
            };
            let dir = picker.current_dir.clone();
            picker.grid_view = *last_grid_view;
            s.file_picker = Some(picker);
            return spawn_thumbnails(&dir, &s.thumbnail_cache);
        }
        Message::LbFilePickerNavigate(dir) => {
            *last_dir = Some(dir.clone());
            if let Some(fp) = s.file_picker.as_mut() {
                fp.current_dir = dir.clone();
                fp.refresh();
            }
            return spawn_thumbnails(&dir, &s.thumbnail_cache);
        }
        Message::LbThumbnailReady(path, w, h, pixels) => {
            s.thumbnail_cache.insert(path, ImageHandle::from_rgba(w, h, pixels));
        }
        Message::LbFilePickerToggleView => {
            if let Some(fp) = s.file_picker.as_mut() {
                fp.grid_view = !fp.grid_view;
                *last_grid_view = fp.grid_view;
            }
        }
        Message::LbFilePickerSelect(path) => {
            if let Some(fp) = s.file_picker.as_mut() {
                if fp.mode == FilePickerMode::Save {
                    if let Some(name) = path.file_name() {
                        fp.save_name = name.to_string_lossy().into_owned();
                    }
                }
                fp.selected_file = Some(path);
            }
        }
        Message::LbFilePickerSaveName(name) => {
            if let Some(fp) = s.file_picker.as_mut() {
                fp.save_name = name;
            }
        }
        Message::LbFilePickerCancel => {
            s.file_picker = None;
        }
        Message::LbFilePickerConfirm(src) => {
            *last_dir = src.parent().map(|p| p.to_path_buf());
            let ctx = s.file_picker.take().map(|fp| fp.context);
            if let Some(ctx) = ctx {
                return update_file_confirm(s, ctx, src);
            }
        }

        // ---------------------------------------------------------------
        // Rivalries
        // ---------------------------------------------------------------
        Message::LbRivalPickerOpen(ctx) => {
            s.rival_picker = Some(ctx);
            s.rival_search = String::new();
        }
        Message::LbRivalPickerClose => {
            s.rival_picker = None;
            s.rival_search = String::new();
        }
        Message::LbRivalSearchChanged(q) => { s.rival_search = q; }
        Message::LbTeamSetPrimaryRival(new_rival_abbr) => {
            if let Some(id) = s.selected_team_id {
                let (my_abbr, old_primary) = match s.league.school_by_id(id) {
                    Some(sc) => (sc.abbreviation.clone(), sc.primary_rival.clone()),
                    None => { s.rival_picker = None; return Task::none(); }
                };
                for sc in &mut s.league.schools {
                    if sc.id == id {
                        sc.primary_rival = Some(new_rival_abbr.clone());
                    } else if sc.abbreviation == new_rival_abbr {
                        sc.primary_rival = Some(my_abbr.clone());
                    } else if old_primary.as_deref() == Some(sc.abbreviation.as_str())
                        && sc.primary_rival.as_deref() == Some(my_abbr.as_str())
                    {
                        sc.primary_rival = None;
                    }
                }
                s.dirty = true;
            }
            s.rival_picker = None;
            s.rival_search = String::new();
        }
        Message::LbTeamClearPrimaryRival => {
            if let Some(id) = s.selected_team_id {
                let (my_abbr, old_primary) = match s.league.school_by_id(id) {
                    Some(sc) => (sc.abbreviation.clone(), sc.primary_rival.clone()),
                    None => return Task::none(),
                };
                for sc in &mut s.league.schools {
                    if sc.id == id {
                        sc.primary_rival = None;
                    } else if old_primary.as_deref() == Some(sc.abbreviation.as_str())
                        && sc.primary_rival.as_deref() == Some(my_abbr.as_str())
                    {
                        sc.primary_rival = None;
                    }
                }
                s.dirty = true;
            }
        }
        Message::LbTeamAddRival(abbr) => {
            if let Some(id) = s.selected_team_id {
                if let Some(sc) = s.league.school_by_id_mut(id) {
                    if !sc.rivals.contains(&abbr) {
                        sc.rivals.push(abbr);
                        s.dirty = true;
                    }
                }
            }
            s.rival_picker = None;
            s.rival_search = String::new();
        }
        Message::LbTeamRemoveRival(abbr) => {
            if let Some(id) = s.selected_team_id {
                if let Some(sc) = s.league.school_by_id_mut(id) {
                    sc.rivals.retain(|r| r != &abbr);
                    s.dirty = true;
                }
            }
        }

        // ---------------------------------------------------------------
        // Delete logos
        // ---------------------------------------------------------------
        Message::LbDeleteSchoolLogo(school_id) => {
            if let Some(school) = s.league.school_by_id(school_id) {
                if let Some(p) = find_logo(&school.name, &school.abbreviation) {
                    let _ = std::fs::remove_file(&p);
                    s.logo_cache.remove(&p);
                }
            }
        }
        Message::LbDeleteConfLogo(conf_id) => {
            let rel = s.league.conferences.iter()
                .find(|c| c.id == conf_id)
                .and_then(|c| c.logo.clone());
            if let Some(rel) = rel {
                let path = PathBuf::from("data").join(&rel);
                let _ = std::fs::remove_file(&path);
                s.logo_cache.remove(&path);
                if let Some(conf) = s.league.conferences.iter_mut().find(|c| c.id == conf_id) {
                    conf.logo = None;
                    s.dirty = true;
                }
            }
        }
        Message::LbDeleteTourneyLogo(tourney_id) => {
            let rel = s.league.tournaments.iter()
                .find(|t| t.id == tourney_id)
                .and_then(|t| t.logo.clone());
            if let Some(rel) = rel {
                let path = PathBuf::from("data").join(&rel);
                let _ = std::fs::remove_file(&path);
                s.logo_cache.remove(&path);
                if let Some(t) = s.league.tournaments.iter_mut().find(|t| t.id == tourney_id) {
                    t.logo = None;
                    s.dirty = true;
                }
            }
        }

        // ---------------------------------------------------------------
        // League I/O
        // ---------------------------------------------------------------
        Message::LbSaveLeague => {
            let _ = std::fs::create_dir_all("data");
            match bbs_ch::storage::db::LeagueDb::open("data/league.db") {
                Ok(mut db) => {
                    match db.save_league(&s.league) {
                        Ok(()) => { s.dirty = false; }
                        Err(e) => eprintln!("Failed to save league: {e:#}"),
                    }
                }
                Err(e) => eprintln!("Failed to open league DB: {e:#}"),
            }
        }
        Message::LbExportLeague => {
            return update(s, last_dir, last_grid_view, Message::LbFilePickerOpen(FilePickerContext::ExportLeague));
        }
        Message::LbImportLeague => {
            return update(s, last_dir, last_grid_view, Message::LbFilePickerOpen(FilePickerContext::ImportLeague));
        }

        _ => {} // non-LB messages are handled by App::update in main.rs
    }
    Task::none()
}

/// Inner handler for `LbFilePickerConfirm` — split out to keep `update` readable.
fn update_file_confirm(s: &mut LeagueBuilderState, ctx: FilePickerContext, src: PathBuf) -> Task<Message> {
    match ctx {
        FilePickerContext::SchoolLogo(school_id) => {
            let school_name = s.league.school_by_id(school_id).map(|sc| sc.name.clone());
            if let Some(name) = school_name {
                let ext = src.extension().and_then(|e| e.to_str()).unwrap_or("png");
                let _ = std::fs::create_dir_all("data/logos");
                let dest = PathBuf::from(format!("data/logos/{name}.{ext}"));
                if std::fs::copy(&src, &dest).is_ok() {
                    if let Ok(bytes) = std::fs::read(&dest) {
                        s.logo_cache.insert(dest, ImageHandle::from_bytes(bytes));
                    }
                    s.dirty = true;
                }
            }
        }
        FilePickerContext::SchoolCourt(school_id) => {
            if let Some(school) = s.league.school_by_id(school_id) {
                let _ = std::fs::create_dir_all("data/courts");
                let dest = PathBuf::from(format!("data/courts/{}.png", school.name));
                let _ = std::fs::copy(&src, &dest);
            }
        }
        FilePickerContext::ConfLogo(conf_id) => {
            let conf_abbr = s.league.conferences.iter()
                .find(|c| c.id == conf_id)
                .map(|c| c.abbreviation.clone());
            if let Some(abbr) = conf_abbr {
                let ext = src.extension().and_then(|e| e.to_str()).unwrap_or("png");
                let _ = std::fs::create_dir_all("data/logos");
                let rel = format!("logos/{abbr}.{ext}");
                let dest = PathBuf::from("data").join(&rel);
                if std::fs::copy(&src, &dest).is_ok() {
                    if let Ok(bytes) = std::fs::read(&dest) {
                        s.logo_cache.insert(dest, ImageHandle::from_bytes(bytes));
                    }
                    if let Some(conf) = s.league.conferences.iter_mut().find(|c| c.id == conf_id) {
                        conf.logo = Some(rel);
                    }
                    s.dirty = true;
                }
            }
        }
        FilePickerContext::TourneyLogo(tourney_id) => {
            let tourney_abbr = s.league.tournaments.iter()
                .find(|t| t.id == tourney_id)
                .map(|t| t.abbreviation.clone());
            if let Some(abbr) = tourney_abbr {
                let ext = src.extension().and_then(|e| e.to_str()).unwrap_or("png");
                let _ = std::fs::create_dir_all("data/logos");
                let rel = format!("logos/{abbr}.{ext}");
                let dest = PathBuf::from("data").join(&rel);
                if std::fs::copy(&src, &dest).is_ok() {
                    if let Ok(bytes) = std::fs::read(&dest) {
                        s.logo_cache.insert(dest, ImageHandle::from_bytes(bytes));
                    }
                    if let Some(t) = s.league.tournaments.iter_mut().find(|t| t.id == tourney_id) {
                        t.logo = Some(rel);
                    }
                    s.dirty = true;
                }
            }
        }
        FilePickerContext::ExportLeague => {
            #[derive(serde::Serialize)]
            struct LeagueExport<'a> {
                schools: &'a Vec<School>,
                conferences: &'a Vec<Conference>,
                tournaments: &'a Vec<Tournament>,
            }
            let export = LeagueExport {
                schools: &s.league.schools,
                conferences: &s.league.conferences,
                tournaments: &s.league.tournaments,
            };
            if let Ok(json) = serde_json::to_string_pretty(&export) {
                let _ = std::fs::write(&src, json);
            }
        }
        FilePickerContext::ImportLeague => {
            if let Ok(json) = std::fs::read_to_string(&src) {
                #[derive(serde::Deserialize)]
                struct LeagueImport {
                    schools: Vec<School>,
                    conferences: Vec<Conference>,
                    tournaments: Vec<Tournament>,
                }
                if let Ok(imp) = serde_json::from_str::<LeagueImport>(&json) {
                    s.league.schools = imp.schools;
                    s.league.conferences = imp.conferences;
                    s.league.tournaments = imp.tournaments;
                    s.dirty = true;
                }
            }
        }
    }
    Task::none()
}

// ---------------------------------------------------------------------------
// Background thumbnail loader
// ---------------------------------------------------------------------------

/// Spawns one async task per uncached PNG in `dir`.
///
/// Each task decodes and downscales the image to 32×32 RGBA off the main
/// thread, then sends `Message::LbThumbnailReady` when done. The file picker
/// list renders a placeholder until the handle arrives.
fn spawn_thumbnails(
    dir: &std::path::Path,
    existing: &HashMap<std::path::PathBuf, ImageHandle>,
) -> Task<Message> {
    let Ok(rd) = std::fs::read_dir(dir) else { return Task::none() };
    let tasks: Vec<Task<Message>> = rd
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            if !path.to_string_lossy().to_lowercase().ends_with(".png") { return None; }
            if existing.contains_key(&path) { return None; }
            Some(Task::perform(
                async move {
                    let bytes = std::fs::read(&path).ok()?;
                    let img = image::load_from_memory(&bytes).ok()?;
                    let thumb = img.thumbnail(96, 96).to_rgba8();
                    let (w, h) = thumb.dimensions();
                    Some((path, w, h, thumb.into_raw()))
                },
                |r| match r {
                    Some((path, w, h, pixels)) => Message::LbThumbnailReady(path, w, h, pixels),
                    None => Message::Noop,
                },
            ))
        })
        .collect();
    Task::batch(tasks)
}

// ---------------------------------------------------------------------------
// Palette aliases — local shorthand for frequently-used theme tokens.
// ---------------------------------------------------------------------------

use theme::{
    BG_DIVIDER as DIVIDER_BG, BG_HEADER as HEADER_BG, BG_PANEL as PANEL_BG,
    BG_PICKER as PICKER_BG, BG_SIDEBAR as SIDEBAR_BG,
    ACCENT_WARN, ACCENT_SUCCESS as ACCENT_GREEN, ACCENT_DANGER as ACCENT_RED,
    TEXT_DIM as LABEL_DIM,
};

/// Renders a logo thumbnail from a data/-relative path (e.g. `"logos/ACC.svg"`).
///
/// For PNG files the cache is checked first so freshly-imported logos display
/// immediately without waiting for iced's texture cache to clear. SVGs are
/// always loaded from disk (they render fine without caching). Falls back to
/// a colored placeholder square when the path is `None` or missing on disk.
fn logo_thumb(
    rel_path: Option<&str>,
    size: u32,
    fallback_color: Color,
    cache: &HashMap<PathBuf, ImageHandle>,
) -> Element<'static, Message> {
    if let Some(rel) = rel_path {
        let full = PathBuf::from("data").join(rel);
        if full.exists() {
            return if rel.ends_with(".svg") {
                iced::widget::svg(iced::widget::svg::Handle::from_path(full))
                    .width(size).height(size).into()
            } else {
                let handle = cache.get(&full).cloned()
                    .unwrap_or_else(|| ImageHandle::from_path(full));
                iced::widget::image(handle)
                    .filter_method(FilterMethod::Linear)
                    .width(size).height(size).into()
            };
        }
    }
    placeholder_swatch(size, fallback_color, 8.0)
}

/// Renders a logo thumbnail for a school, using `find_logo` to discover the
/// file and the cache for hot-reload on PNG images.
fn school_logo_thumb(
    school: &bbs_ch::model::school::School,
    size: u32,
    cache: &HashMap<PathBuf, ImageHandle>,
) -> Element<'static, Message> {
    let fallback = super::hex_to_color(&school.primary_color);
    match find_logo(&school.name, &school.abbreviation) {
        Some(path) if path.extension().is_some_and(|e| e == "svg") => {
            iced::widget::svg(iced::widget::svg::Handle::from_path(path))
                .width(size).height(size).into()
        }
        Some(path) => {
            let handle = cache.get(&path).cloned()
                .unwrap_or_else(|| ImageHandle::from_path(path));
            iced::widget::image(handle)
                .filter_method(FilterMethod::Linear)
                .width(size).height(size).into()
        }
        None => placeholder_swatch(size, fallback, 4.0),
    }
}

/// A solid-color rounded square used as a logo placeholder.
fn placeholder_swatch(size: u32, color: Color, radius: f32) -> Element<'static, Message> {
    container(Space::new().width(size).height(size))
        .style(move |_| CStyle {
            background: Some(Background::Color(color)),
            border: Border { radius: radius.into(), ..Default::default() },
            ..Default::default()
        })
        .into()
}

fn bg(c: Color) -> impl Fn(&iced::Theme) -> CStyle {
    theme::bg(c)
}

fn row_bg(i: usize) -> impl Fn(&iced::Theme) -> CStyle {
    theme::row_alt(i)
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

pub fn view(state: &LeagueBuilderState) -> Element<'_, Message> {
    let base = column![
        header(state),
        tab_bar(state),
        content(state),
    ]
    .height(Fill);

    // Foreground card — same palette as the main-menu / new-game cards.
    let card = container(base)
        .width(Fill)
        .height(Fill)
        .style(theme::card_style);

    // Outer shell — darker backdrop with equal margins on all sides.
    let shell = container(card)
        .width(Fill)
        .height(Fill)
        .padding(20)
        .style(theme::shell_style);

    match &state.file_picker {
        Some(fp) => iced::widget::stack![shell, file_picker_overlay(fp, &state.thumbnail_cache)].into(),
        None     => iced::widget::stack![shell].into(),
    }
}

// ---------------------------------------------------------------------------
// File picker overlay
// ---------------------------------------------------------------------------

fn file_picker_overlay<'a>(
    fp: &'a super::FilePickerState,
    thumb_cache: &'a HashMap<std::path::PathBuf, iced::widget::image::Handle>,
) -> Element<'a, Message> {
    use iced::widget::Grid;
    use super::FilePickerMode;

    const PANEL_W:     u32 = 1080;
    const LIST_W:      u32 = 620;
    const PREV_W:      u32 = 260;
    const PREV_H:      u32 = 420;
    const LIST_H:      u32 = 640;
    const LIST_THUMB:  u32 = 32;
    const GRID_THUMB:  u32 = 88;
    const GRID_COLS:   usize = 5;

    // ---- toolbar: up button + breadcrumb + view toggle ----
    let parent_btn = if let Some(parent) = fp.current_dir.parent() {
        let parent = parent.to_path_buf();
        button(text("↑ Up").size(13))
            .on_press(Message::LbFilePickerNavigate(parent))
            .style(button::text)
    } else {
        button(text("↑ Up").size(13)).style(button::text)
    };
    let path_str  = fp.current_dir.to_string_lossy().into_owned();
    let breadcrumb = text(path_str).size(12).color(LABEL_DIM);
    let view_label = if fp.grid_view { "☰ List" } else { "⊞ Grid" };
    let view_toggle = button(text(view_label).size(13))
        .on_press(Message::LbFilePickerToggleView)
        .style(button::text);

    let toolbar = row![parent_btn, Space::new().width(8), breadcrumb, Space::new().width(Fill), view_toggle]
        .align_y(Vertical::Center);

    // ---- build the file browser (list or grid) ----
    let browser: Element<'_, Message> = if fp.grid_view {
        // Grid view: image tiles with filename underneath.
        // Folders appear first as plain text tiles.
        let tiles: Vec<Element<'_, Message>> = fp.entries.iter().map(|(is_dir, name)| {
            let path = fp.current_dir.join(name);
            let is_selected = !is_dir && fp.selected_file.as_ref().is_some_and(|p| p == &path);
            let msg = if *is_dir {
                Message::LbFilePickerNavigate(path.clone())
            } else {
                Message::LbFilePickerSelect(path.clone())
            };

            let icon: Element<'_, Message> = if *is_dir {
                container(text("📁").size(36))
                    .width(GRID_THUMB).height(GRID_THUMB)
                    .center_x(GRID_THUMB).center_y(GRID_THUMB)
                    .into()
            } else {
                let lower = name.to_lowercase();
                if lower.ends_with(".svg") {
                    iced::widget::svg(iced::widget::svg::Handle::from_path(&path))
                        .width(GRID_THUMB).height(GRID_THUMB)
                        .into()
                } else if lower.ends_with(".png") {
                    let handle = thumb_cache.get(&path).cloned()
                        .unwrap_or_else(|| iced::widget::image::Handle::from_path(&path));
                    iced::widget::image(handle)
                        .filter_method(FilterMethod::Linear)
                        .width(GRID_THUMB).height(GRID_THUMB)
                        .into()
                } else {
                    Space::new().width(GRID_THUMB).height(GRID_THUMB).into()
                }
            };

            // Truncate long filenames to fit the tile width.
            let display = if name.len() > 14 {
                format!("{}…", &name[..13])
            } else {
                name.clone()
            };

            let tile_body = column![
                icon,
                Space::new().height(4),
                text(display).size(11).align_x(Horizontal::Center),
            ]
            .align_x(Horizontal::Center)
            .spacing(0);

            let tile_bg = if is_selected {
                theme::BG_SELECTED
            } else {
                Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }
            };

            container(
                button(tile_body)
                    .on_press(msg)
                    .style(button::text),
            )
            .style(move |_| CStyle {
                background: Some(Background::Color(tile_bg)),
                border: Border { radius: 6.0.into(), ..Default::default() },
                ..Default::default()
            })
            .padding(6)
            .into()
        }).collect();

        scrollable(
            Grid::with_children(tiles)
                .columns(GRID_COLS)
                .spacing(4),
        )
        .height(LIST_H)
        .into()
    } else {
        // List view: one row per entry with a small thumbnail.
        let entries: Vec<Element<'_, Message>> = fp.entries.iter().enumerate().map(|(i, (is_dir, name))| {
            let path = fp.current_dir.join(name);
            let is_selected = !is_dir && fp.selected_file.as_ref().is_some_and(|p| p == &path);
            let msg = if *is_dir {
                Message::LbFilePickerNavigate(path.clone())
            } else {
                Message::LbFilePickerSelect(path.clone())
            };
            let row_color = if is_selected {
                theme::BG_SELECTED
            } else if i % 2 == 0 { theme::BG_PANEL } else { theme::BG_ROW_B };

            let thumb: Element<'_, Message> = if *is_dir {
                container(text("📁").size(18))
                    .width(LIST_THUMB).height(LIST_THUMB)
                    .center_x(LIST_THUMB).center_y(LIST_THUMB)
                    .into()
            } else {
                let lower = name.to_lowercase();
                if lower.ends_with(".svg") {
                    iced::widget::svg(iced::widget::svg::Handle::from_path(&path))
                        .width(LIST_THUMB).height(LIST_THUMB)
                        .into()
                } else if lower.ends_with(".png") {
                    let handle = thumb_cache.get(&path).cloned()
                        .unwrap_or_else(|| iced::widget::image::Handle::from_path(&path));
                    iced::widget::image(handle)
                        .filter_method(FilterMethod::Linear)
                        .width(LIST_THUMB).height(LIST_THUMB)
                        .into()
                } else {
                    Space::new().width(LIST_THUMB).height(LIST_THUMB).into()
                }
            };

            container(
                button(
                    row![thumb, Space::new().width(6), text(name).size(13)]
                        .align_y(Vertical::Center),
                )
                .on_press(msg)
                .width(Fill)
                .style(button::text),
            )
            .style(move |_| CStyle {
                background: Some(Background::Color(row_color)),
                ..Default::default()
            })
            .width(Fill)
            .into()
        }).collect();

        scrollable(column(entries).width(Fill)).height(LIST_H).into()
    };

    // ---- image preview panel ----
    let preview: Element<'_, Message> = if fp.selected_is_image() {
        let path = fp.selected_file.as_ref().unwrap();
        let name = path.to_string_lossy().to_lowercase();
        if name.ends_with(".svg") {
            iced::widget::svg(iced::widget::svg::Handle::from_path(path))
                .width(PREV_W).height(PREV_H)
                .into()
        } else {
            iced::widget::image(iced::widget::image::Handle::from_path(path))
                .filter_method(FilterMethod::Linear)
                .width(PREV_W).height(PREV_H)
                .into()
        }
    } else {
        container(
            text(if fp.selected_file.is_some() { "" } else { "Select a file\nto preview" })
                .size(12)
                .color(LABEL_DIM)
                .align_x(Horizontal::Center),
        )
        .width(PREV_W).height(PREV_H)
        .center_x(PREV_W).center_y(PREV_H)
        .style(|_| CStyle {
            background: Some(Background::Color(PICKER_BG)),
            border: Border { radius: 4.0.into(), ..Default::default() },
            ..Default::default()
        })
        .into()
    };

    let preview_name: Element<'_, Message> = if let Some(p) = &fp.selected_file {
        let name = p.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
        text(name).size(11).color(LABEL_DIM).into()
    } else {
        Space::new().height(0).into()
    };

    let right_col = column![preview, Space::new().height(6), preview_name]
        .width(PREV_W)
        .align_x(Horizontal::Center);

    // ---- bottom action row ----
    let bottom: Element<'_, Message> = if fp.mode == FilePickerMode::Save {
        let confirm_path = fp.current_dir.join(&fp.save_name);
        row![
            text("File name:").size(13),
            Space::new().width(8),
            text_input("my_league.json", &fp.save_name)
                .on_input(Message::LbFilePickerSaveName)
                .size(13)
                .width(Fill),
            Space::new().width(8),
            button(text("Save").size(13))
                .on_press(Message::LbFilePickerConfirm(confirm_path))
                .style(button::primary),
            Space::new().width(6),
            button(text("Cancel").size(13))
                .on_press(Message::LbFilePickerCancel)
                .style(button::secondary),
        ]
        .align_y(Vertical::Center)
        .into()
    } else {
        let open_btn = if let Some(path) = fp.selected_file.clone() {
            button(text("Open").size(13))
                .on_press(Message::LbFilePickerConfirm(path))
                .style(button::primary)
        } else {
            button(text("Open").size(13)).style(button::primary)
        };
        row![
            Space::new().width(Fill),
            open_btn,
            Space::new().width(6),
            button(text("Cancel").size(13))
                .on_press(Message::LbFilePickerCancel)
                .style(button::secondary),
        ]
        .into()
    };

    // ---- assemble the dialog panel ----
    let panel = container(
        column![
            toolbar,
            Space::new().height(6),
            row![
                container(browser)
                    .style(|_| CStyle {
                        background: Some(Background::Color(PICKER_BG)),
                        border: Border { radius: 4.0.into(), ..Default::default() },
                        ..Default::default()
                    })
                    .width(LIST_W),
                Space::new().width(16),
                right_col,
            ],
            Space::new().height(10),
            bottom,
        ]
        .padding(16),
    )
    .width(PANEL_W)
    .style(theme::dialog_panel_style);

    // Two-layer stack:
    // - Bottom: full-screen mouse_area that closes the picker on any click.
    // - Top: the panel wrapped in a mouse_area(Noop) so clicks *inside* the
    //   dialog are consumed before reaching the close layer.
    iced::widget::stack![
        iced::widget::mouse_area(
            container(Space::new().width(Fill).height(Fill))
                .width(Fill)
                .height(Fill)
                .style(theme::overlay_style),
        )
        .on_press(Message::LbFilePickerCancel),
        iced::widget::mouse_area(
            container(panel).center_x(Fill).center_y(Fill),
        )
        .on_press(Message::Noop),
    ]
    .into()
}

// ---------------------------------------------------------------------------
// Header bar
// ---------------------------------------------------------------------------

fn header(state: &LeagueBuilderState) -> Element<'_, Message> {
    let dirty_indicator = if state.dirty {
        text("● Unsaved changes").size(12).color(ACCENT_WARN)
    } else {
        text("").size(12)
    };

    container(
        row![
            // Left: back button (fixed width so title stays centred)
            container(
                button(text("← Main Menu").size(13))
                    .on_press(Message::LeagueBuilderBack)
                    .style(button::text),
            )
            .width(FillPortion(1))
            .align_x(Horizontal::Left),
            // Centre: title
            container(
                text("League Builder").size(18),
            )
            .width(FillPortion(1))
            .align_x(Horizontal::Center),
            // Right: dirty indicator + action buttons
            container(
                row![
                    dirty_indicator,
                    Space::new().width(12),
                    button(text("Save League").size(13))
                        .on_press(Message::LbSaveLeague)
                        .style(button::primary),
                    Space::new().width(6),
                    button(text("Export…").size(13))
                        .on_press(Message::LbExportLeague)
                        .style(button::secondary),
                    Space::new().width(6),
                    button(text("Import…").size(13))
                        .on_press(Message::LbImportLeague)
                        .style(button::secondary),
                ]
                .align_y(Vertical::Center),
            )
            .width(FillPortion(1))
            .align_x(Horizontal::Right),
        ]
        .align_y(Vertical::Center),
    )
    .width(Fill)
    .padding(iced::Padding { top: 10., bottom: 10., left: 16., right: 16. })
    .style(bg(HEADER_BG))
    .into()
}

// ---------------------------------------------------------------------------
// Top tab bar — section nav
// ---------------------------------------------------------------------------

fn tab_bar(state: &LeagueBuilderState) -> Element<'_, Message> {
    let mut tabs = row![].spacing(8).align_y(Vertical::Center);
    for section in LEAGUE_BUILDER_SECTIONS {
        let active = &state.active_section == section;
        tabs = tabs.push(
            button(text(section.label()).size(15))
                .on_press(Message::LeagueBuilderNav(section.clone()))
                .style(if active { button::primary } else { button::secondary })
                .padding(iced::Padding { top: 8., bottom: 8., left: 20., right: 20. }),
        );
    }
    container(container(tabs).center_x(Fill))
        .width(Fill)
        .padding(iced::Padding { top: 10., bottom: 10., left: 12., right: 12. })
        .style(bg(HEADER_BG))
        .into()
}

// ---------------------------------------------------------------------------
// Content dispatch
// ---------------------------------------------------------------------------

fn content(state: &LeagueBuilderState) -> Element<'_, Message> {
    match state.active_section {
        LeagueBuilderSection::League      => league_section(state),
        LeagueBuilderSection::Conferences => conferences_section(state),
        LeagueBuilderSection::Tournaments => tournaments_section(state),
        LeagueBuilderSection::Teams       => teams_section(state),
        LeagueBuilderSection::Rosters     => rosters_section(state),
    }
}

fn placeholder(title: &'static str, note: &'static str) -> Element<'static, Message> {
    container(
        column![text(title).size(20), text(note).size(13)]
            .spacing(8)
            .align_x(Horizontal::Center),
    )
    .center(Fill)
    .into()
}

// ---------------------------------------------------------------------------
// League section
// ---------------------------------------------------------------------------

fn league_section(state: &LeagueBuilderState) -> Element<'_, Message> {
    let stat_schools  = state.league.schools.len();
    let stat_confs    = state.league.conferences.len();
    let stat_tourneys = state.league.tournaments.len();

    let stats = column![
        text("League Summary").size(16),
        Space::new().height(12),
        stat_row("Schools",     stat_schools),
        stat_row("Conferences", stat_confs),
        stat_row("Tournaments", stat_tourneys),
    ]
    .spacing(6);

    let actions = column![
        text("Data Files").size(16),
        Space::new().height(8),
        text("Save League — writes schools.json, conferences.json, tournaments.json")
            .size(12).color(LABEL_DIM),
        Space::new().height(4),
        button(text("Save League to data/").size(13))
            .on_press(Message::LbSaveLeague)
            .style(button::primary),
        Space::new().height(16),
        text("Export — bundle everything into a single portable JSON file")
            .size(12).color(LABEL_DIM),
        Space::new().height(4),
        button(text("Export League…").size(13))
            .on_press(Message::LbExportLeague)
            .style(button::secondary),
        Space::new().height(16),
        text("Import — load a previously exported league JSON bundle")
            .size(12).color(LABEL_DIM),
        Space::new().height(4),
        button(text("Import League…").size(13))
            .on_press(Message::LbImportLeague)
            .style(button::secondary),
    ]
    .spacing(2);

    container(
        row![
            container(stats).width(FillPortion(1)).padding(32),
            container(actions).width(FillPortion(2)).padding(32),
        ]
    )
    .width(Fill)
    .height(Fill)
    .style(bg(PANEL_BG))
    .into()
}

fn stat_row(label: &'static str, value: usize) -> Element<'static, Message> {
    row![
        text(label).size(13).color(LABEL_DIM),
        Space::new().width(Fill),
        text(value.to_string()).size(13),
    ]
    .into()
}

// ---------------------------------------------------------------------------
// Conferences section
// ---------------------------------------------------------------------------

fn conferences_section(state: &LeagueBuilderState) -> Element<'_, Message> {
    let editor: Element<'_, Message> = match state.selected_conf_id
        .and_then(|id| state.league.conference_by_id(id))
    {
        Some(c) => conf_editor(c, state),
        None    => empty_editor("Select a conference to edit."),
    };

    row![
        list_panel(conf_list(state), Message::LbAddConference, "+ Add Conference"),
        container(editor).width(Fill).height(Fill),
    ]
    .into()
}

fn conf_list(state: &LeagueBuilderState) -> Element<'_, Message> {
    let mut confs: Vec<&Conference> = state.league.conferences.iter().collect();
    confs.sort_by(|a, b| a.name.cmp(&b.name));

    let mut col = column![].spacing(0);
    for c in confs {
        let active = state.selected_conf_id == Some(c.id);
        let thumb = logo_thumb(c.logo.as_deref(), 32, LABEL_DIM, &state.logo_cache);
        col = col.push(
            button(
                row![
                    thumb,
                    Space::new().width(8),
                    text(&c.name).size(13),
                ]
                .align_y(Vertical::Center)
                .padding(iced::Padding { top: 6., bottom: 6., left: 10., right: 10. })
            )
            .on_press(Message::LbConferenceSelect(c.id))
            .style(if active { button::primary } else { button::text })
            .width(Fill),
        );
    }

    scrollable(col).height(Fill).id(WidgetId::new("lb_conf_list")).into()
}

fn conf_editor<'a>(c: &'a Conference, state: &'a LeagueBuilderState) -> Element<'a, Message> {
    let tier_label = match c.tier {
        bbs_ch::model::prestige::ConferenceTier::PowerConference => "Power Conference",
        bbs_ch::model::prestige::ConferenceTier::MidMajor        => "Mid-Major",
        bbs_ch::model::prestige::ConferenceTier::LowMajor        => "Low-Major",
        bbs_ch::model::prestige::ConferenceTier::Independent      => "Independent",
    };
    let region_label = match c.region {
        bbs_ch::model::conference::Region::Northeast   => "Northeast",
        bbs_ch::model::conference::Region::MidAtlantic => "Mid-Atlantic",
        bbs_ch::model::conference::Region::Southeast   => "Southeast",
        bbs_ch::model::conference::Region::Midwest     => "Midwest",
        bbs_ch::model::conference::Region::Plains      => "Plains",
        bbs_ch::model::conference::Region::Southwest   => "Southwest",
        bbs_ch::model::conference::Region::West        => "West",
    };

    let conf_id = c.id;
    let has_logo = c.logo.is_some();
    let logo_widget = logo_thumb(c.logo.as_deref(), 64, LABEL_DIM, &state.logo_cache);
    let logo_status_color = if has_logo { ACCENT_GREEN } else { ACCENT_RED };
    let logo_status = if has_logo { "✓  Logo set" } else { "✗  No logo" };
    let import_label = if has_logo { "Change Logo…" } else { "Import Logo…" };

    let form = column![
        editor_toolbar(&c.name, button(text("Remove Conference").size(12))
            .on_press(Message::LbRemoveConference(conf_id))
            .style(button::danger)),
        scrollable(
            column![
                section_header("Identity"),
                form_row(
                    field("Name",
                        text_input("Conference name", &c.name)
                            .on_input(Message::LbConfName).padding(8)),
                    field("Abbreviation",
                        text_input("e.g. ACC", &c.abbreviation)
                            .on_input(Message::LbConfAbbr).padding(8)),
                ),
                section_header("Classification"),
                form_row(
                    field("Region",
                        read_only_field(region_label)),
                    field("Tier",
                        read_only_field(tier_label)),
                ),
                section_header("Members"),
                container(
                    text(format!("{} schools", c.members.len()))
                        .size(13).color(LABEL_DIM)
                ).padding(iced::Padding { top: 4., bottom: 12., left: 24., right: 24. }),
                section_header("Assets"),
                container(
                    row![
                        logo_widget,
                        Space::new().width(16),
                        column![
                            text(logo_status).size(13).color(logo_status_color),
                            Space::new().height(6),
                            text("SVG or PNG — stored as data/logos/{ABBR}.ext")
                                .size(11).color(LABEL_DIM),
                            Space::new().height(8),
                            if has_logo {
                                row![
                                    button(text(import_label).size(13))
                                        .on_press(Message::LbFilePickerOpen(super::FilePickerContext::ConfLogo(conf_id)))
                                        .style(button::primary),
                                    Space::new().width(6),
                                    button(text("Delete Logo").size(13))
                                        .on_press(Message::LbDeleteConfLogo(conf_id))
                                        .style(button::danger),
                                ]
                            } else {
                                row![
                                    button(text(import_label).size(13))
                                        .on_press(Message::LbFilePickerOpen(super::FilePickerContext::ConfLogo(conf_id)))
                                        .style(button::primary),
                                ]
                            },
                        ]
                        .spacing(0),
                    ]
                    .align_y(Vertical::Center),
                )
                .padding(iced::Padding { top: 8., bottom: 24., left: 24., right: 24. }),
            ]
            .spacing(0)
        ).height(Fill).id(WidgetId::new("lb_conf_editor")),
    ]
    .spacing(0);

    form.into()
}

// ---------------------------------------------------------------------------
// Tournaments section
// ---------------------------------------------------------------------------

fn tournaments_section(state: &LeagueBuilderState) -> Element<'_, Message> {
    let editor: Element<'_, Message> = match state.selected_tourney_id
        .and_then(|id| state.league.tournament_by_id(id))
    {
        Some(t) => tourney_editor(t, state),
        None    => empty_editor("Select a tournament to edit."),
    };

    row![
        list_panel(tourney_list(state), Message::LbAddTournament, "+ Add Tournament"),
        container(editor).width(Fill).height(Fill),
    ]
    .into()
}

fn tourney_list(state: &LeagueBuilderState) -> Element<'_, Message> {
    let mut col = column![].spacing(0);
    for t in &state.league.tournaments {
        let active = state.selected_tourney_id == Some(t.id);
        let kind_label = match t.kind {
            bbs_ch::model::tournament::TournamentKind::NationalChampionship => "National",
            bbs_ch::model::tournament::TournamentKind::SecondaryPostseason  => "Secondary",
            bbs_ch::model::tournament::TournamentKind::ConferenceTourney    => "Conf",
            bbs_ch::model::tournament::TournamentKind::InSeason             => "In-Season",
        };
        let thumb = logo_thumb(t.logo.as_deref(), 32, LABEL_DIM, &state.logo_cache);
        col = col.push(
            button(
                row![
                    thumb,
                    Space::new().width(8),
                    text(&t.name).size(13),
                    Space::new().width(Fill),
                    text(kind_label).size(11).color(LABEL_DIM),
                ]
                .align_y(Vertical::Center)
                .padding(iced::Padding { top: 6., bottom: 6., left: 10., right: 10. })
            )
            .on_press(Message::LbTournamentSelect(t.id))
            .style(if active { button::primary } else { button::text })
            .width(Fill),
        );
    }

    scrollable(col).height(Fill).id(WidgetId::new("lb_tourney_list")).into()
}

fn tourney_editor<'a>(t: &'a Tournament, state: &'a LeagueBuilderState) -> Element<'a, Message> {
    let kind_label = match t.kind {
        bbs_ch::model::tournament::TournamentKind::NationalChampionship => "National Championship",
        bbs_ch::model::tournament::TournamentKind::SecondaryPostseason  => "Secondary Postseason",
        bbs_ch::model::tournament::TournamentKind::ConferenceTourney    => "Conference Tournament",
        bbs_ch::model::tournament::TournamentKind::InSeason             => "In-Season Event",
    };
    let format_label = match t.bracket_format {
        bbs_ch::model::tournament::BracketFormat::SingleElimination => "Single Elimination",
        bbs_ch::model::tournament::BracketFormat::DoubleElimination => "Double Elimination",
        bbs_ch::model::tournament::BracketFormat::RoundRobin        => "Round Robin",
    };

    let tourney_id = t.id;
    let has_logo = t.logo.is_some();
    let logo_widget = logo_thumb(t.logo.as_deref(), 64, LABEL_DIM, &state.logo_cache);
    let logo_status_color = if has_logo { ACCENT_GREEN } else { ACCENT_RED };
    let logo_status = if has_logo { "✓  Logo set" } else { "✗  No logo" };
    let import_label = if has_logo { "Change Logo…" } else { "Import Logo…" };

    column![
        editor_toolbar(&t.name, button(text("Remove Tournament").size(12))
            .on_press(Message::LbRemoveTournament(tourney_id))
            .style(button::danger)),
        scrollable(
            column![
                section_header("Identity"),
                form_row(
                    field("Name",
                        text_input("Tournament name", &t.name)
                            .on_input(Message::LbTourneyName).padding(8)),
                    field("Abbreviation",
                        text_input("e.g. NCAAT", &t.abbreviation)
                            .on_input(Message::LbTourneyAbbr).padding(8)),
                ),
                section_header("Format"),
                form_row(
                    field("Kind", read_only_field(kind_label)),
                    field("Bracket", read_only_field(format_label)),
                ),
                section_header("Field"),
                form_row(
                    field("Field Size",
                        text_input("e.g. 68", &t.field_size.to_string())
                            .on_input(Message::LbTourneyFieldSize).padding(8)),
                    field("Host Site",
                        text_input("City, state or venue", &t.host_site)
                            .on_input(Message::LbTourneyHostSite).padding(8)),
                ),
                form_row(
                    field("Auto Bids",
                        text_input("0", &t.automatic_bids.to_string())
                            .on_input(Message::LbTourneyAutoBids).padding(8)),
                    field("At-Large Bids",
                        text_input("0", &t.at_large_bids.to_string())
                            .on_input(Message::LbTourneyAtLargeBids).padding(8)),
                ),
                section_header("Assets"),
                container(
                    row![
                        logo_widget,
                        Space::new().width(16),
                        column![
                            text(logo_status).size(13).color(logo_status_color),
                            Space::new().height(6),
                            text("SVG or PNG — stored as data/logos/{ABBR}.ext")
                                .size(11).color(LABEL_DIM),
                            Space::new().height(8),
                            if has_logo {
                                row![
                                    button(text(import_label).size(13))
                                        .on_press(Message::LbFilePickerOpen(super::FilePickerContext::TourneyLogo(tourney_id)))
                                        .style(button::primary),
                                    Space::new().width(6),
                                    button(text("Delete Logo").size(13))
                                        .on_press(Message::LbDeleteTourneyLogo(tourney_id))
                                        .style(button::danger),
                                ]
                            } else {
                                row![
                                    button(text(import_label).size(13))
                                        .on_press(Message::LbFilePickerOpen(super::FilePickerContext::TourneyLogo(tourney_id)))
                                        .style(button::primary),
                                ]
                            },
                        ]
                        .spacing(0),
                    ]
                    .align_y(Vertical::Center),
                )
                .padding(iced::Padding { top: 8., bottom: 24., left: 24., right: 24. }),
            ]
            .spacing(0)
        ).height(Fill).id(WidgetId::new("lb_tourney_editor")),
    ]
    .spacing(0)
    .into()
}

// ---------------------------------------------------------------------------
// Teams section
// ---------------------------------------------------------------------------

fn teams_section(state: &LeagueBuilderState) -> Element<'_, Message> {
    let editor: Element<'_, Message> = match state.selected_team_id
        .and_then(|id| state.league.school_by_id(id))
    {
        Some(_) => team_editor(state),
        None    => empty_editor("Select a team to edit."),
    };

    row![
        list_panel(team_list(state), Message::LbAddTeam, "+ Add Team"),
        container(editor).width(Fill).height(Fill),
    ]
    .into()
}

fn rosters_section(state: &LeagueBuilderState) -> Element<'_, Message> {
    row![
        list_panel(team_list(state), Message::LbAddTeam, "+ Add Team"),
        container(placeholder("Rosters", "Roster editing coming soon."))
            .width(Fill)
            .height(Fill),
    ]
    .into()
}

fn team_list(state: &LeagueBuilderState) -> Element<'_, Message> {
    let mut schools: Vec<&School> = state.league.schools.iter().collect();
    schools.sort_by(|a, b| a.name.cmp(&b.name));

    let mut col = column![].spacing(0);
    for (i, school) in schools.iter().enumerate() {
        let active = state.selected_team_id == Some(school.id);
        let thumb = school_logo_thumb(school, 32, &state.logo_cache);

        col = col.push(
            container(
                button(
                    row![
                        thumb,
                        Space::new().width(10),
                        row![
                            text(&school.name).size(13).font(theme::bold()),
                            Space::new().width(6),
                            text(&school.nickname).size(13).color(LABEL_DIM),
                        ]
                        .align_y(Vertical::Center),
                    ]
                    .align_y(Vertical::Center)
                    .padding(iced::Padding { top: 6., bottom: 6., left: 10., right: 10. })
                )
                .on_press(Message::LbTeamSelect(school.id))
                .style(if active { button::primary } else { button::text })
                .width(Fill),
            )
            .style(row_bg(i)),
        );
    }

    scrollable(col).height(Fill).id(WidgetId::new("lb_team_list")).into()
}

fn team_editor(state: &LeagueBuilderState) -> Element<'_, Message> {
    let s = match state.selected_team_id.and_then(|id| state.league.school_by_id(id)) {
        Some(sc) => sc,
        None => return empty_editor("Select a team to edit."),
    };
    use super::PrestigeField;
    let p = &s.prestige;
    let school_id = s.id;
    let primary_color   = hex_to_color(&s.primary_color);
    let secondary_color = hex_to_color(&s.secondary_color);
    let has_logo = find_logo(&s.name, &s.abbreviation).is_some();

    // ── Left column: logo + import/delete + colors ───────────────────────
    let logo_80 = school_logo_thumb(s, 80, &state.logo_cache);

    // Logo action buttons: always show "Import Logo", show "Delete" only when set.
    let logo_actions: Element<'_, Message> = if has_logo {
        row![
            button(text("Change Logo…").size(11))
                .on_press(Message::LbFilePickerOpen(super::FilePickerContext::SchoolLogo(school_id)))
                .style(button::secondary),
            Space::new().width(4),
            button(text("Delete").size(11))
                .on_press(Message::LbDeleteSchoolLogo(school_id))
                .style(button::danger),
        ].into()
    } else {
        button(text("Import Logo…").size(11))
            .on_press(Message::LbFilePickerOpen(super::FilePickerContext::SchoolLogo(school_id)))
            .style(button::secondary)
            .into()
    };

    let left_col: Element<'_, Message> = column![
        logo_80,
        Space::new().height(6),
        logo_actions,
        Space::new().height(12),
        field_label("Primary"),
        row![
            color_swatch(primary_color, 22),
            Space::new().width(6),
            text_input("#rrggbb", &s.primary_color)
                .on_input(Message::LbTeamPrimaryColor).padding(5),
        ].align_y(Vertical::Center),
        Space::new().height(6),
        field_label("Secondary"),
        row![
            color_swatch(secondary_color, 22),
            Space::new().width(6),
            text_input("#rrggbb", &s.secondary_color)
                .on_input(Message::LbTeamSecondaryColor).padding(5),
        ].align_y(Vertical::Center),
    ]
    .spacing(0)
    .width(200)
    .into();

    // ── Right column: identity fields ────────────────────────────────────
    let right_col: Element<'_, Message> = column![
        field("Name",
            text_input("School name", &s.name)
                .on_input(Message::LbTeamName).padding(8)),
        Space::new().height(6),
        field("Nickname / Mascot",
            text_input("e.g. Wildcats", &s.nickname)
                .on_input(Message::LbTeamNickname).padding(8)),
        Space::new().height(6),
        row![
            container(
                field("Abbreviation",
                    text_input("e.g. UK", &s.abbreviation)
                        .on_input(Message::LbTeamAbbr).padding(8))
            ).width(FillPortion(1)),
            Space::new().width(12),
            container(
                field("Conference",
                    text_input("Conference abbr.", &s.conference)
                        .on_input(Message::LbTeamConference).padding(8))
            ).width(FillPortion(1)),
        ],
        Space::new().height(6),
        row![
            container(
                field("Zip Code",
                    text_input("e.g. 40506", &s.zip_code)
                        .on_input(Message::LbTeamZip).padding(8))
            ).width(FillPortion(1)),
            Space::new().width(12),
            container(
                field("Enrollment",
                    text_input("e.g. 32000", &s.enrollment.to_string())
                        .on_input(Message::LbTeamEnrollment).padding(8))
            ).width(FillPortion(1)),
        ],
        Space::new().height(6),
        field("Prefer Abbr. in UI",
            button(text(if s.prefer_abbreviation { "✓ Yes" } else { "  No" }).size(13))
                .on_press(Message::LbTeamPreferAbbr(!s.prefer_abbreviation))
                .style(if s.prefer_abbreviation { button::primary } else { button::secondary })),
    ]
    .spacing(0)
    .width(Fill)
    .into();

    // ── Identity header card ─────────────────────────────────────────────
    let identity_card: Element<'_, Message> = container(
        row![left_col, Space::new().width(20), right_col]
            .align_y(Vertical::Top)
    )
    .padding(iced::Padding { top: 16., bottom: 16., left: 20., right: 20. })
    .width(Fill)
    .into();

    column![
        editor_toolbar(&s.name, button(text("Remove Team").size(12))
            .on_press(Message::LbRemoveTeam(school_id))
            .style(button::danger)),
        scrollable(
            column![
                identity_card,
                // ── Home Court ──────────────────────────────────────────────
                section_header("Home Court"),
                form_row(
                    field("Arena Name",
                        text_input("e.g. Rupp Arena", &s.home_court.name)
                            .on_input(Message::LbTeamCourtName).padding(8)),
                    field("Capacity",
                        text_input("e.g. 23500", &s.home_court.capacity.to_string())
                            .on_input(Message::LbTeamCourtCapacity).padding(8)),
                ),
                form_row(
                    field("Atmosphere (0–99)",
                        text_input("e.g. 85", &s.home_court.atmosphere.to_string())
                            .on_input(Message::LbTeamCourtAtmosphere).padding(8)),
                    field("", Space::new().height(0)),
                ),
                // ── Prestige ────────────────────────────────────────────────
                section_header("Prestige"),
                container(
                    column![
                        prestige_row("Cumulative Success", p.cumulative_success, PrestigeField::CumulativeSuccess),
                        prestige_row("Recent Success",     p.recent_success,     PrestigeField::RecentSuccess),
                        prestige_row("Facilities",         p.facilities,         PrestigeField::Facilities),
                        prestige_row("Resources",          p.resources,          PrestigeField::Resources),
                        prestige_row("Atmosphere",         p.atmosphere,         PrestigeField::Atmosphere),
                        prestige_row("Coaching Staff",     p.coaching_staff,     PrestigeField::CoachingStaff),
                        prestige_row("Market",             p.market,             PrestigeField::Market),
                        prestige_row("Media Perception",   p.media_perception,   PrestigeField::MediaPerception),
                        prestige_row("Academics",          p.academics,          PrestigeField::Academics),
                        Space::new().height(12),
                        row![
                            container(text("NIL Budget ($)").size(13)).width(200),
                            text_input("e.g. 5000000", &s.nil_budget.to_string())
                                .on_input(Message::LbTeamNilBudget).padding(6),
                        ]
                        .align_y(Vertical::Center)
                        .spacing(12),
                    ]
                    .spacing(6)
                )
                .padding(iced::Padding { top: 8., bottom: 16., left: 24., right: 24. }),
                // ── Assets (court graphic only) ──────────────────────────────
                section_header("Court Graphic"),
                container(
                    column![
                        text("PNG — 500×500 or 1024×1024 recommended").size(11).color(LABEL_DIM),
                        Space::new().height(8),
                        button(text("Import Court Graphic…").size(13))
                            .on_press(Message::LbFilePickerOpen(super::FilePickerContext::SchoolCourt(school_id)))
                            .style(button::secondary),
                    ]
                    .spacing(0)
                )
                .padding(iced::Padding { top: 8., bottom: 24., left: 24., right: 24. }),
                // ── Rivalries ───────────────────────────────────────────────
                section_header("Rivalries"),
                rivalries_section(s, state),
            ]
            .spacing(0)
        ).height(Fill).id(WidgetId::new("lb_team_editor")),
    ]
    .spacing(0)
    .into()
}

/// Renders the Rivalries section inside the team editor.
///
/// When a rival picker is open, an inline search panel replaces the "add" controls
/// at the bottom of the section. Clicking a school commits the selection and closes
/// the picker.
fn rivalries_section<'a>(s: &'a School, state: &'a LeagueBuilderState) -> Element<'a, Message> {
    let school_id = s.id;

    // ── Primary rival row ─────────────────────────────────────────────────
    // Hoist the match to avoid type-inference issues inside macro invocations.
    let primary_display: Element<'_, Message> = match &s.primary_rival {
        Some(abbr) => row![
            text(abbr.as_str()).size(14),
            Space::new().width(12),
            button(text("✕  Clear").size(12))
                .on_press(Message::LbTeamClearPrimaryRival)
                .style(button::danger),
        ]
        .align_y(Vertical::Center)
        .into(),
        None => text("None").size(13).color(LABEL_DIM).into(),
    };

    let primary_row: Element<'_, Message> = container(
        row![
            column![
                text("Primary Rival").size(11).color(LABEL_DIM),
                Space::new().height(4),
                primary_display,
            ]
            .spacing(0),
            Space::new().width(Fill),
            button(text(if s.primary_rival.is_some() { "Change…" } else { "Set Primary Rival…" }).size(12))
                .on_press(Message::LbRivalPickerOpen(RivalPickerContext::Primary))
                .style(button::secondary),
        ]
        .align_y(Vertical::Center)
    )
    .padding(iced::Padding { top: 8., bottom: 8., left: 24., right: 24. })
    .width(Fill)
    .into();

    // ── Rivals list ───────────────────────────────────────────────────────
    let mut rivals_col = column![].spacing(2);
    for (i, rival_abbr) in s.rivals.iter().enumerate() {
        let abbr_clone = rival_abbr.clone();
        rivals_col = rivals_col.push(
            container(
                row![
                    text(rival_abbr).size(13),
                    Space::new().width(Fill),
                    button(text("✕").size(12))
                        .on_press(Message::LbTeamRemoveRival(abbr_clone))
                        .style(button::danger),
                ]
                .align_y(Vertical::Center)
                .padding(iced::Padding { top: 4., bottom: 4., left: 8., right: 8. }),
            )
            .style(row_bg(i))
            .width(Fill),
        );
    }
    let add_rival_btn = button(text("+ Add Rival…").size(12))
        .on_press(Message::LbRivalPickerOpen(RivalPickerContext::AddRival))
        .style(button::secondary);

    // ── Rival picker (inline, shown when open) ────────────────────────────
    let picker_panel: Option<Element<'_, Message>> = state.rival_picker.as_ref().map(|ctx| {
        let query = state.rival_search.to_lowercase();
        let mut results = column![].spacing(0);
        let mut count = 0usize;
        let mut all_schools: Vec<&School> = state.league.schools.iter()
            .filter(|sc| sc.id != school_id) // exclude self
            .filter(|sc| {
                if let RivalPickerContext::AddRival = ctx {
                    // exclude schools already in the rivals list
                    !s.rivals.contains(&sc.abbreviation)
                } else {
                    true
                }
            })
            .collect();
        all_schools.sort_by(|a, b| a.name.cmp(&b.name));

        for (i, sc) in all_schools.iter().enumerate() {
            let matches = query.is_empty()
                || sc.name.to_lowercase().contains(&query)
                || sc.abbreviation.to_lowercase().contains(&query);
            if !matches { continue; }
            count += 1;
            let abbr = sc.abbreviation.clone();
            let select_msg = match ctx {
                RivalPickerContext::Primary  => Message::LbTeamSetPrimaryRival(abbr),
                RivalPickerContext::AddRival => Message::LbTeamAddRival(abbr),
            };
            results = results.push(
                button(
                    row![
                        text(&sc.name).size(13),
                        Space::new().width(8),
                        text(format!("({})", sc.abbreviation)).size(11).color(LABEL_DIM),
                    ]
                    .padding(iced::Padding { top: 6., bottom: 6., left: 10., right: 10. })
                )
                .on_press(select_msg)
                .style(if i % 2 == 0 { button::text } else { button::text })
                .width(Fill),
            );
        }
        if count == 0 {
            results = results.push(
                container(text("No matches").size(12).color(LABEL_DIM)).padding(10),
            );
        }

        let title = match ctx {
            RivalPickerContext::Primary  => "Set Primary Rival",
            RivalPickerContext::AddRival => "Add Rival",
        };

        container(
            column![
                container(
                    row![
                        text(title).size(12).color(LABEL_DIM),
                        Space::new().width(Fill),
                        button(text("✕ Close").size(11))
                            .on_press(Message::LbRivalPickerClose)
                            .style(button::text),
                    ]
                    .align_y(Vertical::Center)
                )
                .padding(iced::Padding { top: 6., bottom: 6., left: 10., right: 8. })
                .width(Fill)
                .style(bg(HEADER_BG)),
                container(
                    text_input("Search by name or abbreviation…", &state.rival_search)
                        .on_input(Message::LbRivalSearchChanged)
                        .padding(8)
                )
                .padding(iced::Padding { top: 6., bottom: 6., left: 8., right: 8. })
                .width(Fill),
                scrollable(results).height(200),
            ]
            .spacing(0)
        )
        .width(Fill)
        .style(bg(PICKER_BG))
        .into()
    });

    // ── Primary rival notice ──────────────────────────────────────────────
    let primary_notice: Element<'_, Message> = container(
        text("Setting a primary rival applies to both teams automatically.")
            .size(11).color(LABEL_DIM)
    )
    .padding(iced::Padding { top: 0., bottom: 8., left: 24., right: 24. })
    .width(Fill)
    .into();

    // ── Assemble ─────────────────────────────────────────────────────────
    let mut col = column![
        primary_row,
        primary_notice,
        container(
            column![
                text("Rivals").size(11).color(LABEL_DIM),
                Space::new().height(4),
                rivals_col,
                Space::new().height(6),
                add_rival_btn,
            ]
            .spacing(0)
        )
        .padding(iced::Padding { top: 4., bottom: 16., left: 24., right: 24. })
        .width(Fill),
    ]
    .spacing(0);

    if let Some(picker) = picker_panel {
        col = col.push(
            container(picker)
                .padding(iced::Padding { top: 0., bottom: 8., left: 24., right: 24. })
                .width(Fill),
        );
    }

    col.into()
}

// Takes u8 directly and formats inline so the returned element is 'static
// (iced copies the value string into Value::new() immediately, no borrow survives).
fn prestige_row(label: &'static str, value: u8, field: PrestigeField) -> Element<'static, Message> {
    row![
        container(text(label).size(13)).width(200),
        text_input("0–99", &value.to_string())
            .on_input(move |v| Message::LbTeamPrestige(field.clone(), v))
            .padding(6),
    ]
    .align_y(Vertical::Center)
    .spacing(12)
    .into()
}

// ---------------------------------------------------------------------------
// Layout helpers shared across sections
// ---------------------------------------------------------------------------

/// A two-column panel: scrollable list on the left + fixed Add button at the bottom.
fn list_panel<'a>(list: Element<'a, Message>, add_msg: Message, add_label: &'static str) -> Element<'a, Message> {
    column![
        list,
        container(
            button(text(add_label).size(12))
                .on_press(add_msg)
                .style(button::secondary)
                .width(Fill)
        )
        .padding(8)
        .width(Fill)
        .style(bg(SIDEBAR_BG)),
    ]
    .width(320)
    .height(Fill)
    .into()
}

/// Toolbar at the top of an editor: title on the left, an action button on the right.
fn editor_toolbar<'a>(title: &'a str, action: iced::widget::Button<'a, Message>) -> Element<'a, Message> {
    container(
        row![
            text(title).size(14),
            Space::new().width(Fill),
            action,
        ]
        .align_y(Vertical::Center),
    )
    .padding(iced::Padding { top: 10., bottom: 10., left: 20., right: 16. })
    .width(Fill)
    .style(bg(HEADER_BG))
    .into()
}

/// A styled section divider with a label.
fn section_header(label: &'static str) -> Element<'static, Message> {
    container(text(label).size(11).color(LABEL_DIM))
        .padding(iced::Padding { top: 10., bottom: 6., left: 24., right: 24. })
        .width(Fill)
        .style(bg(DIVIDER_BG))
        .into()
}

/// Two fields side-by-side with equal width, padded.
fn form_row<'a>(left: Element<'a, Message>, right: Element<'a, Message>) -> Element<'a, Message> {
    container(
        row![
            container(left).width(FillPortion(1)),
            Space::new().width(16),
            container(right).width(FillPortion(1)),
        ]
    )
    .padding(iced::Padding { top: 4., bottom: 4., left: 24., right: 24. })
    .into()
}

/// A labelled field — label above, widget below.
fn field<'a>(label: &'static str, widget: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    column![field_label(label), widget.into()]
        .spacing(4)
        .into()
}

fn read_only_field(value: &str) -> Element<'_, Message> {
    container(text(value).size(13))
        .padding(8)
        .style(bg(theme::BG_ROW_B))
        .width(Fill)
        .into()
}

fn color_swatch(color: Color, size: u32) -> Element<'static, Message> {
    container(Space::new().width(size).height(size))
        .style(move |_| CStyle {
            background: Some(Background::Color(color)),
            border: Border { radius: 4.0.into(), ..Default::default() },
            ..Default::default()
        })
        .into()
}

fn empty_editor(hint: &'static str) -> Element<'static, Message> {
    container(text(hint).size(13).color(LABEL_DIM))
        .center(Fill)
        .into()
}

fn field_label(label: &'static str) -> Element<'static, Message> {
    text(label).size(11).color(LABEL_DIM).into()
}
