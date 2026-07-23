//! The `tests` command group: add/delete/push/pull, with push-time
//! auto-discovery of untracked test config files. Ports v0's
//! `src/tests/`.
//!
//! (Scaffold — command handlers land in a later step.)

use fern_cli_sdk::app::CliApp;

/// Register the `tests` command group.
pub fn register(app: CliApp) -> CliApp {
    app
}
