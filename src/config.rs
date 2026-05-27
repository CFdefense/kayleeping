///! Config Helper Functions and Setup
use std::path::PathBuf;
use std::{env, error::Error, fs};

/// Function to resolve and initialize .env directories for PASSWORD
///
/// Will fail if password not set
///
/// Will create directories if not present
pub fn load_password() -> Result<String, Box<dyn Error>> {
    // use the config dirs to get the default dir across many systems
    // on MACOS ~/Library/Application Support/
    // on Linux ~/.local/share/
    let app_dir = app_data_dir()?;

    // then get the env path
    let env_path = app_dir.join(".env");

    // finally read the .env file variables from the final path
    let _ = dotenvy::from_path(&env_path);

    Ok(env::var("PASSWORD")?)
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
