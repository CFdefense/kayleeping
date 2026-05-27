//! Error types and GUI error display for user feedback.
//!
//! Provides structured error types with user-friendly messages and recovery suggestions.

use std::error::Error;
use std::fmt;

/// Application-specific error types with user-friendly messages and recovery suggestions.
#[derive(Debug, Clone)]
pub enum AppError {
    /// PASSWORD environment variable is not set
    PasswordMissing,
    /// Failed to fetch remote content
    NetworkError { url: String, details: String },
    /// Failed to decrypt content (wrong password or corrupt data)
    DecryptionError { details: String },
    /// Failed to read or write files
    FileSystemError { path: String, details: String },
    /// Invalid or corrupt encrypted data
    InvalidCiphertext { details: String },
    /// UTF-8 decoding failed
    EncodingError { details: String },
    /// Generic error with custom message
    Generic {
        message: String,
        trace: Option<String>,
    },
}

impl AppError {
    /// Returns a user-friendly error message
    pub fn user_message(&self) -> String {
        match self {
            AppError::PasswordMissing => {
                "Password Not Set\n\nThe PASSWORD environment variable is required but not found."
                    .to_string()
            }
            AppError::NetworkError { url, details } => {
                format!(
                    "Network Error\n\nFailed to fetch content from:\n{}\n\nDetails: {}",
                    url, details
                )
            }
            AppError::DecryptionError { details } => {
                format!(
                    "Decryption Failed\n\nCould not decrypt the content. This usually means the password is incorrect or the data is corrupted.\n\nDetails: {}",
                    details
                )
            }
            AppError::FileSystemError { path, details } => {
                format!(
                    "File System Error\n\nFailed to access:\n{}\n\nDetails: {}",
                    path, details
                )
            }
            AppError::InvalidCiphertext { details } => {
                format!(
                    "Invalid Data Format\n\nThe encrypted data appears to be corrupted or in an unexpected format.\n\nDetails: {}",
                    details
                )
            }
            AppError::EncodingError { details } => {
                format!(
                    "Text Encoding Error\n\nThe decrypted text contains invalid UTF-8 characters.\n\nDetails: {}",
                    details
                )
            }
            AppError::Generic { message, trace } => {
                if let Some(t) = trace {
                    format!("Error\n\n{}\n\nTrace:\n{}", message, t)
                } else {
                    format!("Error\n\n{}", message)
                }
            }
        }
    }

    /// Returns recovery suggestions for the user
    pub fn recovery_suggestions(&self) -> Vec<String> {
        match self {
            AppError::PasswordMissing => vec![
                "Set the PASSWORD environment variable".to_string(),
                "Create a .env file in your app data directory with PASSWORD=your_password".to_string(),
                format!("App data directory: {}", 
                    dirs::data_local_dir()
                        .map(|p| p.join("kayleedrop").display().to_string())
                        .unwrap_or_else(|| "~/.local/share/kayleedrop (Linux) or ~/Library/Application Support/kayleedrop (macOS)".to_string())
                ),
            ],
            AppError::NetworkError { .. } => vec![
                "Check your internet connection".to_string(),
                "Verify the remote URLs are accessible".to_string(),
                "Try again in a few moments".to_string(),
            ],
            AppError::DecryptionError { .. } => vec![
                "Verify your PASSWORD is correct".to_string(),
                "Check if the encrypted files have been updated".to_string(),
                "Ensure the data hasn't been corrupted during transfer".to_string(),
            ],
            AppError::FileSystemError { .. } => vec![
                "Check file permissions".to_string(),
                "Ensure the directory exists and is writable".to_string(),
                "Verify you have sufficient disk space".to_string(),
            ],
            AppError::InvalidCiphertext { .. } => vec![
                "Re-download the encrypted files".to_string(),
                "Verify the source URLs are correct".to_string(),
                "Check if the encryption format has changed".to_string(),
            ],
            AppError::EncodingError { .. } => vec![
                "The encrypted text may be corrupted".to_string(),
                "Try re-encrypting the content with valid UTF-8 text".to_string(),
            ],
            AppError::Generic { .. } => vec![
                "Check the error details above".to_string(),
                "Try restarting the application".to_string(),
            ],
        }
    }

    /// Creates an AppError from a generic error with optional trace
    pub fn from_error(error: &dyn Error, context: &str) -> Self {
        let message = format!("{}: {}", context, error);
        let mut trace = String::new();
        let mut source = error.source();
        while let Some(err) = source {
            trace.push_str(&format!("\nCaused by: {}", err));
            source = err.source();
        }

        AppError::Generic {
            message,
            trace: if trace.is_empty() { None } else { Some(trace) },
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.user_message())
    }
}

impl Error for AppError {}

/// Convert from std::env::VarError
impl From<std::env::VarError> for AppError {
    fn from(_: std::env::VarError) -> Self {
        AppError::PasswordMissing
    }
}

/// Convert from std::string::FromUtf8Error
impl From<std::string::FromUtf8Error> for AppError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        AppError::EncodingError {
            details: err.to_string(),
        }
    }
}
