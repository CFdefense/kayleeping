///! Config Helper Functions and Setup
use std::path::PathBuf;
use std::{env, error::Error, fs};

use crate::error::AppError;

/// Function to resolve and initialize .env directories for PASSWORD
///
/// Will fail if password not set
///
/// Will create directories if not present
pub fn load_password() -> Result<String, Box<dyn Error>> {
    // First check if PASSWORD is already set in the environment
    if let Ok(password) = env::var("PASSWORD") {
        return Ok(password);
    }

    // use the config dirs to get the default dir across many systems
    // on MACOS ~/Library/Application Support/
    // on Linux ~/.local/share/
    let app_dir = app_data_dir()?;

    // then get the env path
    let env_path = app_dir.join(".env");

    // finally read the .env file variables from the final path
    if env_path.exists() {
        dotenvy::from_path(&env_path).map_err(|e| {
            let err: Box<dyn Error> = format!(
                "Failed to load .env file from {}: {}",
                env_path.display(),
                e
            )
            .into();
            err
        })?;
    }

    env::var("PASSWORD").map_err(|_| {
        let err: Box<dyn Error> = AppError::Generic {
            message: format!(
                "PASSWORD not found in environment or .env file.\nChecked: {}",
                env_path.display()
            ),
            trace: Some(format!(
                ".env file exists: {}\nCurrent working directory: {:?}",
                env_path.exists(),
                env::current_dir().ok()
            )),
        }
        .into();
        err
    })
}

/// Function to resolve the general kayleedrop dir
///
/// Will create all directories if needed
///
/// Returns:
///     PathBuf: the path to the directory
pub fn app_data_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let dir = dirs::data_local_dir()
        .expect("failed to locate app data dir")
        .join("kayleedrop");

    fs::create_dir_all(&dir)?;

    Ok(dir)
}

/// Function to resolve the encrypted img path
///
/// Returns:
///     PathBuf: the path to the directory
pub fn encrypted_img_path() -> PathBuf {
    PathBuf::from("data/source/img.enc")
}

/// Function to resolve the encrypted txt path
///
/// Returns:
///     PathBuf: the path to the directory
pub fn encrypted_txt_path() -> PathBuf {
    PathBuf::from("data/source/txt.enc")
}

/// Function to resolve the decrypted img path
///
/// Returns:
///     PathBuf: the path to the directory
pub fn decrypted_img_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    Ok(app_data_dir()?.join("img.png"))
}

/// Function to resolve the decrypted txt path
///
/// Returns:
///     PathBuf: the path to the directory
pub fn decrypted_txt_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    Ok(app_data_dir()?.join("txt.out"))
}
