use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, column, container, row, text, Space};
use iced::{Background, Color, Element, Fill};
use iced::widget::container::Style as CStyle;

use bbs_ch::model::school::School;

use super::{InGameState, NavItem, SCALE_STEPS_PCT, TeamTheme};
use crate::Message;

fn bg(color: Color) -> impl Fn(&iced::Theme) -> CStyle {
    move |_| CStyle {
        background: Some(Background::Color(color)),
        ..Default::default()
    }
}
// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Renders the full top section: the nav bar and any open expand panels.
pub fn view<'a>(
    state: &'a InGameState,
    school: Option<&'a School>,
    scale_idx: usize,
    theme: TeamTheme,
) -> Element<'a, Message> {
    let bar = nav_bar(state, school, theme);

    let mut col = column![bar];

    if state.schedule_open {
        col = col.push(schedule_panel(theme));
    }
    if state.advance_open {
        col = col.push(advance_panel(theme));
    }
    if state.game_menu_open {
        col = col.push(game_menu_panel(scale_idx, theme));
    }

    container(col).width(Fill).style(bg(theme.bar_bg)).into()
}

// ---------------------------------------------------------------------------
// Nav bar row
// ---------------------------------------------------------------------------

fn nav_bar<'a>(state: &'a InGameState, school: Option<&'a School>, theme: TeamTheme) -> Element<'a, Message> {
    let left_anchor = team_identity(school, theme);
    let team_nav = team_nav_buttons(state);
    let hub = dashboard_hub(state, theme);
    let league_nav = league_nav_buttons(state);
    let right_anchor = game_menu_button(state);

    row![
        left_anchor,
        Space::new().width(8),
        team_nav,
        Space::new().width(Fill),
        hub,
        Space::new().width(Fill),
        league_nav,
        Space::new().width(8),
        right_anchor,
    ]
    .align_y(Vertical::Center)
    .padding(6)
    .into()
}

// ---------------------------------------------------------------------------
// Left anchor — team identity
// ---------------------------------------------------------------------------

fn team_identity<'a>(school: Option<&'a School>, theme: TeamTheme) -> Element<'a, Message> {
    let (color_hex, name, nickname, abbrev, prefer_abbrev) = match school {
        Some(s) => (
            s.primary_color.as_str(),
            s.name.as_str(),
            s.nickname.as_str(),
            s.abbreviation.as_str(),
            s.prefer_abbreviation,
        ),
        None => ("#444444", "Observer", "Mode", "", false),
    };

    let display = if prefer_abbrev { abbrev } else { name };
    let _ = color_hex; // color comes from theme now

    let color_swatch = container(Space::new().width(36).height(36))
        .style(move |_| CStyle {
            background: Some(Background::Color(theme.primary)),
            border: iced::Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        });

    let label_col = column![
        text(display).size(14),
        text(nickname).size(11),
        text("0-0 (0-0)").size(11),
    ]
    .spacing(1);

    row![color_swatch, Space::new().width(8), label_col]
        .align_y(Vertical::Center)
        .into()
}

// ---------------------------------------------------------------------------
// Team nav buttons
// ---------------------------------------------------------------------------

fn team_nav_buttons(state: &InGameState) -> Element<'_, Message> {
    const TEAM_NAV: &[NavItem] = &[
        NavItem::Roster,
        NavItem::Recruiting,
        NavItem::Schedule,
        NavItem::Tactics,
        NavItem::Staff,
    ];

    let mut r = row![].spacing(2).align_y(Vertical::Center);
    for item in TEAM_NAV {
        r = r.push(nav_button(item, &state.active_nav));
    }
    r.into()
}

// ---------------------------------------------------------------------------
// Center — dashboard hub (raised)
// ---------------------------------------------------------------------------

fn dashboard_hub(state: &InGameState, theme: TeamTheme) -> Element<'_, Message> {
    let season_start = state.game.season - 1;
    let season_end_short = state.game.season % 100;
    let season_label = format!("{}-{:02}", season_start, season_end_short);

    let date_str = state.game.current_date.to_string();
    let phase_str = state.game.phase.label();

    let schedule_btn = button(text("📅").size(14))
        .on_press(Message::SchedulePanelToggle)
        .style(if state.schedule_open { button::primary } else { button::secondary });

    let advance_btn = button(text("▶▶").size(14))
        .on_press(Message::AdvancePanelToggle)
        .style(if state.advance_open { button::primary } else { button::secondary });

    let center_text = column![
        text(date_str).size(15),
        text(format!("{season_label}  ·  {phase_str}")).size(11),
    ]
    .spacing(2)
    .align_x(Horizontal::Center);

    let hub_row = row![
        schedule_btn,
        Space::new().width(10),
        center_text,
        Space::new().width(10),
        advance_btn,
    ]
    .align_y(Vertical::Center);

    container(hub_row)
        .style(bg(theme.hub_bg))
        .padding(iced::Padding { top: 10., bottom: 10., left: 16., right: 16. })
        .into()
}

// ---------------------------------------------------------------------------
// League nav buttons
// ---------------------------------------------------------------------------

fn league_nav_buttons(state: &InGameState) -> Element<'_, Message> {
    const LEAGUE_NAV: &[NavItem] = &[
        NavItem::Standings,
        NavItem::Rankings,
        NavItem::Scores,
        NavItem::Portal,
        NavItem::LeagueNews,
    ];

    let mut r = row![].spacing(2).align_y(Vertical::Center);
    for item in LEAGUE_NAV {
        r = r.push(nav_button(item, &state.active_nav));
    }
    r.into()
}

// ---------------------------------------------------------------------------
// Right anchor — game menu button
// ---------------------------------------------------------------------------

fn game_menu_button(state: &InGameState) -> Element<'_, Message> {
    button(text("☰").size(18))
        .on_press(Message::GameMenuToggle)
        .style(if state.game_menu_open { button::primary } else { button::secondary })
        .into()
}

// ---------------------------------------------------------------------------
// Shared nav button
// ---------------------------------------------------------------------------

fn nav_button<'a>(item: &'a NavItem, active: &Option<NavItem>) -> Element<'a, Message> {
    let is_active = active.as_ref() == Some(item);
    button(text(item.label()).size(13))
        .on_press(Message::InGameNav(item.clone()))
        .style(if is_active { button::primary } else { button::text })
        .into()
}

// ---------------------------------------------------------------------------
// Expand panels
// ---------------------------------------------------------------------------

fn schedule_panel<'a>(theme: TeamTheme) -> Element<'a, Message> {
    container(text("Schedule viewer coming soon.").size(13))
        .padding(16)
        .width(Fill)
        .style(bg(theme.panel_bg))
        .into()
}

fn advance_panel<'a>(theme: TeamTheme) -> Element<'a, Message> {
    let panel = column![
        button(text("Advance 1 Day").size(13))
            .on_press(Message::AdvanceDay),
        button(text("Advance to Next Event").size(13))
            .on_press(Message::AdvanceToNextEvent),
        button(text("Simulate to End of Phase").size(13))
            .on_press(Message::AdvanceToEndOfPhase),
    ]
    .spacing(6)
    .padding(12);

    container(container(panel).style(bg(theme.panel_bg)))
        .align_x(Horizontal::Right)
        .width(Fill)
        .into()
}

fn game_menu_panel<'a>(scale_idx: usize, theme: TeamTheme) -> Element<'a, Message> {
    let pct = SCALE_STEPS_PCT[scale_idx];
    let scale_row = row![
        button(text("−").size(13))
            .on_press(Message::UiScaleDecrease)
            .style(if scale_idx == 0 { button::secondary } else { button::text }),
        container(text(format!("{pct}%")).size(13))
            .align_x(Horizontal::Center)
            .padding(iced::Padding { top: 0., bottom: 0., left: 6., right: 6. }),
        button(text("+").size(13))
            .on_press(Message::UiScaleIncrease)
            .style(if scale_idx + 1 == SCALE_STEPS_PCT.len() { button::secondary } else { button::text }),
    ]
    .align_y(Vertical::Center);

    let panel = column![
        button(text("Save Game").size(13)),
        Space::new().height(4),
        iced::widget::rule::horizontal(1),
        Space::new().height(2),
        text("Game Settings").size(11),
        button(text("Toggle Ticker").size(13))
            .on_press(Message::TickerToggle),
        row![
            text("UI Scale").size(13),
            Space::new().width(Fill),
            scale_row,
        ]
        .align_y(Vertical::Center),
        Space::new().height(4),
        iced::widget::rule::horizontal(1),
        Space::new().height(2),
        button(text("Main Menu").size(13))
            .on_press(Message::QuitToMenu)
            .style(button::danger),
        button(text("Quit to Desktop").size(13))
            .on_press(Message::QuitToDesktop)
            .style(button::danger),
    ]
    .spacing(4)
    .padding(12);

    container(container(panel).style(bg(theme.panel_bg)))
        .align_x(Horizontal::Right)
        .width(Fill)
        .into()
}
