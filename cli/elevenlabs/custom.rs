//! Custom command handlers for the ElevenLabs "agents-as-code" workflow.
//!
//! This file and the `workflow/` module tree are hand-written and are
//! protected from regeneration by `.fernignore`
//! (`cli/elevenlabs/custom.rs` and `cli/elevenlabs/workflow/`). The
//! generated `main.rs` calls `custom::register(app)` at startup, composing
//! these commands into the CLI at compile time.
//!
//! Handlers get a fully-wired SDK client via `super::sdk::client(ctx)`
//! (inherits auth, retries, TLS, base URL, and global headers) and run
//! async SDK calls with `super::sdk::block_on(future)`. Types come from
//! `elevenlabs_sdk::api::*`.

use fern_cli_sdk::app::CliApp;

// The workflow module tree lives at `cli/elevenlabs/workflow/`. Because
// this file is `custom.rs` (not `mod.rs`), point `mod` at the directory
// explicitly so the tree sits alongside the generated files rather than
// under a `custom/` subdirectory.
#[path = "workflow/mod.rs"]
mod workflow;

/// Register all custom commands on the CLI app builder.
pub fn register(app: CliApp) -> CliApp {
    workflow::register(app)
}
