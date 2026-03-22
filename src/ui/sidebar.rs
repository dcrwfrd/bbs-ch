use iced::widget::{button, column, container, scrollable, text};
use iced::{Background, Element, Fill, FillPortion};

use super::{InGameState, NavItem, TeamTheme};
use crate::Message;

pub fn view<'a>(state: &'a InGameState, nav: &'a NavItem, theme: TeamTheme) -> Element<'a, Message> {
    let sub_items = nav.sub_items();
    let mut list = column![].spacing(2);

    for (i, &label) in sub_items.iter().enumerate() {
        let is_active = state.active_sub == Some(i);
        list = list.push(
            button(text(label).size(13))
                .on_press(Message::InGameSubNav(i))
                .style(if is_active { button::primary } else { button::text })
                .width(Fill),
        );
    }

    container(
        scrollable(
            column![
                text(nav.label()).size(15),
                iced::widget::rule::horizontal(1),
                list,
            ]
            .spacing(8)
            .padding(12),
        )
        .height(Fill),
    )
    .width(FillPortion(1))
    .height(Fill)
    .style(move |_| iced::widget::container::Style {
        background: Some(Background::Color(theme.sidebar_bg)),
        ..Default::default()
    })
    .into()
}
