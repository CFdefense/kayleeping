mod app;
mod content;
mod encryption;

use app::MyApp;
use encryption::encrypt_content_and_write;
use std::env::args;
use std::env::var;
use std::error::Error;
use std::process;
use std::sync::Mutex;
use tokio::runtime::Runtime;

use crate::content::{Content, REMOTE_IMG_URL, REMOTE_TEXT_URL};

/// Program entrypoint: routes to GUI mode (`argv.len() == 1`), encrypt CLI (`3` arguments), or help + exit `64`.
fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();

    let args: Vec<String> = args().collect();
    let bin = args.first().map(String::as_str).unwrap_or("kayleedrop");

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

/// Runs interactive mode after remote / local consistency checks succeed.
///
/// Exits quietly when [`content::remote_plaintext_matches_destination`] reports no new content, when
/// `PASSWORD` is missing, or when the remote fetch/decrypt fails. Otherwise builds an iced
/// application seeded with decrypted [`Content`] and blocks until the window closes.
///
/// # Errors
///
/// Returns boxed errors mainly from Tokio blocking on the runtime or iced startup failures.
fn run_gui() -> Result<(), Box<dyn Error>> {
    let rt = Runtime::new()?;

    // need PASSWORD to fetch/decrypt
    if var("PASSWORD").is_err() {
        eprintln!("PASSWORD environment variable is not set; not starting GUI.");
        return Ok(());
    }

    // fetch + decrypt; any failure ⇒ no GUI
    let content = match rt.block_on(Content::fetch_blocking(REMOTE_IMG_URL, REMOTE_TEXT_URL)) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("failed to load remote content: {e}");
            return Ok(());
        }
    };

    // create the gui app
    let iced_window_layout = content.into_window();
    let initial_content = Mutex::new(Some(content));
    let iced_window = iced::application(
        move || {
            MyApp::new(
                initial_content
                    .lock()
                    .expect("mutex poisoned")
                    .take()
                    .expect("iced boot invoked more than once"),
            )
        },
        MyApp::update,
        MyApp::view,
    )
    .window(iced_window_layout)
    .centered();

    // run the gui app
    iced_window
        .title(|state: &MyApp| state.title.clone())
        .run()
        .map_err(|e| -> Box<dyn Error> { e.into() })?;

    Ok(())
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
