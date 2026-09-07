//! `generate-skills`, extended to cover the hand-written commands.
//!
//! The framework's own `generate-skills` renders one `SKILL.md` per OpenAPI
//! resource group, driven entirely by the embedded spec — so the commands in
//! this `workflow/` tree, which exist only here, are invisible to it. An
//! agent that installed the generated skills would have no idea `say`,
//! `agents push` or `residency` exist.
//!
//! This shadows the built-in rather than duplicating it: `graft_subcommand`
//! is custom-wins on leaf collision and custom commands are dispatched ahead
//! of binding operations, so registering `generate-skills` here takes over
//! the name. Every spec-derived file still comes from the framework's own
//! emitter via [`skill_emitter::generate_skills`], so improvements to it
//! keep flowing through; this only appends the hand-written skills and
//! writes the result.
//!
//! Adding a skill: drop a `<name>.md` in `skills/` next to this file and add
//! it to [`CUSTOM_SKILLS`]. It lives here, rather than under `.agents/skills/`
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
use fern_cli_sdk::auth::{no_auth_provider, SchemeBinding};
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::{skill_emitter, AppContext};

/// The binary name, which `main.rs` pins via `CliApp::new("elevenlabs")`.
/// The emitter takes it as a parameter to prefix every skill directory
/// (`elevenlabs-shared`, `elevenlabs-agents`, …); `AppContext` does not
/// expose it, and this file only ever ships in the elevenlabs CLI.
const BIN_NAME: &str = "elevenlabs";

/// Hand-written skills, as `(directory suffix, contents)`.
///
/// `include_str!` rather than a runtime read: the generated skills have to
/// work from an installed binary, which has no repo to read from.
const CUSTOM_SKILLS: &[(&str, &str)] = &[("say", include_str!("skills/say.md"))];

/// The auth bindings to render the shared skill's "Authentication" section
/// from.
///
/// `AppContext` carries the resolved auth *provider* but not the bindings
/// that describe it, and they live on a `pub(crate)` field of the framework's
/// `CliApp`, so there is nothing to forward. Passing an empty slice is not an
/// option: the emitter would fall back to the spec's `securitySchemes`, which
/// this API declares as `{}`, and the section would read "No authentication
/// configured."
///
/// So it is reconstructed to match what `main.rs` declares — a PKCE login
/// flow named `OAuth`. The emitter renders a `Custom` binding as the constant
/// string "custom auth provider" and collects no environment variables from
/// it, so the provider handed over here is never consulted and the output is
/// identical to the built-in's. `the_shared_skill_still_documents_oauth`
/// fails loudly if a future emitter starts reading it.
fn auth_bindings() -> Vec<(String, SchemeBinding)> {
    vec![(
        "OAuth".to_string(),
        SchemeBinding::Custom(no_auth_provider()),
    )]
}

/// Every file `generate-skills` should write, spec-derived ones first.
fn skill_files(ctx: &AppContext) -> Vec<(PathBuf, String)> {
    let mut files = skill_emitter::generate_skills(ctx.spec(), BIN_NAME, &auth_bindings());
    files.extend(CUSTOM_SKILLS.iter().map(|(name, content)| {
        (
            PathBuf::from(format!("{BIN_NAME}-{name}")).join("SKILL.md"),
            (*content).to_string(),
        )
    }));
    files
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

#[derive(clap::Args)]
struct SkillsArgs {
    /// Output directory [default: skills]
    #[arg(long, value_name = "PATH")]
    output_dir: Option<String>,
}

fn handle(args: SkillsArgs, ctx: &AppContext) -> Result<(), CliError> {
    let out_dir = args.output_dir.as_deref().unwrap_or("skills");
    // Same guard the framework applies: refuses control characters and paths
    // that would escape the working tree.
    let resolved = fern_cli_sdk::validate::validate_safe_output_dir(out_dir)?;

    let files = skill_files(ctx);
    write_all(&resolved, &files)?;

    eprintln!(
        "Wrote {} skill file(s) to {}/",
        files.len(),
        resolved.display()
    );
    Ok(())
}

/// Register `generate-skills`, replacing the framework's own.
///
/// The `about` text matches the built-in so `--help` reads the same whether
/// or not this override is present.
pub fn register(app: CliApp) -> CliApp {
    app.command_typed_with(
        clap::Command::new("generate-skills")
            .about("Generate SKILL.md files for AI agent integration"),
        handle,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

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

    /// Guards the reconstruction in [`auth_bindings`]: if the emitter ever
    /// starts reading the provider inside a `Custom` binding, the stand-in
    /// stops being equivalent and this notices.
    #[test]
    fn the_shared_skill_still_documents_oauth() {
        let doc = fern_cli_sdk::openapi::discovery::RestDescription::default();
        let files = skill_emitter::generate_skills(&doc, BIN_NAME, &auth_bindings());
        let (_, shared) = files
            .iter()
            .find(|(p, _)| p.starts_with(format!("{BIN_NAME}-shared")))
            .expect("the shared skill is always emitted");
        assert!(
            shared.contains("- **OAuth** (bearer): custom auth provider"),
            "the shared skill lost its authentication line; auth_bindings() \
             no longer reproduces what the framework renders:\n{shared}"
        );
        assert!(
            !shared.contains("No authentication configured"),
            "an empty binding list leaked through"
        );
    }
}
