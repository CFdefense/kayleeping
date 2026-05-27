//! Remote ciphertext URLs and the [`Content`] bundle used by the GUI loader.
//!
//! See [`Content::fetch_blocking`]

use crate::encryption::decrypt_content_and_save;
use iced::widget::image;
use iced::{Size, window};
use reqwest;
use std::error::Error;

/// GitHub URLs for ciphertext that mirror [`encryption::IMG_SRC_PATH`] /
/// [`encryption::TXT_SRC_PATH`] (**`txt.enc`**, never `text.enc`).
pub const REMOTE_IMG_URL: &str =
    "https://raw.githubusercontent.com/CFdefense/kayleedrop/main/data/source/img.enc";
pub const REMOTE_TEXT_URL: &str =
    "https://raw.githubusercontent.com/CFdefense/kayleedrop/main/data/source/txt.enc";

/// Bundle of decrypted image handle, intrinsic PNG size (when applicable), and caption text.
#[derive(Clone, Debug)]
pub struct Content {
    pub image_handle: image::Handle,
    pub image_size: (u32, u32),
    pub text: String,
}

impl Default for Content {
    /// Returns a trivial 1×1 transparent raster handle and empty caption (useful only in tests or placeholders).
    fn default() -> Self {
        Self::new(
            image::Handle::from_rgba(1, 1, vec![0u8; 4]),
            (1, 1),
            String::new(),
        )
    }
}

impl Content {
    /// Wraps decrypted assets for rendering / window sizing hints.
    ///
    /// # Arguments
    ///
    /// - `image_size` — PNG IHDR geometry when known; iced uses this for native window sizing.
    /// - `image_handle` — typically produced by [`image::Handle::from_bytes`] after decryption.
    pub fn new(image_handle: image::Handle, image_size: (u32, u32), text: String) -> Self {
        Self {
            image_handle,
            image_size,
            text,
        }
    }

    /// Blocking-friendly async entry that mirrors the bundled GitHub payloads.
    ///
    /// Hits `img_hook`/`text_hook` with `GET`, rejects empty bodies, decrypts blobs using the ambient
    /// [`std::env::var`] `"PASSWORD"` (usually loaded via `dotenvy`), persists plaintext beneath
    /// [`crate::encryption::IMG_DEST_PATH`] / [`crate::encryption::TXT_DEST_PATH`], and finally wraps
    /// the results in [`Content`].
    ///
    /// # Errors
    ///
    /// Covers HTTP/stack failures when downloading, decryption/AEAD mismatches when the password does
    /// not match, UTF-8 issues in the decrypted caption, or filesystem persistence errors surfaced by
    /// [`crate::encryption::decrypt_content_and_save`].
    pub async fn fetch_blocking(
        img_hook: &str,
        text_hook: &str,
    ) -> Result<Content, Box<dyn Error>> {
        // get the encrpyted image from the remote store
        let img_response = reqwest::get(img_hook).await?;
        let img_body = img_response.bytes().await?;

        // get the encrypted text from the remote store
        let txt_response = reqwest::get(text_hook).await?;
        let txt_body = txt_response.bytes().await?;

        // err if we got empty data
        if img_body.is_empty() || txt_body.is_empty() {
            return Err("remote returned empty encrypted payload".into());
        }

        // decrypt and save the contents
        let result = decrypt_content_and_save(img_body.as_ref(), txt_body.as_ref())?;

        Ok(result)
    }

    /// Builds the initial iced window size around the decrypted bitmap plus space for caption text.
    ///
    /// Width and height come from [`Content::image_size`]. Adds a fixed vertical band for one line of
    /// UI text so the inner client area fits image + caption without clipping.
    ///
    /// See also [`iced::window::Settings`].
    pub fn into_window(&self) -> window::Settings {
        const SPACING: f32 = 8.0;
        const CAPTION_ROWS: f32 = 26.0;
        let (w, h) = self.image_size;
        window::Settings {
            size: Size::new(w as f32, h as f32 + SPACING + CAPTION_ROWS),
            ..window::Settings::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_content_minimal_dimensions() {
        let c = Content::default();
        assert_eq!(c.image_size, (1, 1));
        assert!(c.text.is_empty());
    }

    #[test]
    fn new_preserves_fields() {
        let h = image::Handle::from_bytes(vec![9, 9, 9]);
        let c = Content::new(h.clone(), (10, 20), "a".into());
        assert_eq!(c.image_size, (10, 20));
        assert_eq!(c.text, "a");
    }
}
