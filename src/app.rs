//! Iced GUI shell: [`MyApp`], [`update`], [`view`].

use crate::content::Content;
use iced::{
    Element, Length,
    alignment::{Horizontal, Vertical},
    widget::{column, container, image::Image, text},
};

/// Default window title surfaced in iced window chrome.
///
/// Duplicated verbatim into [`MyApp::title`] during construction.
pub const TITLE: &str = "kayleedrop";

/// Holds UI state fetched before the iced runtime spins up.
///
/// Fields are public so `main` can wire [`iced::application`] title hooks.
pub struct MyApp {
    pub title: String,
    pub content: Content,
}

impl MyApp {
    /// Constructs UI state ahead of spawning iced.
    ///
    /// Copies the shared [`TITLE`] into `title` and stores the decrypted [`crate::content::Content`]
    /// bundle for rendering. Does not fetch network data or resize windows.
    pub fn new(content: Content) -> Self {
        Self {
            title: TITLE.to_string(),
            content,
        }
    }

    /// Renders stacked image + text inside a padded, centered iced container tree.
    ///
    /// Uses fill lengths so the widgets stay centered regardless of fractional window chrome; callers
    /// are expected to size the outer window tightly around the raster when desired.
    pub fn view(&self) -> Element<'_, ()> {
        container(
            column![
                Image::new(self.content.image_handle.clone()),
                text(self.content.text.as_str()),
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

    /// Empty update function to sastisfy iced
    /// However our application is a static gui and such function is not needed
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
        assert_eq!(app.content.text, c.text);
    }
}
