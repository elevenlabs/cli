//! WIF error categories from the client behavior spec; identical in every SDK.

use std::fmt;

use fern_cli_sdk::error::CliError;

pub const CONFIG_INVALID: &str = "config_invalid";
pub const SOURCE_TOKEN_UNAVAILABLE: &str = "source_token_unavailable";
pub const EXCHANGE_REJECTED: &str = "exchange_rejected";
pub const EXCHANGE_UNAVAILABLE: &str = "exchange_unavailable";

/// A WIF failure; callers branch on `category`, never on the message.
#[derive(Debug, Clone)]
pub struct WifError {
    pub category: &'static str,
    pub message: String,
    pub error: Option<String>,
    pub error_description: Option<String>,
}

impl WifError {
    fn new(category: &'static str, cause: impl fmt::Display) -> Self {
        Self {
            category,
            message: format!("ElevenLabs WIF {category}: {cause}"),
            error: None,
            error_description: None,
        }
    }
}

impl fmt::Display for WifError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for WifError {}

impl From<WifError> for CliError {
    fn from(e: WifError) -> Self {
        CliError::Auth(e.message)
    }
}

pub fn config_invalid(cause: impl fmt::Display) -> WifError {
    WifError::new(CONFIG_INVALID, cause)
}

pub fn source_unavailable(cause: impl fmt::Display) -> WifError {
    WifError::new(SOURCE_TOKEN_UNAVAILABLE, cause)
}

pub fn exchange_unavailable(cause: impl fmt::Display) -> WifError {
    WifError::new(EXCHANGE_UNAVAILABLE, cause)
}

pub fn exchange_rejected(
    message: String,
    error: Option<String>,
    error_description: Option<String>,
) -> WifError {
    WifError {
        category: EXCHANGE_REJECTED,
        message,
        error,
        error_description,
    }
}

/// Describes an HTTP failure without its URL, which can carry query parameters.
pub fn describe_http_error(e: &reqwest::Error) -> &'static str {
    if e.is_timeout() {
        "timeout"
    } else if e.is_connect() {
        "connection failed"
    } else if e.is_body() || e.is_decode() {
        "response body error"
    } else {
        "request failed"
    }
}
