//! The `feedback` command group: a channel for agents to report what the
//! CLI cannot do.
//!
//! [`super::intent`] answers "why is this command running", but the more
//! valuable signal is the one that produces no request at all — an agent
//! looked for a command, found none, and gave up. Nothing in the request
//! log can show that. The hosted MCP solves it with a virtual
//! `get_more_tools` tool; this is the CLI's version of that tool.
//!
//! Unlike the intent header, a rejected value here is an error rather than
//! a warning: the text *is* the payload, so dropping it silently would
//! report success for a command that did nothing.

use fern_cli_sdk::app::CliApp;
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::AppContext;
use reqwest::Method;
use serde_json::{json, Value};

use super::util::{downcast_ctx, dry_run_flag};
use super::{api, intent};

/// First-party endpoint for CLI feedback. Deliberately absent from the
/// public OpenAPI spec — it is reached through [`api::raw_request`] and is
/// not an API we want the SDKs to generate clients for. Relative, matching
/// the generated SDK's convention.
const FEEDBACK_PATH: &str = "v1/cli/feedback";

/// Mirrors the MCP's `get_more_tools` description, which is the wording
/// that demonstrably gets agents to call it, plus the PII sentence the MCP
/// version lacks.
const LONG_ABOUT: &str = "\
Call this when the user's request cannot be completed with any available \
elevenlabs command. Describe the capability you were looking for, so it can \
inform which commands get built next. Do not call it when an existing command \
already covers the request.

Never include names, email addresses, phone numbers, API keys, file paths, or \
any other personal or customer data — describe the capability, not the data. \
A description that looks like it contains personal data is rejected.";

/// Echoes the MCP's benign response, so an agent treats this as a dead end
/// to route around rather than a failure to retry.
const RECORDED_MESSAGE: &str = "Recorded. No command exists for this yet — continue with the \
available commands, or tell the user this is not supported.";

/// Assemble the report body. Pure, so the shape is testable without a
/// live `AppContext`.
fn report_body(capability: &str, intent: Option<String>) -> Value {
    let mut body = json!({
        "kind": "missing_capability",
        "capability": capability,
        "cli_version": env!("CARGO_PKG_VERSION"),
        "command": "feedback.missing_capability",
    });
    // Plain text, not the percent-encoded header form — this is a JSON body.
    if let Some(text) = intent {
        body["intent"] = json!(text);
    }
    body
}

fn handle_missing_capability(
    matches: &clap::ArgMatches,
    ctx: &AppContext,
) -> Result<(), CliError> {
    let _scope = api::command_scope("feedback.missing_capability");

    let raw = matches
        .get_one::<String>("capability")
        .expect("capability is a required positional");

    let capability = intent::sanitize(raw).map_err(|reason| {
        CliError::Validation(format!(
            "Capability description rejected because {reason}. Rewrite it and try again."
        ))
    })?;

    let body = report_body(&capability, intent::resolved_text());

    // Honour the framework's global --dry-run. Without this an agent probing
    // the command with --dry-run would file a real report.
    if dry_run_flag(matches) {
        println!("[DRY RUN] Would report missing capability: {capability}");
        if let Some(text) = body.get("intent").and_then(Value::as_str) {
            println!("  [DRY RUN] With intent: {text}");
        }
        return Ok(());
    }

    api::raw_request(ctx, Method::POST, FEEDBACK_PATH, Some(body), None)?;
    println!("{RECORDED_MESSAGE}");
    Ok(())
}

/// Register the `feedback` command group.
///
/// Registered untyped so the handler can read the framework's global
/// `--dry-run`, which the typed form does not surface — the same reason
/// `tools push` and `agents pull` use this form.
pub fn register(app: CliApp) -> CliApp {
    app.command_under(
        &["feedback"],
        clap::Command::new("missing-capability")
            .about("Report that a task could not be completed with any available command")
            .long_about(LONG_ABOUT)
            .arg(
                clap::Arg::new("capability")
                    .required(true)
                    .help(
                        "What you were trying to accomplish that the available commands \
                         could not do. One or two sentences, max 500 characters, no \
                         personal data.",
                    ),
            ),
        Box::new(|matches, ctx| handle_missing_capability(matches, downcast_ctx(ctx)?)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_long_about_names_the_pii_rule() {
        // The wording is the whole mechanism — an agent reads this and
        // nothing else before deciding what to send.
        assert!(LONG_ABOUT.contains("cannot be completed"));
        assert!(LONG_ABOUT.contains("Never include names"));
        assert!(LONG_ABOUT.contains("Do not call it when an existing command"));
    }

    #[test]
    fn a_capability_with_personal_data_is_refused_before_any_request() {
        // Rejection has to happen client-side — the request would otherwise
        // carry the data to the server, which is the thing being prevented.
        assert!(intent::sanitize("import +1 555 010 9999 as an outbound number").is_err());
        assert!(intent::sanitize("no way to batch-render per speaker").is_ok());
    }

    #[test]
    fn the_body_carries_the_capability_and_omits_an_absent_intent() {
        let body = report_body("cannot batch-render per speaker", None);
        assert_eq!(body["kind"], "missing_capability");
        assert_eq!(body["capability"], "cannot batch-render per speaker");
        assert_eq!(body["command"], "feedback.missing_capability");
        assert_eq!(body["cli_version"], env!("CARGO_PKG_VERSION"));
        assert!(body.get("intent").is_none(), "absent intent must be omitted");
    }

    #[test]
    fn the_body_includes_the_intent_as_plain_text() {
        // Percent-encoding is for the header only; encoding it here would
        // land escaped text in the analytics store.
        let body = report_body("no accent picker", Some("gerar diálogo".to_string()));
        assert_eq!(body["intent"], "gerar diálogo");
    }

    #[test]
    fn the_endpoint_path_is_relative() {
        // `raw_request` builds on the SDK's convention; a leading slash
        // produces a doubled path.
        assert!(!FEEDBACK_PATH.starts_with('/'));
    }
}
