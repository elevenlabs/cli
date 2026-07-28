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

/// Which base URL to export, given whatever the env already holds and the
/// stored region. `None` means "leave the env alone" — either something more
/// explicit is already set, or the region is the API's default host. Kept
/// separate from the env plumbing so the precedence rules are unit-testable.
fn base_url_to_export(existing: Option<&str>, residency: &str) -> Option<&'static str> {
    // An empty value is treated as unset; exporting "" would break every request.
    if existing.is_some_and(|v| !v.trim().is_empty()) {
        return None;
    }
    if residency == settings::DEFAULT_RESIDENCY {
        return None;
    }
    Some(settings::base_url_for(residency))
}

/// Export the stored residency's base URL unless something more explicit
/// already set one. Precedence ends up: `--base-url` > `ELEVENLABS_BASE_URL`
/// (real env or `.env`) > stored residency > default.
fn apply_stored_residency() {
    // The framework loads `.env` inside `run()`, i.e. after this point, and
    // dotenvy never overrides an existing var — so load it here first to see
    // a `.env`-provided base URL and leave it alone.
    let _ = dotenvy::dotenv();
    let existing = std::env::var(BASE_URL_ENV).ok();
    let residency = settings::read_residency();
    if let Some(url) = base_url_to_export(existing.as_deref(), &residency) {
        std::env::set_var(BASE_URL_ENV, url);
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_explicit_base_url_is_never_overridden() {
        assert_eq!(
            base_url_to_export(Some("https://example.test"), "eu-residency"),
            None
        );
    }

    #[test]
    fn an_empty_base_url_counts_as_unset() {
        // Exporting "" would break every request, so treat it as absent.
        assert_eq!(
            base_url_to_export(Some("   "), "eu-residency"),
            Some("https://api.eu.residency.elevenlabs.io")
        );
    }

    #[test]
    fn the_default_region_exports_nothing() {
        assert_eq!(base_url_to_export(None, settings::DEFAULT_RESIDENCY), None);
    }

    #[test]
    fn an_isolated_region_exports_its_host() {
        assert_eq!(
            base_url_to_export(None, "sg-residency"),
            Some("https://api.sg.residency.elevenlabs.io")
        );
    }
}
