//! The environment WIF reads: the process's own, never the CLI's `.env` file.
//!
//! The CLI loads `.env` from the working directory and its parents into the process environment,
//! and a repository the operator did not write can ship one. WIF config, sources and endpoint
//! overrides decide which identity is used and where credentials go, so they come only from a
//! snapshot taken before that load.

use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::sync::OnceLock;

static PROCESS_ENV: OnceLock<HashMap<OsString, OsString>> = OnceLock::new();

/// Snapshots the process environment; must run before the CLI loads `.env`.
pub fn capture() {
    let _ = PROCESS_ENV.set(std::env::vars_os().collect());
}

/// A non-empty variable from the snapshot (the live environment where none was taken, as in tests).
pub fn var_os(name: &str) -> Option<OsString> {
    let value = match PROCESS_ENV.get() {
        Some(env) => env.get(OsStr::new(name)).cloned(),
        None => std::env::var_os(name),
    };
    value.filter(|v| !v.is_empty())
}

/// Like [`var_os`], for values that must be UTF-8.
pub fn var(name: &str) -> Option<String> {
    var_os(name).and_then(|v| v.into_string().ok())
}
