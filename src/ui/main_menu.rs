use iced::alignment::Horizontal;
use iced::widget::{button, column, container, text, Space};
use iced::{Element, Fill, Padding};

use super::theme;
use crate::Message;

pub fn view() -> Element<'static, Message> {
    let buttons = column![
        button(text("New Game").size(16))
            .on_press(Message::NewGamePressed)
            .width(Fill),
        button(text("Load Game").size(16)).width(Fill),
        button(text("League Builder").size(16))
            .on_press(Message::LeagueBuilderOpen)
            .width(Fill),
        button(text("Settings").size(16)).width(Fill),
        button(text("Quit").size(16))
            .on_press(Message::QuitPressed)
            .width(Fill),
    ]
    .spacing(10);

    let card = container(buttons)
        .width(300)
        .padding(Padding { top: 28., right: 28., bottom: 28., left: 28. })
        .style(card_style);

    menu_shell(card.into())
}

/// The game title shown above both the main menu and new game wizard.
pub fn game_title() -> Element<'static, Message> {
    column![
        text("Blue Blood Sports").size(48),
        text("College Hoops").size(24),
    ]
    .spacing(4)
    .align_x(Horizontal::Center)
    .into()
}

/// Shared card style used by the main menu and new game wizard.
pub fn card_style(t: &iced::Theme) -> iced::widget::container::Style {
    theme::card_style(t)
}

/// Wraps `content` in the title + card chrome and centers the whole thing.
pub fn menu_shell<'a>(card: Element<'a, Message>) -> Element<'a, Message> {
    let layout = column![
        game_title(),
        Space::new().height(28),
        card,
    ]
    .align_x(Horizontal::Center);

    container(layout).center(Fill).style(theme::shell_style).into()
}
