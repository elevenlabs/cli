//! The `residency` command: get/set the data-residency region, which
//! maps to the API base URL. Ports v0's `auth residency`.
//!
//! (Scaffold — command handler lands in a later step.)

use fern_cli_sdk::app::CliApp;

/// Register the `residency` command.
pub fn register(app: CliApp) -> CliApp {
    app
}
