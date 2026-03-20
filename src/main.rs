mod data;
mod model;
mod storage;

use iced::{Element, Fill, Theme};
use iced::widget::{container, text};

fn main() -> iced::Result {
    // iced 0.14: first argument is a boot function that produces the initial
    // state, not a title string. The title is set separately on the builder.
    iced::application(App::default, App::update, App::view)
        .title("Blue Blood Sports: College Hoops")
        .theme(App::theme)
        .run()
}

/// Top-level application state.
///
/// Empty for now — this is the anchor that the iced runtime drives.
/// Game state (roster, recruiting, season progress) will be added here
/// as fields over time.
#[derive(Default)]
struct App;

/// All events the application can respond to.
///
/// Empty for now since our scaffold has no interactive elements.
/// Each future feature (recruiting action, menu navigation, etc.)
/// gets a variant here.
#[derive(Debug, Clone)]
enum Message {}

impl App {
    /// Returns the application theme.
    ///
    /// Defined as a method rather than a closure because iced's `.theme()`
    /// builder requires a function that is generic over the state's lifetime,
    /// which closures can't satisfy without explicit annotation.
    fn theme(&self) -> Theme {
        Theme::Dark
    }

    /// Receives a Message and returns the updated application state.
    ///
    /// iced calls this whenever the user interacts with the UI.
    /// For now there are no messages, so this is intentionally unreachable.
    fn update(&mut self, _message: Message) {}

    /// Renders the current application state as a tree of widgets.
    ///
    /// iced calls this after every update to rebuild the UI.
    fn view(&self) -> Element<'_, Message> {
        // container fills the window and centers its child both axes.
        container(text("Blue Blood Sports: College Hoops").size(28))
            .center(Fill)
            .into()
    }
}
