//! `onboard init`: the first step. It checks the project and says what's next:
//! signing in, the manual key path on a remote machine, or straight to
//! `check` when a working key is already there.

use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::AppContext;
use super::check::key_problem;
use super::connect;
use super::manual::{self, NO_BROWSER};
use super::project::{last4, Project};
use super::reply::{Next, Reply};
use super::session::Session;

const SIGN_IN: &str = "Next, I'll open the ElevenLabs approval page in your browser. Sign in (or create an account) and click Authorize.";
const SIGN_IN_LINK: &str = "Next, I'll give you the link to the ElevenLabs approval page. Open it, sign in (or create an account) and click Authorize.";

pub(super) fn handle(matches: &clap::ArgMatches, ctx: &AppContext) -> Result<Reply, CliError> {
    let project = Project::open(matches)?;
    if let Some(problem) = project.env_file_problem() {
        return Ok(Reply::new(Next::Fix, problem)
            .run(&project.next("init"))
            .with_rules());
    }
    Ok(next_step(ctx, &project)?
        .with_rules()
        .detail("env_file", &project.env_display))
}

fn next_step(ctx: &AppContext, project: &Project) -> Result<Reply, CliError> {
    if let Some(key) = project.key() {
        return Ok(key_problem(ctx, project, &key)?.unwrap_or_else(|| {
            Reply::new(
                Next::Run,
                format!(
                    "This project already has a working ElevenLabs key (ending in {}).",
                    last4(&key.value)
                ),
            )
            .run(&project.next("check"))
        }));
    }
    if project.remote() {
        return manual::reply(project, &project.env_file, NO_BROWSER);
    }
    // A sign-in is still open for this project: wait on it, don't start another.
    // (No home folder means no sign-in could have started.)
    if let Ok(session) = Session::for_project(&project.dir) {
        if session.running() {
            return Ok(connect::waiting(&session, project, false));
        }
    }
    let say = if project.no_browser {
        SIGN_IN_LINK
    } else {
        SIGN_IN
    };
    Ok(Reply::new(Next::Run, say).run(&project.next("connect")))
}
