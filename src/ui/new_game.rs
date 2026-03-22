use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, column, container, row, scrollable, text, text_input, Grid, Space};
use iced::{Element, Fill, FillPortion, Padding};

use bbs_ch::data::loader::DataLoader;
use bbs_ch::model::{
    game_state::{
        DEFAULT_START_SEASON, MAX_START_SEASON, MIN_START_SEASON, SaveMode,
    },
    league::League,
};

use super::main_menu::{card_style, menu_shell};
use crate::Message;

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

pub struct NewGameState {
    pub step: WizardStep,
    pub save_name: String,
    pub season: u32,
    pub mode: Option<SaveMode>,
    pub selected_conf_id: Option<u32>,
    pub selected_school_id: Option<u32>,
    pub world: Option<League>,
    pub world_error: Option<String>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WizardStep {
    NameAndSeason,
    ChooseMode,
    PickSchool,
    ObserverConfirm,
}

impl NewGameState {
    pub fn new() -> Self {
        let (world, world_error) = match DataLoader::new("data").load_league() {
            Ok(w) => (Some(w), None),
            Err(e) => (None, Some(e.to_string())),
        };
        Self {
            step: WizardStep::NameAndSeason,
            save_name: "NewGame".to_string(),
            season: DEFAULT_START_SEASON,
            mode: None,
            selected_conf_id: None,
            selected_school_id: None,
            world,
            world_error,
        }
    }

    pub fn can_advance(&self) -> bool {
        match self.step {
            WizardStep::NameAndSeason => !self.save_name.trim().is_empty(),
            WizardStep::ChooseMode => self.mode.is_some(),
            WizardStep::PickSchool => self.selected_school_id.is_some(),
            WizardStep::ObserverConfirm => true,
        }
    }
}

// ---------------------------------------------------------------------------
// View
// ---------------------------------------------------------------------------

pub fn view(ng: &NewGameState) -> Element<'_, Message> {
    let step_view: Element<'_, Message> = match ng.step {
        WizardStep::NameAndSeason => view_step_name_season(ng),
        WizardStep::ChooseMode => view_step_mode(ng),
        WizardStep::PickSchool => view_step_school(ng),
        WizardStep::ObserverConfirm => view_step_observer_confirm(ng),
    };

    let back_label = if ng.step == WizardStep::NameAndSeason { "← Main Menu" } else { "← Back" };
    let back_btn = button(text(back_label)).on_press(Message::PrevStep);

    let is_final = matches!(ng.step, WizardStep::PickSchool | WizardStep::ObserverConfirm);
    let next_label = if is_final { "Start Game →" } else { "Next →" };
    let next_msg = if is_final { Message::StartGame } else { Message::NextStep };
    let next_btn = button(text(next_label));
    let next_btn = if ng.can_advance() { next_btn.on_press(next_msg) } else { next_btn };

    let nav = container(
        row![back_btn, Space::new().width(Fill), next_btn].align_y(Vertical::Center),
    )
    .padding(Padding { top: 8., right: 20., bottom: 20., left: 20. });

    let card = container(
        column![
            container(step_view).width(Fill).height(Fill),
            nav,
        ],
    )
    .width(740)
    .height(580)
    .style(card_style);

    menu_shell(card.into())
}

// ---------------------------------------------------------------------------
// Step 1 — name + season
// ---------------------------------------------------------------------------

fn view_step_name_season(ng: &NewGameState) -> Element<'_, Message> {
    let content = column![
        text("New Dynasty").size(36),
        Space::new().height(36),
        text("Save Name"),
        Space::new().height(8),
        text_input("e.g. My Dynasty", &ng.save_name)
            .on_input(Message::SaveNameChanged)
            .width(Fill),
        Space::new().height(28),
        text("Starting Season"),
        Space::new().height(8),
        row![
            button(text("–")).on_press(Message::SeasonDecrement),
            container(text(ng.season.to_string()).size(20))
                .width(72)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center),
            button(text("+")).on_press(Message::SeasonIncrement),
        ]
        .spacing(12)
        .align_y(Vertical::Center),
        Space::new().height(8),
        text(format!("Range: {MIN_START_SEASON}–{MAX_START_SEASON}")).size(12),
    ]
    .spacing(0)
    .align_x(Horizontal::Left);

    container(container(content).width(420))
        .center(Fill)
        .padding(32)
        .into()
}

// ---------------------------------------------------------------------------
// Step 2 — choose save mode
// ---------------------------------------------------------------------------

fn view_step_mode(ng: &NewGameState) -> Element<'_, Message> {
    let modes: &[(SaveMode, &str, &str)] = &[
        (SaveMode::Normal, "Normal",
         "Standard dynasty. Recruiting rules, contract pressure, and all game systems apply."),
        (SaveMode::Commissioner, "Commissioner",
         "God mode. Edit ratings, financials, commitments, and hidden attributes at will."),
        (SaveMode::Observer, "Observer",
         "No school, no coach. Direct the simulation and watch all 365 programs unfold."),
    ];

    let mut mode_list = column![].spacing(12);
    for (mode, label, desc) in modes {
        let selected = ng.mode.as_ref() == Some(mode);
        let dot = if selected { "●  " } else { "○  " };
        let btn_body = column![
            text(format!("{dot}{label}")).size(17),
            text(*desc).size(13),
        ]
        .spacing(4);
        mode_list = mode_list.push(
            button(btn_body)
                .on_press(Message::ModeSelected(mode.clone()))
                .width(Fill),
        );
    }

    let content = column![
        text("Choose Your Mode").size(36),
        Space::new().height(36),
        mode_list,
    ]
    .align_x(Horizontal::Center);

    container(container(content).width(520))
        .center(Fill)
        .padding(32)
        .into()
}

// ---------------------------------------------------------------------------
// Step 3 — pick school
// ---------------------------------------------------------------------------

fn view_step_school(ng: &NewGameState) -> Element<'_, Message> {
    let heading = text("Choose Your School").size(32);

    let body: Element<'_, Message> = match &ng.world {
        None => {
            let err = ng.world_error.as_deref().unwrap_or("unknown error");
            text(format!("Could not load schools: {err}")).into()
        }
        Some(world) => {
            let mut confs: Vec<_> = world.conferences.iter().collect();
            confs.sort_by(|a, b| a.name.cmp(&b.name));

            let mut conf_list = column![].spacing(2);
            for conf in &confs {
                let selected = ng.selected_conf_id == Some(conf.id);
                let label = if selected { format!("▶  {}", conf.name) } else { conf.name.clone() };
                conf_list = conf_list.push(
                    button(text(label).size(13))
                        .on_press(Message::ConferenceSelected(conf.id))
                        .width(Fill),
                );
            }
            let conf_panel = scrollable(conf_list).height(Fill);

            let school_panel: Element<'_, Message> = match ng.selected_conf_id {
                None => container(text("← Pick a conference").size(13)).center(Fill).into(),
                Some(cid) => {
                    let mut schools: Vec<_> = world
                        .schools
                        .iter()
                        .filter(|s| s.conference_id == cid)
                        .collect();
                    schools.sort_by(|a, b| a.name.cmp(&b.name));

                    let cards = schools.iter().map(|school| {
                        school_card(school, ng.selected_school_id == Some(school.id))
                    });

                    scrollable(Grid::with_children(cards).columns(4).spacing(8))
                        .height(Fill)
                        .into()
                }
            };

            row![
                container(conf_panel).width(FillPortion(1)).height(Fill),
                container(school_panel).width(FillPortion(3)).height(Fill),
            ]
            .height(Fill)
            .into()
        }
    };

    column![heading, Space::new().height(16), body]
        .height(Fill)
        .padding(Padding { top: 24., right: 24., bottom: 0., left: 24. })
        .into()
}

// ---------------------------------------------------------------------------
// Step 3 (observer) — confirmation
// ---------------------------------------------------------------------------

fn view_step_observer_confirm(ng: &NewGameState) -> Element<'_, Message> {
    let season_start = ng.season - 1;
    let season_end = ng.season;

    let content = column![
        text("Observer Mode").size(36),
        Space::new().height(32),
        text(format!(
            "You will oversee the {} – {} season across all 365 programs.",
            season_start, season_end
        ))
        .size(16),
        Space::new().height(16),
        text("No school. No contract. No recruiting battles.").size(16),
        Space::new().height(8),
        text("You direct the simulation — the outcomes belong to the game.").size(16),
    ]
    .spacing(0)
    .align_x(Horizontal::Center);

    container(content).center(Fill).padding(32).into()
}

// ---------------------------------------------------------------------------
// School picker — grid card
// ---------------------------------------------------------------------------

fn school_card<'a>(
    school: &'a bbs_ch::model::school::School,
    selected: bool,
) -> Element<'a, Message> {
    let logo: Element<'a, Message> = match super::find_logo(&school.name, &school.abbreviation) {
        Some(path) if path.extension().is_some_and(|e| e == "svg") => {
            iced::widget::svg(iced::widget::svg::Handle::from_path(path))
                .width(72).height(72)
                .into()
        }
        Some(path) => {
            iced::widget::image(iced::widget::image::Handle::from_path(path))
                .width(72).height(72)
                .into()
        }
        None => {
            let color = super::hex_to_color(school.primary_color.as_str());
            container(Space::new().width(72).height(72))
                .style(move |_| iced::widget::container::Style {
                    background: Some(iced::Background::Color(color)),
                    border: iced::Border { radius: 8.0.into(), ..Default::default() },
                    ..Default::default()
                })
                .into()
        }
    };

    let display = if school.prefer_abbreviation { &school.abbreviation } else { &school.name };

    let card_body = column![
        logo,
        Space::new().height(4),
        text(display.as_str()).size(11),
    ]
    .align_x(Horizontal::Center)
    .spacing(0);

    button(card_body)
        .on_press(Message::SchoolSelected(school.id))
        .style(if selected { button::primary } else { button::secondary })
        .into()
}
