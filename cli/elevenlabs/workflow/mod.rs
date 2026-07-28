//! ElevenLabs "agents-as-code" workflow — hand-written custom commands
//! layered on top of the Fern-generated CLI.
//!
//! This module tree ports the bespoke workflow from the v0 (TypeScript)
//! CLI: local project files (`agents.json`/`tools.json`/`tests.json` +
//! `*_configs/` dirs) plus push/pull/verify/branch tooling that the raw
//! generated API commands do not provide.
//!
//! Everything here is protected from regeneration via `.fernignore`.
//! Each submodule exposes `register(app) -> CliApp`, and [`register`]
//! composes them. New top-level groups (`agents`, `tools`, `tests`) are
//! distinct from the generated `conversational-ai agents …` commands.

use fern_cli_sdk::app::CliApp;

mod agents;
mod api;
mod components;
mod project;
mod residency;
mod settings;
mod templates;
mod tests;
mod tools;
mod util;
mod verify;

/// Register every custom command group on the CLI app builder.
pub fn register(app: CliApp) -> CliApp {
    let app = agents::register(app);
    let app = templates::register(app);
    let app = tools::register(app);
    let app = tests::register(app);
    let app = residency::register(app);
    components::register(app)
}
