//! The `tools` command group: add/delete/push/pull. Ports v0's
//! `src/tools/`.
//!
//! (Scaffold — command handlers land in a later step.)

use fern_cli_sdk::app::CliApp;

/// Register the `tools` command group.
pub fn register(app: CliApp) -> CliApp {
    app
}
