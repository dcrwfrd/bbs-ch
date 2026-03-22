use iced::widget::{container, row, scrollable, text};
use iced::{Background, Element, Fill};
use iced::widget::container::Style as CStyle;

use super::TeamTheme;
use crate::Message;

// Placeholder headlines — will be driven by MediaNarrative and live game data.
const ITEMS: &[&str] = &[
    "League News: Transfer portal opens April 15",
    "Rankings: Preseason Top 25 to be released in October",
    "Scores: Offseason — no games scheduled",
    "Portal: 847 players have entered the transfer portal",
    "Recruiting: Early signing period opens November 13",
];

pub fn view(theme: TeamTheme) -> Element<'static, Message> {
    let items: Vec<Element<'static, Message>> = ITEMS
        .iter()
        .flat_map(|s| {
            [
                text(*s).size(12).into(),
                text("  ·  ").size(12).into(),
            ]
        })
        .collect();

    let ticker_row = row(items).align_y(iced::alignment::Vertical::Center);

    container(
        scrollable(ticker_row)
            .direction(scrollable::Direction::Horizontal(
                scrollable::Scrollbar::new(),
            )),
    )
    .width(Fill)
    .height(28)
    .padding(iced::Padding { top: 4., bottom: 4., left: 8., right: 8. })
    .style(move |_| CStyle {
        background: Some(Background::Color(theme.ticker_bg)),
        ..Default::default()
    })
    .into()
}
