mod app;
mod config;
mod content;
mod encryption;
mod error;

use app::{AppState, MyApp};
use encryption::encrypt_content_and_write;
use error::AppError;
use std::env::args;
use std::error::Error;
use std::process;
use std::sync::Mutex;
use tokio::runtime::Runtime;

use crate::content::{Content, REMOTE_IMG_URL, REMOTE_TEXT_URL};

/// Program entrypoint: routes to GUI mode (`argv.len() == 1`), encrypt CLI (`3` arguments), or help + exit `64`.
fn main() -> Result<(), Box<dyn Error>> {
    // next get the arguments provided in the call
    let args: Vec<String> = args().collect();
    let bin = args.first().map(String::as_str).unwrap_or("kayleeping");

    match args.len() {
        1 => run_gui(),
        3 => {
            encrypt_content_and_write(&args[1], &args[2])?;
            Ok(())
        }
        _ => {
            print_encrypt_usage(bin);
            process::exit(64);
        }
    }
}

/// Runs interactive mode, displaying either content or errors in the GUI.
///
/// Always shows a GUI window - either with successfully loaded content or with
/// detailed error information and recovery suggestions. This ensures users always
/// get feedback about what went wrong instead of silent failures.
///
/// # Errors
///
/// Returns boxed errors mainly from Tokio runtime creation or iced startup failures.
/// Application-level errors (password missing, network issues, decryption failures)
/// are displayed in the GUI rather than returned.
fn run_gui() -> Result<(), Box<dyn Error>> {
    let rt = Runtime::new()?;

    // Attempt to load content, capturing any errors for GUI display
    let app_state = match load_content_or_error(&rt) {
        Ok(content) => AppState::Content(content),
        Err(error) => AppState::Error(error),
    };

    // Determine window settings based on state
    let (iced_window_layout, _window_title) = match &app_state {
        AppState::Content(content) => (content.into_window(), app::TITLE.to_string()),
        AppState::Error(_) => (
            iced::window::Settings {
                size: iced::Size::new(800.0, 600.0),
                ..iced::window::Settings::default()
            },
            format!("{} - Error", app::TITLE),
        ),
    };

    // Create the GUI app with the determined state
    let initial_state = Mutex::new(Some(app_state));
    let iced_window = iced::application(
        move || {
            let state = initial_state
                .lock()
                .expect("mutex poisoned")
                .take()
                .expect("iced boot invoked more than once");

            match state {
                AppState::Content(content) => MyApp::new(content),
                AppState::Error(error) => MyApp::new_with_error(error),
            }
        },
        MyApp::update,
        MyApp::view,
    )
    .window(iced_window_layout)
    .centered();

    // Run the GUI app
    iced_window
        .title(|state: &MyApp| state.title.clone())
        .run()
        .map_err(|e| -> Box<dyn Error> { e.into() })?;

    Ok(())
}

/// Attempts to load and decrypt content, returning structured errors on failure.
///
/// Loads PASSWORD from environment or .env file, fetches remote content, and decrypts it.
/// All failures are converted to user-friendly AppError variants.
fn load_content_or_error(rt: &Runtime) -> Result<Content, AppError> {
    // Load PASSWORD from environment or .env file
    // This will check environment first, then try to load from .env
    let _password = config::load_password().map_err(|e| {
        // If it's already an AppError, extract it; otherwise create a generic one
        if let Some(app_err) = e.downcast_ref::<AppError>() {
            app_err.clone()
        } else {
            AppError::from_error(e.as_ref(), "Failed to load PASSWORD")
        }
    })?;

    // Fetch and decrypt content
    rt.block_on(Content::fetch_blocking(REMOTE_IMG_URL, REMOTE_TEXT_URL))
        .map_err(|e| {
            // Try to categorize the error
            let error_msg = e.to_string();
            if error_msg.contains("PASSWORD") || error_msg.contains("decrypt") {
                AppError::DecryptionError { details: error_msg }
            } else if error_msg.contains("network")
                || error_msg.contains("fetch")
                || error_msg.contains("request")
            {
                AppError::NetworkError {
                    url: REMOTE_IMG_URL.to_string(),
                    details: error_msg,
                }
            } else if error_msg.contains("truncated") || error_msg.contains("invalid") {
                AppError::InvalidCiphertext { details: error_msg }
            } else {
                AppError::from_error(e.as_ref(), "Failed to load content")
            }
        })
}

///
/// The caller terminates the process with code `64` for invalid arity; this helper only emits the
/// message body.
fn print_encrypt_usage(bin: &str) {
    eprintln!(
        "\
Usage:
  {bin}
      Run the GUI (remote fetch/decrypt gates apply).

  {bin} <IMAGE_PATH> <TEXT>
      Read the image from disk and encrypt it with TEXT; write ciphertext to bundled paths \
(requires PASSWORD in the environment or a `.env` file).
",
        bin = bin,
    );
}
