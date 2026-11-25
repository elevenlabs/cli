//! `onboard init`: the first step. It checks the project and says what's next:
//! signing in, the manual key path on a remote machine, or, when the project
//! already has a key, whatever `check` says about it.

use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::AppContext;
use super::check;
use super::connect;
use super::manual::{self, NO_BROWSER};
use super::project::Project;
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
    let reply = next_step(ctx, &project)?.with_rules();
    // A reply from `check` already names the file the key is in.
    Ok(if reply.details.contains_key("env_file") {
        reply
    } else {
        reply.detail("env_file", &project.env_display)
    })
}

fn next_step(ctx: &AppContext, project: &Project) -> Result<Reply, CliError> {
    // A key already there: check it now, so what comes next (what to build, or
    // the test) is this reply, not a `check` the agent could skip.
    if project.key().is_some() {
        return check::verify(ctx, project, false);
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
