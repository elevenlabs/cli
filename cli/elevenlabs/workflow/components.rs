//! The `components add` command: install ElevenLabs UI components by
//! delegating to `shadcn`. Ports v0's `src/components/`.

use std::process::Command;

use fern_cli_sdk::app::CliApp;
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::AppContext;

const REGISTRY_BASE: &str = "https://ui.elevenlabs.io/r";

/// Registry component name. Restricted to the characters a registry slug can
/// contain so it can't inject extra arguments or reshape the URL.
fn validate_component(name: &str) -> Result<(), CliError> {
    let valid = !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
        && !name.starts_with('.')
        && !name.contains("..");
    if valid {
        Ok(())
    } else {
        Err(CliError::Validation(format!(
            "Invalid component name '{name}'. Use letters, digits, '-', '_' or '.'."
        )))
    }
}

/// Build the argv for the shadcn invocation. Arguments are passed
/// individually (never through a shell), so nothing in `url` is re-parsed.
/// Windows needs `cmd /C` because `npx` is a `.cmd` shim that
/// `std::process::Command` won't resolve on its own.
fn shadcn_argv(url: &str) -> (&'static str, Vec<String>) {
    let shadcn = ["-y", "shadcn@latest", "add", url].map(str::to_string);
    if cfg!(windows) {
        let mut args = vec!["/C".to_string(), "npx".to_string()];
        args.extend(shadcn);
        ("cmd", args)
    } else {
        ("npx", shadcn.to_vec())
    }
}

#[derive(clap::Args)]
struct AddArgs {
    /// Component to install. Defaults to `all`.
    #[arg(default_value = "all")]
    name: String,
}

fn handle_add(args: AddArgs, _ctx: &AppContext) -> Result<(), CliError> {
    validate_component(&args.name)?;
    let url = format!("{REGISTRY_BASE}/{}.json", args.name);

    println!("Installing {} from the ElevenLabs UI registry...", args.name);
    println!("Source: {url}\n");

    let (program, argv) = shadcn_argv(&url);
    // stdio is inherited so shadcn's interactive prompts work.
    let status = Command::new(program).args(&argv).status().map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            CliError::Validation(
                "npx is not available. Please install Node.js/npm to add components.".to_string(),
            )
        } else {
            CliError::Other(anyhow::anyhow!("Could not run npx: {e}"))
        }
    })?;

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
    Ok(())
}

/// Register the `components` command group.
pub fn register(app: CliApp) -> CliApp {
    app.command_under_typed_with(
        &["components"],
        clap::Command::new("add").about("Add a component from the ElevenLabs UI registry"),
        handle_add,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_registry_slugs() {
        for name in ["all", "conversation-bar", "orb_v2", "widget.embed"] {
            assert!(validate_component(name).is_ok(), "{name} should be valid");
        }
    }

    #[test]
    fn rejects_injection_and_traversal_attempts() {
        for name in [
            "",
            "a b",
            "a;rm -rf /",
            "a&&b",
            "a|b",
            "$(whoami)",
            "../../etc/passwd",
            "..",
            ".hidden",
            "a/b",
        ] {
            assert!(
                validate_component(name).is_err(),
                "{name:?} should be rejected"
            );
        }
    }

    #[test]
    fn argv_never_goes_through_a_shell_on_unix() {
        let (program, argv) = shadcn_argv("https://ui.elevenlabs.io/r/all.json");
        if cfg!(windows) {
            assert_eq!(program, "cmd");
            assert_eq!(argv[0], "/C");
        } else {
            assert_eq!(program, "npx");
            assert_eq!(
                argv,
                vec![
                    "-y",
                    "shadcn@latest",
                    "add",
                    "https://ui.elevenlabs.io/r/all.json"
                ]
            );
        }
    }
}
