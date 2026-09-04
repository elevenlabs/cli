//! `generate-skills`, shadowing the framework's built-in so the emitted
//! SKILL.md files mention the two agent-feedback affordances.
//!
//! ## Why this shadows rather than extends
//!
//! The emitter (`fern_cli_sdk::openapi::skill_emitter`) walks the OpenAPI
//! spec and renders fixed templates. It has no hook for extra prose, and
//! it is generated code — editing it would be clobbered by the next
//! `fern generate`. Registering a custom command with the same name wins
//! dispatch over the built-in, so this file (protected by `.fernignore`)
//! can wrap it instead.
//!
//! The wrapper stays deliberately thin: it calls the framework's
//! [`generate_skills`] for the actual content, so upstream improvements to
//! the templates still arrive. It owns only the output path and the extra
//! section. If the emitter's signature changes upstream, this fails to
//! compile — visibly, rather than silently emitting stale skills.
//!
//! ## Why the two features need this at all
//!
//! Neither is reachable by the emitter. `--intent` is a
//! [`GlobalParameter`](super::intent), and the Global Flags table is a
//! hardcoded list that does not enumerate registered globals. `feedback
//! missing-capability` is a hand-written command, and the emitter only
//! walks spec-derived resources. Both gaps are worth fixing upstream in
//! the generator; until then, an agent reading only the skills would never
//! learn either exists.

use std::path::PathBuf;

use fern_cli_sdk::app::CliApp;
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::skill_emitter::{generate_skills, generate_skills_command};
use fern_cli_sdk::openapi::AppContext;

use super::util::downcast_ctx;

/// What the emitter writes when it is handed no auth bindings. We cannot
/// hand it ours: they live on the `OpenApiBinding`, which `AppContext` does
/// not expose, and the spec carries no `securitySchemes` for the fallback to
/// use either. Left alone the shared skill would tell agents this CLI needs
/// no credentials, which is both wrong and the first thing they read.
const NO_AUTH_LINE: &str = "No authentication configured.";

/// Replaces [`NO_AUTH_LINE`]. Hand-written rather than rendered, so it can
/// say the thing an agent actually needs — the env var name — which the
/// generic rendering of our OAuth binding ("custom auth provider") does not.
const AUTH_SECTION: &str = "\
Every request is authenticated with an ElevenLabs API key, sent as the \
`xi-api-key` header.

```bash
export ELEVENLABS_API_KEY=xi-...
```

A `.env` file in the working directory is loaded automatically. For a one-off \
call, pass `--xi-api-key xi-...` instead. `elevenlabs auth login` sets up \
OAuth in a keyring as an alternative to the env var.";

/// Matches the emitter's own naming: `{bin_name}-shared/SKILL.md`.
const SHARED_SKILL: &str = "elevenlabs-shared";

/// The binary name the emitter uses for headings and file prefixes.
const BIN_NAME: &str = "elevenlabs";

/// Appended to the shared skill, which every group skill links as a
/// prerequisite — so this is read once and applies everywhere.
///
/// Wording tracks the hosted MCP's `context` argument and `get_more_tools`
/// tool, which are what demonstrably get agents to supply these, with the
/// PII sentence the MCP versions lack.
const FEEDBACK_SECTION: &str = r#"
## Telling us what you are doing

Two optional inputs let you report what you are trying to accomplish and what
you could not do. Both are opt-in and neither changes what a command does.

### `--intent` — why you are running this command

Available on every command. Sent as a request header.

```bash
elevenlabs voices search --intent "pick a narrator voice for an audiobook"
```

Set `ELEVENLABS_AGENT_INTENT` instead to apply one intent to every command in a
task, which usually fits better than repeating the flag:

```bash
export ELEVENLABS_AGENT_INTENT="migrate the support bot to eleven_turbo_v2"
```

Keep it to one sentence describing the user's goal, max 500 characters.

### `elevenlabs feedback missing-capability` — what you could not do

Call this when the user's request cannot be completed with any available
`elevenlabs` command. Describe the capability you were looking for, so it can
inform which commands get built next. Do not call it when an existing command
already covers the request.

```bash
elevenlabs feedback missing-capability \
  "no way to batch-render a script to separate files per speaker"
```

It records the report and returns; it does not fail the task. Continue with the
available commands, or tell the user the thing is not supported yet.

### Never put personal data in either field

Describe the *goal*, not the data. Resource ids (`agent_01jz…`) and
project-relative paths are fine; names, customer content, and anything you
would not want in an analytics store are not.

Two rules are enforced rather than trusted: a value over 500 characters, or one
carrying credentials or an absolute file path, is dropped before the request is
built. `--intent` warns on stderr and the command proceeds normally; `feedback`
fails so you can rewrite it.
"#;

fn handle(matches: &clap::ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let out_dir = matches
        .get_one::<String>("output-dir")
        .map(String::as_str)
        .unwrap_or("skills");
    // The framework's own validator, so this path behaves exactly as the
    // built-in did. Note it deliberately does not sandbox: it rejects control
    // characters and resolves the path, but the target may be anywhere on the
    // filesystem (see its docs). Shadowing neither adds nor removes that.
    let resolved = fern_cli_sdk::validate::validate_safe_output_dir(out_dir)?;

    let shared = PathBuf::from(SHARED_SKILL).join("SKILL.md");
    let mut files = generate_skills(ctx.spec(), BIN_NAME, &[]);

    let mut appended = false;
    for (path, content) in files.iter_mut() {
        if *path == shared {
            // Only when the emitter actually produced the no-auth text. If a
            // future framework version renders real bindings, defer to it
            // rather than overwriting a better section with ours.
            if content.contains(NO_AUTH_LINE) {
                *content = content.replace(NO_AUTH_LINE, AUTH_SECTION);
            }
            content.push_str(FEEDBACK_SECTION);
            appended = true;
        }
    }
    if !appended {
        // The emitter renamed or dropped the shared skill. Fail loudly: the
        // alternative is silently shipping skills without the section, which
        // is the exact failure this command exists to prevent.
        return Err(CliError::Other(anyhow::anyhow!(
            "expected the emitter to produce {}; the feedback section had nowhere to go",
            shared.display()
        )));
    }

    for (rel_path, content) in &files {
        let full_path = resolved.join(rel_path);
        if let Some(parent) = full_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                CliError::Validation(format!(
                    "Failed to create directory {}: {e}",
                    parent.display()
                ))
            })?;
        }
        std::fs::write(&full_path, content).map_err(|e| {
            CliError::Validation(format!("Failed to write {}: {e}", full_path.display()))
        })?;
    }

    eprintln!(
        "Wrote {} skill file(s) to {}/",
        files.len(),
        resolved.display()
    );
    Ok(())
}

/// Register the shadowing `generate-skills`.
///
/// Reuses the framework's own clap definition so `--help` and `--output-dir`
/// stay identical to the command being replaced.
pub fn register(app: CliApp) -> CliApp {
    app.command(
        generate_skills_command(),
        Box::new(|matches, ctx| handle(matches, downcast_ctx(ctx)?)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_section_documents_both_affordances() {
        // An agent reads this and nothing else before deciding whether to
        // use them, so the trigger phrasing is the whole mechanism.
        assert!(FEEDBACK_SECTION.contains("--intent"));
        assert!(FEEDBACK_SECTION.contains("ELEVENLABS_AGENT_INTENT"));
        assert!(FEEDBACK_SECTION.contains("feedback missing-capability"));
        assert!(FEEDBACK_SECTION.contains("cannot be completed"));
        assert!(FEEDBACK_SECTION.contains("Do not call it when an existing command"));
    }

    #[test]
    fn the_section_states_the_pii_rule() {
        assert!(FEEDBACK_SECTION.contains("Never put personal data"));
        assert!(FEEDBACK_SECTION.contains("500 characters"));
    }

    #[test]
    fn the_auth_replacement_names_the_env_var() {
        // The whole point of overriding the rendered section: an agent needs
        // the variable name, not the words "custom auth provider".
        assert!(AUTH_SECTION.contains("ELEVENLABS_API_KEY"));
        assert!(AUTH_SECTION.contains("xi-api-key"));
        assert!(!AUTH_SECTION.contains(NO_AUTH_LINE));
    }

    #[test]
    fn the_shared_skill_target_matches_the_emitters_naming() {
        // `generate_skills` builds this path as `{bin_name}-shared/SKILL.md`.
        // If the two drift, `handle` errors rather than emitting silently.
        assert_eq!(SHARED_SKILL, format!("{BIN_NAME}-shared"));
    }
}
