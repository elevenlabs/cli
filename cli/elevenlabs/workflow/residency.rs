//! The `residency` command: get/set the data-residency region, which
//! selects the API base URL. Ports v0's `auth residency`.
//!
//! v0 built its own HTTP client, so it could route calls straight from the
//! stored setting. Here the framework resolves the base URL from
//! `--base-url` / `ELEVENLABS_BASE_URL` and offers no config-file hook, so
//! [`apply_stored_residency`] bridges the gap: it runs during `register`
//! (before `CliApp::run`, which is where the framework reads the env) and
//! exports the stored region's base URL when nothing more explicit is set.
//! That way a configured residency routes *every* command — generated API
//! commands included — while `--base-url` and an explicit
//! `ELEVENLABS_BASE_URL` still win.

use fern_cli_sdk::app::CliApp;
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::AppContext;

use super::settings;

/// Env var the framework reads for the base URL. Derived from the binary
/// name (`elevenlabs` → `ELEVENLABS_`), which the generator pins.
const BASE_URL_ENV: &str = "ELEVENLABS_BASE_URL";

/// Export the stored residency's base URL unless something more explicit
/// already set one. Precedence ends up: `--base-url` > `ELEVENLABS_BASE_URL`
/// (real env or `.env`) > stored residency > default.
fn apply_stored_residency() {
    // The framework loads `.env` inside `run()`, i.e. after this point, and
    // dotenvy never overrides an existing var — so load it here first to see
    // a `.env`-provided base URL and leave it alone.
    let _ = dotenvy::dotenv();
    if std::env::var_os(BASE_URL_ENV).is_some() {
        return;
    }
    let residency = settings::read_residency();
    if residency == settings::DEFAULT_RESIDENCY {
        // `global` is the API's default host; nothing to override.
        return;
    }
    std::env::set_var(BASE_URL_ENV, settings::base_url_for(&residency));
}

#[derive(clap::Args)]
struct ResidencyArgs {
    /// Region to switch to. Omit to show the current setting.
    residency: Option<String>,
}

fn handle_residency(args: ResidencyArgs, _ctx: &AppContext) -> Result<(), CliError> {
    let Some(requested) = args.residency else {
        let current = settings::read_residency();
        println!("Residency: {current}");
        println!("Base URL:  {}", settings::base_url_for(&current));
        println!("\nAvailable: {}", settings::RESIDENCY_VALUES.join(", "));
        println!("Set one with 'elevenlabs residency <region>'.");
        return Ok(());
    };

    if !settings::RESIDENCY_VALUES.contains(&requested.as_str()) {
        return Err(CliError::Validation(format!(
            "Invalid residency '{requested}'. Available: {}",
            settings::RESIDENCY_VALUES.join(", ")
        )));
    }

    settings::write_residency(&requested)?;
    println!("Residency set to: {requested}");
    println!("Base URL:  {}", settings::base_url_for(&requested));
    println!("\nThis applies to every command from now on. Override it per-run with");
    println!("--base-url or by setting {BASE_URL_ENV}.");
    Ok(())
}

/// Register the `residency` command.
///
/// Top-level rather than under `auth` (where v0 had it): the framework owns
/// the `auth` group, grafting it *after* custom commands and dispatching it
/// first, so nesting here would create a duplicate group whose subcommand
/// never reaches this handler.
pub fn register(app: CliApp) -> CliApp {
    apply_stored_residency();
    app.command_typed_with(
        clap::Command::new("residency")
            .about("Show or set the data-residency region (selects the API base URL)"),
        handle_residency,
    )
}
