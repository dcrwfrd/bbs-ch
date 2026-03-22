use iced::alignment::Horizontal;
use iced::widget::{column, container, text};
use iced::{Element, Fill, FillPortion};

use super::InGameState;
use crate::Message;

pub fn view(state: &InGameState) -> Element<'_, Message> {
    let label = match &state.active_nav {
        None => "Select a section from the navigation bar above.".to_string(),
        Some(nav) => {
            let section = nav.label();
            let sub = state
                .active_sub
                .and_then(|i| nav.sub_items().get(i).copied())
                .unwrap_or("Overview");
            format!("{section}  /  {sub}")
        }
    };

    container(placeholder(label))
        .width(FillPortion(4))
        .height(Fill)
        .into()
}

fn placeholder(label: String) -> Element<'static, Message> {
    container(
        column![
            text(label).size(20),
            text("Content coming soon.").size(13),
        ]
        .spacing(8)
        .align_x(Horizontal::Center),
    )
    .center(Fill)
    .into()
}
