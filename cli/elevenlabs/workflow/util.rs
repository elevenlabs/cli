//! Small helpers shared across the custom command handlers: downcasting
//! the type-erased binding context and reading framework-global / string
//! args from a clap match.

use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::AppContext;

/// Downcast the type-erased handler context to the OpenAPI `AppContext`.
pub fn downcast_ctx(ctx: &dyn std::any::Any) -> Result<&AppContext, CliError> {
    ctx.downcast_ref::<AppContext>()
        .ok_or_else(|| CliError::Validation("binding context type mismatch".to_string()))
}

/// Read the framework's global `--dry-run` flag (declared `global(true)`),
/// defaulting to false if absent.
pub fn dry_run_flag(matches: &clap::ArgMatches) -> bool {
    matches
        .try_get_one::<bool>("dry-run")
        .ok()
        .flatten()
        .copied()
        .unwrap_or(false)
}

/// Read an optional string argument.
pub fn opt_string(matches: &clap::ArgMatches, id: &str) -> Option<String> {
    matches.get_one::<String>(id).cloned()
}
