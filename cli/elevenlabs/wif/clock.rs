//! The WIF clock: monotonic, or the conformance test clock file.

use std::sync::{Once, OnceLock};
use std::time::Instant;

use super::error::{config_invalid, WifError};

pub const TEST_CLOCK_ENV: &str = "ELEVENLABS_TEST_CLOCK_PATH";

static START: OnceLock<Instant> = OnceLock::new();
static WARNED: Once = Once::new();

/// Seconds on the WIF clock; only differences between readings matter.
pub fn now() -> Result<f64, WifError> {
    let Some(path) = std::env::var_os(TEST_CLOCK_ENV).filter(|p| !p.is_empty()) else {
        return Ok(START.get_or_init(Instant::now).elapsed().as_secs_f64());
    };
    WARNED.call_once(|| super::warn("ElevenLabs WIF test clock active"));
    let shown = std::path::Path::new(&path).display().to_string();
    let text = std::fs::read_to_string(&path)
        .map_err(|e| config_invalid(format!("test clock {shown} is unreadable ({e})")))?;
    text.trim()
        .parse::<f64>()
        .ok()
        .filter(|v| v.is_finite())
        .ok_or_else(|| config_invalid(format!("test clock {shown} is not a number")))
}
