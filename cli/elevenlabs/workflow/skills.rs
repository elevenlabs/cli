//! `generate-skills`, shadowing the framework's built-in so the emitted
//! SKILL.md files cover the hand-written commands and the two
//! agent-feedback affordances.
//!
//! ## Why this shadows rather than extends
//!
//! The emitter (`fern_cli_sdk::openapi::skill_emitter`) walks the OpenAPI
//! spec and renders fixed templates. Commands in this `workflow/` tree
//! exist only here, so they are invisible to it — an agent that installed
//! the generated skills would have no idea `say`, `agents push` or
//! `residency` exist. It also has no hook for extra prose, and it is
//! generated code — editing it would be clobbered by the next
//! `fern generate`. Registering a custom command with the same name wins
//! dispatch over the built-in, so this file (protected by `.fernignore`)
//! can wrap it instead.
//!
//! The wrapper stays deliberately thin: it calls the framework's
//! [`generate_skills`] for the spec-derived content, so upstream
//! improvements to the templates still arrive. It owns only the output
//! path, the extra sections and the hand-written skills. If the emitter's
//! signature changes upstream, this fails to compile — visibly, rather
//! than silently emitting stale skills.
//!
//! ## Why the two feedback features need this at all
//!
//! Neither is reachable by the emitter. `--intent` is a
//! [`GlobalParameter`](super::intent), and the Global Flags table is a
//! hardcoded list that does not enumerate registered globals. `feedback
//! missing-capability` is a hand-written command, and the emitter only
//! walks spec-derived resources. Both gaps are worth fixing upstream in
//! the generator; until then, an agent reading only the skills would never
//! learn either exists.
//!
//! ## Adding a hand-written skill
//!
//! Drop a `<name>.md` in `skills/` next to this file and add it to
//! [`CUSTOM_SKILLS`]. It lives here, rather than under `.agents/skills/`
//! with the rest, because `.fernignore` protects `cli/elevenlabs/workflow/`
//! but not `.agents/` — a regeneration that swept `.agents/` away would take
//! the `include_str!` target with it and break the build. `.agents/skills/say/
//! SKILL.md` is a symlink back to this copy, so an agent working in this repo
//! still loads it (`.claude` symlinks to `.agents`) without a second file to
//! keep in sync.
//!
//! Those same bytes serve both audiences, so a skill must not contain paths
//! relative to either layout.

use std::path::{Path, PathBuf};

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

/// The binary name the emitter uses for headings and file prefixes, which
/// `main.rs` pins via `CliApp::new("elevenlabs")`. `AppContext` does not
/// expose it, and this file only ever ships in the elevenlabs CLI.
const BIN_NAME: &str = "elevenlabs";

/// Hand-written skills, as `(directory suffix, contents)`.
///
/// `include_str!` rather than a runtime read: the generated skills have to
/// work from an installed binary, which has no repo to read from.
const CUSTOM_SKILLS: &[(&str, &str)] = &[("say", include_str!("skills/say.md"))];

/// Appended to the shared skill, which every group skill links as a
/// prerequisite — so this is read once and applies everywhere.
///
/// Wording tracks the hosted MCP's `context` argument and `get_more_tools`
/// tool, which are what demonstrably get agents to supply these, with the
/// PII sentence the MCP versions lack.
const FEEDBACK_SECTION: &str = r#"
## Telling us what you are doing

Two inputs let you report what you are trying to accomplish and what you could
not do. Neither changes what a command does.

### `--intent` — why you are running this command

**Required on every command that calls the API.** You are reading this because
you are an agent, not a person at a terminal, and the CLI refuses a command from
a non-interactive caller until `--intent` is supplied:

```
error[validation]: --intent is required when the CLI is not attached to an interactive terminal.
```

It exits `3`, sends nothing, and the same command succeeds once you add the
flag. Put it on every invocation:

```bash
elevenlabs voices search --intent "pick a narrator voice for an audiobook"
```

One sentence describing the user's goal, max 500 characters, on one line.

If you genuinely have nothing to say, pass an empty value and the command runs
with no header — but prefer a real sentence, which is the whole point:

```bash
elevenlabs voices search --intent ""
```

No environment variable sets this once for a whole task. A fresh sentence per
command is the point — read back in order, they show what you were actually
working through.

`--help`, `--schema`, `--version`, `errors`, `completion` and `man` do not need
it; they send no request. Everything else does, `--dry-run` included.

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
built. `--intent` warns on stderr and the command proceeds normally — a dropped
value still satisfies the requirement, so you do not need to retry, but fix the
wording next time; `feedback` fails so you can rewrite it.
"#;

/// Appended to every per-group skill. The long-form prose lives in the shared
/// skill, which the other 31 files reach only through a PREREQUISITE link — and
/// the emitter tells agents to prefer `--schema`, which cannot see custom
/// commands at all. So the pointer has to be in the file actually being read.
/// Kept to four lines: agents read these whole, so every line costs tokens.
const GAP_POINTER: &str = r#"
## Nothing here fits?

`--schema` and `--help` list only what exists. If the user's request cannot be done
with any command, report the gap — it is how missing commands get built:

```bash
elevenlabs feedback missing-capability "<what you needed>"
```
"#;

/// Every file `generate-skills` should write, spec-derived ones first.
fn skill_files(ctx: &AppContext) -> Result<Vec<(PathBuf, String)>, CliError> {
    let shared = PathBuf::from(SHARED_SKILL).join("SKILL.md");
    let mut files = generate_skills(ctx.spec(), BIN_NAME, &[]);

    let mut appended = false;
    let mut pointers = 0usize;
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
        } else {
            content.push_str(GAP_POINTER);
            pointers += 1;
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
    if pointers == 0 {
        // Same reasoning as above: the shared skill alone is reachable only via
        // a link the agent may never follow.
        return Err(CliError::Other(anyhow::anyhow!(
            "expected at least one per-group skill to carry the capability-gap pointer"
        )));
    }

    files.extend(CUSTOM_SKILLS.iter().map(|(name, content)| {
        (
            PathBuf::from(format!("{BIN_NAME}-{name}")).join("SKILL.md"),
            (*content).to_string(),
        )
    }));
    Ok(files)
}

fn write_all(root: &Path, files: &[(PathBuf, String)]) -> Result<(), CliError> {
    for (rel_path, content) in files {
        let full_path = root.join(rel_path);
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
    Ok(())
}

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

    let files = skill_files(ctx)?;
    write_all(&resolved, &files)?;

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
        assert!(FEEDBACK_SECTION.contains("feedback missing-capability"));
        assert!(FEEDBACK_SECTION.contains("cannot be completed"));
        assert!(FEEDBACK_SECTION.contains("Do not call it when an existing command"));
    }

    #[test]
    fn the_section_states_that_intent_is_required() {
        assert!(FEEDBACK_SECTION.contains("**Required on every command that calls the API.**"));
        assert!(FEEDBACK_SECTION.contains("error[validation]: --intent is required"));
        assert!(FEEDBACK_SECTION.contains("--intent \"\""));
        assert!(FEEDBACK_SECTION.contains("No environment variable sets this once"));
        // The exempt commands have to be named, or an agent pays a turn to learn them.
        assert!(FEEDBACK_SECTION.contains("they send no request"));
    }

    #[test]
    fn the_gap_pointer_is_short_and_names_the_command() {
        // It lands in ~31 files that agents read in full, so length matters.
        assert!(GAP_POINTER.contains("feedback missing-capability"));
        assert!(GAP_POINTER.contains("--schema"), "must counter the emitter's advice");
        assert!(
            GAP_POINTER.lines().count() <= 12,
            "got {} lines",
            GAP_POINTER.lines().count()
        );
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

    /// Guards the substitution in [`skill_files`]: it only fires when the
    /// emitter actually renders the no-auth fallback for an empty binding
    /// list. If a future emitter renders something else, the hand-written
    /// section would silently never be applied.
    #[test]
    fn the_emitter_still_renders_the_no_auth_fallback() {
        let doc = fern_cli_sdk::openapi::discovery::RestDescription::default();
        let files = generate_skills(&doc, BIN_NAME, &[]);
        let (_, shared) = files
            .iter()
            .find(|(p, _)| p.starts_with(SHARED_SKILL))
            .expect("the shared skill is always emitted");
        assert!(
            shared.contains(NO_AUTH_LINE),
            "the emitter no longer renders the no-auth fallback, so the \
             hand-written authentication section would never be \
             substituted:\n{shared}"
        );
    }

    /// Every hand-written skill needs the frontmatter an agent harness reads
    /// to decide whether to load it.
    #[test]
    fn custom_skills_carry_usable_frontmatter() {
        for (name, content) in CUSTOM_SKILLS {
            assert!(
                content.starts_with("---\n"),
                "{name}: SKILL.md must open with YAML frontmatter"
            );
            let front = content
                .split("---\n")
                .nth(1)
                .unwrap_or_else(|| panic!("{name}: unterminated frontmatter"));
            assert!(
                front.contains(&format!("name: {BIN_NAME}-{name}")),
                "{name}: the frontmatter name must match the emitted directory \
                 {BIN_NAME}-{name}, or the two copies drift"
            );
            assert!(
                front.contains("description:"),
                "{name}: a skill without a description is never selected"
            );
        }
    }

    /// The same bytes are installed by `generate-skills` and read in-repo
    /// from `.agents/skills/`, so a path that only resolves in one layout is
    /// a broken link in the other.
    #[test]
    fn custom_skills_avoid_layout_relative_links() {
        for (name, content) in CUSTOM_SKILLS {
            assert!(
                !content.contains("](../") && !content.contains("`../"),
                "{name}: SKILL.md must not reference sibling skills by relative \
                 path — the in-repo and generated layouts differ"
            );
        }
    }
}
