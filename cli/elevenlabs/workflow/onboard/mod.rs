//! `elevenlabs onboard`: set ElevenLabs up in a project from a coding agent.
//! Each step prints what the agent does next (see [`reply`]), so the flow is
//! driven by the CLI rather than by instructions the agent has to remember:
//!
//! - `init`: check the project, say what's next
//! - `connect`: sign in in the browser and store an API key for the project
//! - `check`: confirm the key and the integration (`status` is an alias)
//! - `test`: a short text-to-speech request, only when the developer says yes

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
    let _scope = api::command_scope(match name {
        "init" => "onboard.init",
        "connect" => "onboard.connect",
        "check" => "onboard.check",
        _ => "onboard.test",
    });
    let reply = match name {
        "init" => Some(init::handle(sub, ctx)?.to_value()),
        "connect" => connect::handle(sub, ctx)?,
        "check" => Some(check::handle(sub, ctx)?.to_value()),
        _ => Some(test_request::handle(sub, ctx)?.to_value()),
    };
    match reply {
        Some(reply) => reply::emit(&reply, human(sub)),
        None => Ok(()),
    }
}

/// A person asked for prose: `--human`, or `--format table`.
fn human(matches: &clap::ArgMatches) -> bool {
    let flag = matches
        .try_get_one::<bool>("human")
        .ok()
        .flatten()
        .copied()
        .unwrap_or(false);
    let table = matches
        .try_get_one::<String>("format")
        .ok()
        .flatten()
        .is_some_and(|f| f == "table");
    flag || table
}

fn env_file_arg() -> clap::Arg {
    clap::Arg::new("env-file")
        .long("env-file")
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
                .arg(no_browser_arg()),
        )
        .subcommand(
            clap::Command::new("test")
                .about("Make a short text-to-speech request with the project's key (ask the developer first)")
                .arg(env_file_arg())
                .arg(no_browser_arg()),
        )
}

/// The environment as the shell gave it, from before the runtime loads a
/// project's `.env` into it: what the shell itself set.
static SHELL_ENV: OnceLock<HashMap<OsString, OsString>> = OnceLock::new();

/// The shell's own environment, once `register` has run (always, but in unit tests).
fn shell_env() -> Option<&'static HashMap<OsString, OsString>> {
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
