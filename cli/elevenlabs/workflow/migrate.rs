//! `elevenlabs migrate openai`: move a project's OpenAI speech code to
//! ElevenLabs, from a coding agent. The know-how is the `migrate-from-openai`
//! skill; this command only hands it over, so no one installs skills first.
//! It doesn't sign in, touch a key or read the project: the skill ends by
//! offering `onboard init`, which does the first two.

use fern_cli_sdk::app::CliApp;
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::AppContext;

use super::onboard::reply::{self, Next, Reply};
use super::skills_copy;
use super::util::downcast_ctx;

const SKILL: &str = "migrate-from-openai";

/// The reply for `migrate openai`: the skill to follow, in a copy of the
/// skills outside the project, or what to tell the developer when it couldn't
/// be fetched. Converting without the guide would be guessing, so a failed
/// download stops here.
fn openai(ctx: &AppContext) -> Result<Reply, CliError> {
    let dir = skills_copy::new_dir()?;
    let guide = skills_copy::fetch(ctx, &dir).and_then(|_| skills_copy::guide(&dir, SKILL));
    Ok(match guide {
        Ok(guide) => Reply::new(
            Next::FollowGuide,
            "I'll move this project's OpenAI speech code to ElevenLabs, following the ElevenLabs migration guide.",
        )
        .run("init --from openai")
        .detail("guide", guide.display().to_string())
        .detail("skills_dir", dir.display().to_string())
        .with_rules(),
        Err(why) => {
            let _ = std::fs::remove_dir_all(&dir);
            Reply::new(
                Next::AskUser,
                format!(
                    "I couldn't get the ElevenLabs migration guide ({why}), so nothing in your project has \
                     changed. Tell me when you'd like me to try again."
                ),
            )
            .run_cli("migrate openai")
        }
    })
}

fn migrate_command() -> clap::Command {
    clap::Command::new("migrate")
        // Run by coding agents, not typed by people: kept out of `--help`.
        .hide(true)
        .about("Move a project to ElevenLabs from a coding agent. Start with `migrate openai`.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            clap::Command::new("openai")
                .about("Move this project's OpenAI text to speech and transcription code to ElevenLabs"),
        )
}

pub fn register(app: CliApp) -> CliApp {
    app.command(
        migrate_command(),
        Box::new(|_, ctx| reply::emit(&openai(downcast_ctx(ctx)?)?.to_value())),
    )
}
