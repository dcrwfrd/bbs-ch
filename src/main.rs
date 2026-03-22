mod ui;

use std::path::PathBuf;

use iced::{Element, Task, Theme};

use bbs_ch::data::loader::DataLoader;
use bbs_ch::model::{
    game_state::{GameState, SaveMode, SeasonPhase},
    league::League,
};

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("Blue Blood Sports: College Hoops")
        .theme(App::theme)
        .scale_factor(|app| ui::scale_factor(app.scale_idx))
        .window(iced::window::Settings {
            size: iced::Size::new(1280.0, 800.0),
            min_size: Some(iced::Size::new(900.0, 600.0)),
            maximized: true,
            ..Default::default()
        })
        .run()
}

// ---------------------------------------------------------------------------
// Top-level application state
// ---------------------------------------------------------------------------

struct App {
    screen: Screen,
    scale_idx: usize,
    /// Directory last used by any asset import dialog, restored on next open.
    last_asset_dir: Option<PathBuf>,
    /// Whether the file picker was last left in grid view; persisted across opens.
    last_picker_grid_view: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            screen: Screen::MainMenu,
            scale_idx: ui::DEFAULT_SCALE_IDX,
            last_asset_dir: None,
            last_picker_grid_view: false,
        }
    }
}

enum Screen {
    MainMenu,
    NewGame(ui::NewGameState),
    LeagueBuilder(Box<ui::LeagueBuilderState>),
    InGame(Box<ui::InGameState>),
}

// ---------------------------------------------------------------------------
// Messages
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum Message {
    // Main menu
    NewGamePressed,
    QuitPressed,

    // Wizard — step 1
    SaveNameChanged(String),
    SeasonIncrement,
    SeasonDecrement,

    // Wizard — navigation
    NextStep,
    PrevStep,

    // Wizard — step 2
    ModeSelected(SaveMode),

    // Wizard — step 3
    ConferenceSelected(u32),
    SchoolSelected(u32),

    // Wizard — finish
    StartGame,

    // In-game navigation
    InGameNav(ui::NavItem),
    InGameSubNav(usize),

    // Top bar panels
    SchedulePanelToggle,
    AdvancePanelToggle,
    GameMenuToggle,

    // Time advancement
    AdvanceDay,
    AdvanceToNextEvent,
    AdvanceToEndOfPhase,

    // Settings
    TickerToggle,
    UiScaleIncrease,
    UiScaleDecrease,

    // Quit
    QuitToMenu,
    QuitToDesktop,

    // League Builder — screen-level navigation (handled here; rest in league_builder::update)
    LeagueBuilderOpen,
    LeagueBuilderBack,

    // League Builder — all Lb* messages delegated to ui::league_builder::update
    LeagueBuilderNav(ui::LeagueBuilderSection),
    LbConferenceSelect(u32),
    LbConfName(String),
    LbConfAbbr(String),
    LbConfRegion(String),
    LbConfTier(String),
    LbTournamentSelect(u32),
    LbTourneyName(String),
    LbTourneyAbbr(String),
    LbTourneyKind(String),
    LbTourneyFormat(String),
    LbTourneyFieldSize(String),
    LbTourneyHostSite(String),
    LbTourneyAutoBids(String),
    LbTourneyAtLargeBids(String),
    LbTeamSelect(u32),
    LbTeamName(String),
    LbTeamNickname(String),
    LbTeamAbbr(String),
    LbTeamPreferAbbr(bool),
    LbTeamZip(String),
    LbTeamConference(String),
    LbTeamPrimaryColor(String),
    LbTeamSecondaryColor(String),
    LbTeamCourtName(String),
    LbTeamCourtCapacity(String),
    LbTeamCourtAtmosphere(String),
    LbTeamEnrollment(String),
    LbTeamNilBudget(String),
    LbTeamPrestige(ui::PrestigeField, String),
    LbDeleteSchoolLogo(u32),
    LbDeleteConfLogo(u32),
    LbDeleteTourneyLogo(u32),
    LbFilePickerOpen(ui::FilePickerContext),
    LbFilePickerNavigate(PathBuf),
    LbFilePickerSelect(PathBuf),
    LbFilePickerSaveName(String),
    LbFilePickerConfirm(PathBuf),
    LbFilePickerCancel,
    LbFilePickerToggleView,
    LbRivalPickerOpen(ui::RivalPickerContext),
    LbRivalPickerClose,
    LbRivalSearchChanged(String),
    LbTeamSetPrimaryRival(String),
    LbTeamClearPrimaryRival,
    LbTeamAddRival(String),
    LbTeamRemoveRival(String),
    LbAddConference,
    LbRemoveConference(u32),
    LbAddTournament,
    LbRemoveTournament(u32),
    LbAddTeam,
    LbRemoveTeam(u32),
    LbSaveLeague,
    LbExportLeague,
    LbImportLeague,
    /// A single thumbnail finished decoding off the main thread.
    LbThumbnailReady(std::path::PathBuf, u32, u32, Vec<u8>),

    #[allow(dead_code)]
    Noop,
}

// ---------------------------------------------------------------------------
// Update
// ---------------------------------------------------------------------------

impl App {
    fn theme(&self) -> Theme { Theme::Dark }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            // ---------------------------------------------------------------
            // Main menu
            // ---------------------------------------------------------------
            Message::NewGamePressed => {
                self.screen = Screen::NewGame(ui::NewGameState::new());
            }
            Message::QuitPressed | Message::QuitToDesktop => std::process::exit(0),
            Message::QuitToMenu => { self.screen = Screen::MainMenu; }

            // ---------------------------------------------------------------
            // Wizard
            // ---------------------------------------------------------------
            Message::SaveNameChanged(s) => {
                if let Screen::NewGame(ref mut ng) = self.screen { ng.save_name = s; }
            }
            Message::SeasonIncrement => {
                if let Screen::NewGame(ref mut ng) = self.screen {
                    if ng.season < bbs_ch::model::game_state::MAX_START_SEASON { ng.season += 1; }
                }
            }
            Message::SeasonDecrement => {
                if let Screen::NewGame(ref mut ng) = self.screen {
                    if ng.season > bbs_ch::model::game_state::MIN_START_SEASON { ng.season -= 1; }
                }
            }
            Message::NextStep => {
                if let Screen::NewGame(ref mut ng) = self.screen {
                    ng.step = match ng.step {
                        ui::WizardStep::NameAndSeason => ui::WizardStep::ChooseMode,
                        ui::WizardStep::ChooseMode => match &ng.mode {
                            Some(SaveMode::Observer) => ui::WizardStep::ObserverConfirm,
                            _ => ui::WizardStep::PickSchool,
                        },
                        _ => return Task::none(),
                    };
                }
            }
            Message::PrevStep => {
                let go_home = if let Screen::NewGame(ref mut ng) = self.screen {
                    match ng.step {
                        ui::WizardStep::NameAndSeason => true,
                        ui::WizardStep::ChooseMode => { ng.step = ui::WizardStep::NameAndSeason; false }
                        ui::WizardStep::PickSchool | ui::WizardStep::ObserverConfirm => {
                            ng.step = ui::WizardStep::ChooseMode; false
                        }
                    }
                } else { false };
                if go_home { self.screen = Screen::MainMenu; }
            }
            Message::ModeSelected(mode) => {
                if let Screen::NewGame(ref mut ng) = self.screen { ng.mode = Some(mode); }
            }
            Message::ConferenceSelected(id) => {
                if let Screen::NewGame(ref mut ng) = self.screen {
                    ng.selected_conf_id = Some(id);
                    ng.selected_school_id = None;
                }
            }
            Message::SchoolSelected(id) => {
                if let Screen::NewGame(ref mut ng) = self.screen { ng.selected_school_id = Some(id); }
            }
            Message::StartGame => {
                let old = std::mem::replace(&mut self.screen, Screen::MainMenu);
                if let Screen::NewGame(ng) = old {
                    let (coach_id, school_id) = match &ng.mode {
                        Some(SaveMode::Observer) => (None, None),
                        _ => (Some(1), ng.selected_school_id),
                    };
                    let gs = GameState {
                        save_name: ng.save_name,
                        save_mode: ng.mode.unwrap_or(SaveMode::Normal),
                        season: ng.season,
                        phase: SeasonPhase::Offseason,
                        current_date: bbs_ch::model::game_state::GameDate::season_open(ng.season),
                        player_coach_id: coach_id,
                        player_school_id: school_id,
                    };
                    let league = ng.world.unwrap_or(League {
                        schools: vec![], conferences: vec![], tournaments: vec![], logo: None,
                    });
                    self.screen = Screen::InGame(Box::new(ui::InGameState::new(gs, league)));
                }
            }

            // ---------------------------------------------------------------
            // In-game
            // ---------------------------------------------------------------
            Message::InGameNav(nav) => {
                if let Screen::InGame(ref mut state) = self.screen {
                    state.active_nav = if state.active_nav.as_ref() == Some(&nav) { None } else { Some(nav) };
                    state.active_sub = None;
                    state.schedule_open = false;
                    state.advance_open = false;
                    state.game_menu_open = false;
                }
            }
            Message::InGameSubNav(idx) => {
                if let Screen::InGame(ref mut state) = self.screen { state.active_sub = Some(idx); }
            }
            Message::SchedulePanelToggle => {
                if let Screen::InGame(ref mut state) = self.screen {
                    state.schedule_open = !state.schedule_open;
                    state.advance_open = false;
                    state.game_menu_open = false;
                }
            }
            Message::AdvancePanelToggle => {
                if let Screen::InGame(ref mut state) = self.screen {
                    state.advance_open = !state.advance_open;
                    state.schedule_open = false;
                    state.game_menu_open = false;
                }
            }
            Message::GameMenuToggle => {
                if let Screen::InGame(ref mut state) = self.screen {
                    state.game_menu_open = !state.game_menu_open;
                    state.schedule_open = false;
                    state.advance_open = false;
                }
            }
            Message::AdvanceDay => {
                if let Screen::InGame(ref mut state) = self.screen {
                    state.game.current_date = state.game.current_date.advance_by(1);
                    state.advance_open = false;
                }
            }
            Message::AdvanceToNextEvent => {
                if let Screen::InGame(ref mut state) = self.screen {
                    state.game.current_date = state.game.current_date.advance_by(7);
                    state.advance_open = false;
                }
            }
            Message::AdvanceToEndOfPhase => {
                if let Screen::InGame(ref mut state) = self.screen {
                    state.game.current_date = state.game.current_date.advance_by(30);
                    state.advance_open = false;
                }
            }
            Message::TickerToggle => {
                if let Screen::InGame(ref mut state) = self.screen {
                    state.settings.ticker_visible = !state.settings.ticker_visible;
                    state.game_menu_open = false;
                }
            }
            Message::UiScaleIncrease => {
                if self.scale_idx + 1 < ui::SCALE_STEPS_PCT.len() { self.scale_idx += 1; }
            }
            Message::UiScaleDecrease => {
                self.scale_idx = self.scale_idx.saturating_sub(1);
            }

            // ---------------------------------------------------------------
            // League Builder — screen transitions (App-level)
            // ---------------------------------------------------------------
            Message::LeagueBuilderOpen => {
                let league = DataLoader::new("data").load_league().unwrap_or_else(|_| League {
                    schools: vec![], conferences: vec![], tournaments: vec![], logo: None,
                });
                self.screen = Screen::LeagueBuilder(Box::new(ui::LeagueBuilderState::new(league)));
            }
            Message::LeagueBuilderBack => { self.screen = Screen::MainMenu; }

            // ---------------------------------------------------------------
            // League Builder — all Lb* messages delegated to league_builder::update
            // ---------------------------------------------------------------
            msg => {
                if let Screen::LeagueBuilder(ref mut s) = self.screen {
                    return ui::league_builder::update(s, &mut self.last_asset_dir, &mut self.last_picker_grid_view, msg);
                }
            }
        }
        Task::none()
    }

    // -----------------------------------------------------------------------
    // View dispatch
    // -----------------------------------------------------------------------

    fn view(&self) -> Element<'_, Message> {
        match &self.screen {
            Screen::MainMenu => ui::main_menu::view(),
            Screen::NewGame(ng) => ui::new_game::view(ng),
            Screen::LeagueBuilder(state) => ui::league_builder::view(state),
            Screen::InGame(state) => view_in_game(state),
        }
    }
}

// ---------------------------------------------------------------------------
// In-game view
// ---------------------------------------------------------------------------

fn view_in_game(state: &ui::InGameState) -> Element<'_, Message> {
    let school = state.game.player_school_id
        .and_then(|id| state.league.school_by_id(id));

    let theme = school
        .map(ui::TeamTheme::from_school)
        .unwrap_or_else(ui::TeamTheme::observer);

    let top = ui::top_bar::view(state, school, ui::DEFAULT_SCALE_IDX, theme);

    let mut main_row = iced::widget::row![].height(iced::Fill);
    match &state.active_nav {
        Some(nav) if nav.is_team() => {
            main_row = main_row.push(ui::sidebar::view(state, nav, theme));
            main_row = main_row.push(ui::content::view(state));
        }
        Some(nav) => {
            main_row = main_row.push(ui::content::view(state));
            main_row = main_row.push(ui::sidebar::view(state, nav, theme));
        }
        None => {
            main_row = main_row.push(ui::content::view(state));
        }
    }

    let mut page = iced::widget::column![top, main_row];
    if state.settings.ticker_visible {
        page = page.push(ui::ticker::view(theme));
    }
    page.into()
}
