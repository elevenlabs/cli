//! `elevenlabs onboard`: set ElevenLabs up in a project from a coding agent.
//! Each step prints what the agent does next (see [`reply`]), so the flow is
//! driven by the CLI rather than by instructions the agent has to remember:
//!
//! - `init`: check the project, say what's next
//! - `connect`: sign in in the browser and store an API key for the project
//! - `check`: confirm the key and the integration (`status` is an alias)
//! - `test`: a short text-to-speech request, only when the developer says yes
//! - `setup`: install ElevenLabs in the developer's coding agents, when the
//!   flow offered it and they said yes

mod auth;
mod browser;
mod check;
mod clipboard;
mod connect;
mod env_file;
mod init;
mod manual;
mod project;
mod reply;
mod session;
mod test_request;
#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::ffi::OsString;
use std::sync::OnceLock;

use fern_cli_sdk::app::CliApp;
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::AppContext;

use super::api;
use super::util::downcast_ctx;

fn dispatch(matches: &clap::ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let (name, sub) = matches
        .subcommand()
        .ok_or_else(|| CliError::Validation("Start with `elevenlabs onboard init`.".into()))?;
    // The worker reports the sign-in before its own (see `connect::handle`).
    let last = (name != "connect").then(last_sign_in).flatten();
    let _scope = api::command_scope_with(
        match name {
            "init" => "onboard.init",
            "connect" => "onboard.connect",
            "check" => "onboard.check",
            "setup" => "onboard.setup",
            _ => "onboard.test",
        },
        sign_in_tag(last.as_ref()),
    );
    let reply = match name {
        "init" => Some(init::handle(sub, ctx)?.to_value()),
        "connect" => connect::handle(sub, ctx)?,
        "check" => Some(check::handle(sub, ctx)?.to_value()),
        "setup" => Some(setup(sub, ctx)?.to_value()),
        _ => Some(test_request::handle(sub, ctx)?.to_value()),
    };
    match reply {
        Some(reply) => reply::emit(&reply),
        None => Ok(()),
    }
}

/// `onboard setup`: what `setup --yes` does, as a `done` reply. With
/// telemetry on it first makes the account request `check` makes, so the
/// offers accepted can be counted; its answer changes nothing.
fn setup(matches: &clap::ArgMatches, ctx: &AppContext) -> Result<reply::Reply, CliError> {
    let started = std::time::Instant::now();
    let project = project::Project::open(matches)?;
    if super::telemetry::enabled() {
        if let Some(key) = project.key() {
            let _ = auth::check_key(ctx, &key.value);
        }
    }
    let home = super::setup::home()
        .ok_or_else(|| CliError::Other(anyhow::anyhow!("no home folder to set the agents up in")))?;
    // The request's time comes out of the whole run's, so the reply still comes in time.
    let limits = super::setup::Limits {
        total: super::setup::LIMITS.total.saturating_sub(started.elapsed()),
        ..super::setup::LIMITS
    };
    let outcomes = super::setup::run(&super::setup::found_here(&home), &home, limits);
    let mut say: Vec<String> = Vec::new();
    for outcome in &outcomes {
        let mut line = format!("{}.", super::setup::result_line(outcome));
        if let Some(finish) = super::setup::finish(outcome) {
            line.push_str(&format!(" To finish, {finish}."));
        }
        say.push(line);
    }
    if outcomes.iter().any(|o| !o.ok) {
        say.push(format!(
            "To try again later, run `{}`.",
            super::setup::later_command()
        ));
    }
    Ok(reply::Reply::new(reply::Next::Done, say.join(" ")).detail("setup", &outcomes))
}

/// `done`, the end of the flow, or, when this project's sign-in asked for it
/// and a coding agent can take the plugin (or none of the three is there and
/// the skills can be copied), the question that offers `setup`. Made once.
fn or_offer_setup(project: &project::Project, done: reply::Reply) -> reply::Reply {
    let Ok(session) = session::Session::for_project(&project.dir) else {
        return done;
    };
    if !session.setup_offer_due() {
        return done;
    }
    let Some(home) = super::setup::home() else {
        return done;
    };
    match super::setup::question(&super::setup::found_here(&home)) {
        Some(question) => {
            session.setup_offer_made();
            done.offer_setup(question, &project.next("setup"))
        }
        None => done,
    }
}

/// `signin/<outcome>` with telemetry on, so a request after a failed sign-in
/// says how it ended, even when the developer then adds a key by hand.
fn sign_in_tag(sign_in: Option<&session::LastSignIn>) -> Option<String> {
    sign_in
        .filter(|_| super::telemetry::enabled())
        .map(|sign_in| format!("signin/{}", sign_in.outcome))
}

/// How the sign-in for the project in the working folder last ended.
fn last_sign_in() -> Option<session::LastSignIn> {
    let dir = std::env::current_dir().ok()?;
    session::Session::for_project(&dir).ok()?.last_sign_in()
}

/// `--env-path`, not `--env-file`: through npx, Node takes `--env-file` for itself (for any
/// package), so the CLI never sees it and Node fails when the file doesn't exist yet. The old
/// name still works when the CLI is run directly.
fn env_file_arg() -> clap::Arg {
    clap::Arg::new("env-file")
        .long("env-path")
        .alias("env-file")
        .value_name("PATH")
        .help("The env file for ELEVENLABS_API_KEY (default: .env for Python projects, else .env.local)")
}

fn flag(id: &'static str, help: &'static str) -> clap::Arg {
    clap::Arg::new(id)
        .long(id)
        .action(clap::ArgAction::SetTrue)
        .help(help)
}

fn no_browser_arg() -> clap::Arg {
    flag(
        "no-browser",
        "Don't open a browser; the reply gives the approval link to open",
    )
}

fn onboard_command() -> clap::Command {
    clap::Command::new("onboard")
        // Run by coding agents, not typed by people: kept out of `--help`.
        .hide(true)
        .about("Set up ElevenLabs in this project from a coding agent. Start with `onboard init`.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            clap::Command::new("init")
                .about("Check the project and say what's next")
                .arg(env_file_arg())
                .arg(no_browser_arg()),
        )
        .subcommand(
            clap::Command::new("connect")
                .about("Sign in in the browser and store an API key for this project")
                .arg(env_file_arg())
                .arg(no_browser_arg())
                .arg(flag("replace", "Store a new key even if the project already has one"))
                .arg(flag("wait", "Keep waiting for an approval that is already open"))
                .arg(
                    clap::Arg::new("authorize-url")
                        .long("authorize-url")
                        .value_name("URL")
                        .hide(true)
                        .help("A local approval page to open instead of production (development only)"),
                )
                .arg(flag(session::WORKER_FLAG, "").hide(true))
                .arg(flag(session::LAUNCH_FLAG, "").hide(true)),
        )
        .subcommand(
            clap::Command::new("check")
                .alias("status")
                .about("Check the key and the integration, and say what's left")
                .arg(env_file_arg())
                .arg(no_browser_arg())
                // Passed by the question's `run` once the developer has said what to build.
                .arg(flag("answered", "The developer has said what to build").hide(true)),
        )
        .subcommand(
            clap::Command::new("test")
                .about("Make a short text-to-speech request with the project's key (ask the developer first)")
                .arg(env_file_arg())
                .arg(no_browser_arg()),
        )
        .subcommand(
            clap::Command::new("setup")
                // Run only when the flow offers it and the developer says yes.
                .hide(true)
                .about("Install ElevenLabs in the coding agents on this machine, as `elevenlabs setup --yes` does")
                .arg(env_file_arg())
                .arg(no_browser_arg()),
        )
}

/// The environment as the shell gave it, from before the runtime loads a
/// project's `.env` into it: what the shell itself set.
static SHELL_ENV: OnceLock<HashMap<OsString, OsString>> = OnceLock::new();

/// The shell's own environment, once `register` has run (always, but in unit tests).
pub(super) fn shell_env() -> Option<&'static HashMap<OsString, OsString>> {
    SHELL_ENV.get()
}

/// What this process added to its environment after startup: chiefly the
/// project's `.env`, which the runtime loads and which must never configure
/// a program the CLI starts (git, the browser).
fn added_since_startup() -> Vec<OsString> {
    let Some(shell) = shell_env() else {
        return Vec::new();
    };
    std::env::vars_os()
        .map(|(key, _)| key)
        .filter(|key| !shell.contains_key(key))
        .collect()
}

/// Runs before the runtime loads `.env`.
pub fn register(app: CliApp) -> CliApp {
    let _ = SHELL_ENV.set(std::env::vars_os().collect());
    app.command(
        onboard_command(),
        Box::new(|matches, ctx| dispatch(matches, downcast_ctx(ctx)?)),
    )
}
