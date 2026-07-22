//! Built-in agent templates and the `agents templates` command group.
//!
//! Ports v0's `src/agents/templates.ts`. For now this exposes the
//! template catalog (`list`); full template bodies and `show` land with
//! the rest of the `agents` group.

use fern_cli_sdk::app::CliApp;
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::AppContext;

/// `(name, description)` for every built-in agent template, in display
/// order. Mirrors v0's `getTemplateOptions()`.
pub const TEMPLATE_OPTIONS: &[(&str, &str)] = &[
    (
        "default",
        "Complete configuration with all available fields and sensible defaults",
    ),
    ("minimal", "Minimal configuration with only essential fields"),
    ("voice-only", "Optimized for voice-only conversations"),
    ("text-only", "Optimized for text-only conversations"),
    (
        "customer-service",
        "Pre-configured for customer service scenarios",
    ),
    ("assistant", "General purpose AI assistant configuration"),
];

#[derive(clap::Args)]
struct TemplatesListArgs {}

fn handle_list(_args: TemplatesListArgs, _ctx: &AppContext) -> Result<(), CliError> {
    println!("Available agent templates:");
    println!("{}", "=".repeat(50));
    for (name, description) in TEMPLATE_OPTIONS {
        println!("\n{name}");
        println!("   {description}");
    }
    println!(
        "\nUse 'elevenlabs agents add <name> --template <template_name>' \
         to create an agent with a specific template"
    );
    Ok(())
}

/// Register the `agents templates` command group.
pub fn register(app: CliApp) -> CliApp {
    app.command_under_typed_with(
        &["agents", "templates"],
        clap::Command::new("list").about("List available agent templates"),
        handle_list,
    )
}
