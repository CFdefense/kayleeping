//! Iced GUI shell: [`MyApp`], [`update`], [`view`].

use crate::content::Content;
use crate::error::AppError;
use iced::{
    Element, Length,
    alignment::{Horizontal, Vertical},
    widget::{column, container, image::Image, scrollable, text},
};

/// Default window title surfaced in iced window chrome.
///
/// Duplicated verbatim into [`MyApp::title`] during construction.
pub const TITLE: &str = "kayleeping";

/// Holds UI state - either successfully loaded content or an error to display.
///
/// Fields are public so `main` can wire [`iced::application`] title hooks.
pub struct MyApp {
    pub title: String,
    pub state: AppState,
}

/// Application state - either showing content or an error
#[derive(Clone)]
pub enum AppState {
    /// Successfully loaded and decrypted content
    Content(Content),
    /// Error occurred during loading/decryption
    Error(AppError),
}

impl MyApp {
    /// Constructs UI state with successfully loaded content.
    ///
    /// Copies the shared [`TITLE`] into `title` and stores the decrypted [`crate::content::Content`]
    /// bundle for rendering. Does not fetch network data or resize windows.
    pub fn new(content: Content) -> Self {
        Self {
            title: TITLE.to_string(),
            state: AppState::Content(content),
        }
    }

    /// Constructs UI state with an error to display.
    ///
    /// Used when content loading/decryption fails, allowing the GUI to show
    /// detailed error information and recovery suggestions to the user.
    pub fn new_with_error(error: AppError) -> Self {
        Self {
            title: format!("{} - Error", TITLE),
            state: AppState::Error(error),
        }
    }

    /// Renders either content or error display based on application state.
    ///
    /// For successful content: displays stacked image + text inside a padded, centered container.
    /// For errors: displays error message, details, and recovery suggestions in a scrollable view.
    pub fn view(&self) -> Element<'_, ()> {
        match &self.state {
            AppState::Content(content) => self.view_content(content),
            AppState::Error(error) => self.view_error(error),
        }
    }

    /// Renders successfully loaded content (image + text).
    fn view_content<'a>(&'a self, content: &'a Content) -> Element<'a, ()> {
        container(
            column![
                Image::new(content.image_handle.clone()),
                text(content.text.as_str()),
            ]
            .spacing(8)
            .align_x(Horizontal::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into()
    }

    /// Renders error information with recovery suggestions.
    fn view_error<'a>(&'a self, error: &'a AppError) -> Element<'a, ()> {
        let error_message = text(error.user_message()).size(16).line_height(1.5);

        let suggestions = error.recovery_suggestions();
        let mut suggestion_widgets = vec![text("Recovery Suggestions:").size(18).into()];

        for (i, suggestion) in suggestions.iter().enumerate() {
            suggestion_widgets.push(
                text(format!("{}. {}", i + 1, suggestion))
                    .size(14)
                    .line_height(1.4)
                    .into(),
            );
        }

        let content = column![
            text("⚠️ Application Error").size(24),
            error_message,
            column(suggestion_widgets).spacing(8).padding(20.0),
        ]
        .spacing(20)
        .padding(30)
        .max_width(700);

        container(scrollable(content))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .into()
    }

    /// Empty update function to satisfy iced.
    /// However our application is a static GUI and such function is not needed.
    pub fn update(&mut self, _message: ()) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sets_title_constant() {
        let c = Content::default();
        let app = MyApp::new(c.clone());
        assert_eq!(app.title, TITLE);
        match &app.state {
            AppState::Content(content) => assert_eq!(content.text, c.text),
            _ => panic!("Expected Content state"),
        }
    }

    #[test]
    fn new_with_error_sets_error_state() {
        let error = AppError::PasswordMissing;
        let app = MyApp::new_with_error(error.clone());
        assert!(app.title.contains("Error"));
        match &app.state {
            AppState::Error(_) => {}
            _ => panic!("Expected Error state"),
        }
    }
}
