//! `elevenlabs onboard`: set ElevenLabs up in a project from a coding agent.
//! One command puts a working API key in the project, and its reply hands the
//! rest to the agent (see [`reply`]):
//!
//! - `init`: sign in in the browser as `auth login` does, store the key the
//!   approval page hands over, and say how to build with it (`connect` is its
//!   older name)
//! - `check`: verify a key the developer added by hand (`status` is an alias)
//! - `test`: a short text-to-speech request, only when the developer says yes

mod auth;
mod browser;
mod check;
mod clipboard;
mod connect;
mod env_file;
mod manual;
mod project;
mod reply;
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
    let reply = match name {
        // These set their own request tag: the sign-in counts as `connect`.
        "init" => connect::handle("onboard.init", sub, ctx)?,
        "connect" => connect::handle("onboard.connect", sub, ctx)?,
        "check" => {
            let _scope = api::command_scope("onboard.check");
            check::handle(sub, ctx)?
        }
        _ => {
            let _scope = api::command_scope("onboard.test");
            test_request::handle(sub, ctx)?
        }
    };
    reply::emit(&reply.to_value())
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
        "Don't open a browser; the approval link is printed to open by hand",
    )
}

/// `init` and `connect` are the same step, so they take the same flags.
fn key_step(name: &'static str, about: &'static str) -> clap::Command {
    clap::Command::new(name)
        .about(about)
        .arg(env_file_arg())
        .arg(no_browser_arg())
        .arg(flag(
            "replace",
            "Store a new key even if the project already has one",
        ))
        // What older replies asked for: there is no sign-in left to wait on.
        .arg(flag("wait", "").hide(true))
        .arg(
            clap::Arg::new("authorize-url")
                .long("authorize-url")
                .value_name("URL")
                .hide(true)
                .help("A local approval page to open instead of production (development only)"),
        )
}

fn onboard_command() -> clap::Command {
    clap::Command::new("onboard")
        // Run by coding agents, not typed by people: kept out of `--help`.
        .hide(true)
        .about("Set up ElevenLabs in this project from a coding agent. Start with `onboard init`.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(key_step(
            "init",
            "Sign in in the browser, store an API key for this project, and say how to build with it",
        ))
        .subcommand(key_step("connect", "The same as `init`").hide(true))
        .subcommand(
            clap::Command::new("check")
                .alias("status")
                .about("Check a key that was added by hand, and say how to build with it")
                .arg(env_file_arg())
                .arg(no_browser_arg())
                // What older replies asked for: `check` no longer has stages.
                .arg(flag("answered", "").hide(true)),
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
