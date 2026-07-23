//! The `components add` command: install ElevenLabs UI components by
//! shelling out to `npx shadcn`. Ports v0's `src/components/`.
//!
//! (Scaffold — command handler lands in a later step.)

use fern_cli_sdk::app::CliApp;

/// Register the `components` command group.
pub fn register(app: CliApp) -> CliApp {
    app
}
