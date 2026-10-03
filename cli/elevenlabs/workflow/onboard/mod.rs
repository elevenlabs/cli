//! `elevenlabs onboard`: sign in like `auth login`, then move the API key the
//! approval page copied to the clipboard into the env file. `onboard status`
//! reports what a project has. Both print JSON when piped, for the skill.

mod auth;
mod browser;
mod clipboard;
mod env_file;
#[cfg(test)]
mod tests;

use std::path::{Path, PathBuf};

use fern_cli_sdk::app::CliApp;
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::formatter::OutputPipeline;
use fern_cli_sdk::openapi::AppContext;
use serde_json::{json, Value};

use super::api;
use super::util::{downcast_ctx, opt_string};
use clipboard::{Clip, Clipboard};
use env_file::{API_KEY_ENV, DEFAULT_ENV_FILE};

const API_KEYS_PAGE: &str = "https://elevenlabs.io/app/settings/api-keys";
// Named like the skill directories in `elevenlabs/skills`.
const PRODUCTS: &[&str] = &[
    "text-to-speech",
    "speech-to-text",
    "voice-changer",
    "sound-effects",
    "music",
    "voice-isolator",
    "dubbing",
    "agents",
    "speech-engine",
];

fn handle_onboard(matches: &clap::ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let _scope = api::command_scope("onboard");
    let refuse = |msg: String| Err(CliError::Validation(msg));
    let env_display = opt_string(matches, "env-file").unwrap_or_else(|| DEFAULT_ENV_FILE.into());
    let env_file = PathBuf::from(&env_display);
    let no_browser = matches.get_flag("no-browser");
    // A bad --format or a missing directory must fail before a key exists.
    let output = output_pipeline(matches)?;
    if let Some(dir) = env_file.parent().filter(|d| !d.as_os_str().is_empty()) {
        if !dir.is_dir() {
            return refuse(format!(
                "{} is not a directory; create it or pass --env-file with a path in one.",
                dir.display()
            ));
        }
    }

    // Only a local page may replace production: the user types a password there.
    let authorize_url = match opt_string(matches, "authorize-url") {
        None => auth::sign_in_config()?.authorize_url.clone(),
        Some(url) if auth::loopback(&url).is_some() => url,
        Some(_) => return refuse("--authorize-url must point at a local (loopback) page.".into()),
    };
    // Refuse everything we can before a browser opens.
    if env_file::is_symlink(&env_file) {
        return refuse(format!(
            "{env_display} is a symlink; pass --env-file with a regular file."
        ));
    }
    if env_file::is_tracked(&env_file) {
        return refuse(format!(
            "{env_display} is tracked by git, so a secret must not go there. Pass --env-file with \
             a file git does not track, such as .env.local."
        ));
    }
    let existing = env_file::read(&env_file)?;
    let key_line = env_file::file_key(&existing);
    let existing_key = key_line.clone().filter(|k| !k.is_empty());
    if let Some(key) = existing_key
        .as_ref()
        .filter(|_| !matches.get_flag("replace"))
    {
        return refuse(format!(
            "{env_display} already sets {API_KEY_ENV} (ending in {}). Run `elevenlabs onboard \
             status` to check it, or pass --replace to store a new key over it.",
            last4(key)
        ));
    }
    // A `.env` in the directory chain is what the SDKs' loaders read first, so a
    // key there would keep winning over anything written to another file.
    if !matches.get_flag("replace") {
        if let Some((dotenv, _)) = env_file::dotenv_with_key(&current_dir()?) {
            if existing_key.is_none() && !same_file(&dotenv, &env_file) {
                return refuse(format!(
                    "{} already sets {API_KEY_ENV}, and the SDKs read that file first. Run \
                     `elevenlabs onboard status --env-file {0}` to check it, or pass `--env-file {0} \
                     --replace` to store a new key over it.",
                    dotenv.display()
                ));
            }
        }
    }
    if !no_browser && !browser::reachable() {
        return refuse(format!(
            "No browser can reach this machine (an SSH session, a container or a cloud agent), so \
             the approval page cannot hand a key to this command. Create a key at {API_KEYS_PAGE} \
             and expose it as {API_KEY_ENV} the way you manage secrets here, then run `elevenlabs \
             onboard status`. If a browser does reach this machine (port forwarding, say), pass \
             --no-browser to open the link yourself."
        ));
    }

    eprintln!(
        "Sign in to ElevenLabs in the browser (or create an account) and click Authorize.\n\
         The page creates an API key for this project; it is stored in {env_display} and never shown.\n"
    );
    if let Err(error) = auth::sign_in(
        ctx,
        &authorize_url,
        opt_string(matches, "product"),
        no_browser,
    ) {
        // The page may have created and copied a key before the callback
        // failed; nothing can vouch for it now, so it is cleared, not stored.
        if Clipboard::open().discard_key() {
            eprintln!("The sign-in did not finish; a key left on the clipboard was discarded.");
        }
        return Err(error);
    }
    // Re-read: the file may have changed while the browser was open.
    let existing = env_file::read(&env_file)?;
    let key_line = env_file::file_key(&existing);
    let existing_key = key_line.clone().filter(|k| !k.is_empty());

    // Clear the clipboard only once the key is in the file or it held a marker.
    let mut clipboard = Clipboard::open();
    let client = ctx.http_config().build_client()?;
    let api_base = auth::api_base(ctx);
    // Whatever the outcome, a key never stays on the clipboard.
    let (key_status, key_reason, key_last4) = match clipboard.read() {
        Clip::Key(key) => {
            let verdict = auth::key_matches_sign_in(&client, &api_base, &key);
            if verdict.as_ref().is_ok_and(|same| *same) {
                env_file::write(&env_file, &env_file::with_key(&existing, &key))?;
            }
            clipboard.clear_if(&key);
            match verdict {
                Ok(true) => ("created", None, Some(last4(&key))),
                Ok(false) => ("clipboard_invalid", None, None),
                Err(_) => ("failed", Some("could_not_verify"), None),
            }
        }
        Clip::Marker(marker) => {
            clipboard.clear_if(&clipboard::marker_text(marker));
            let reason = (marker == "failed").then_some("page_could_not_create");
            (marker, reason, None)
        }
        Clip::Nothing => ("no_key", None, None),
    };
    let written = key_last4.is_some();
    let placeholder = !written && key_line.is_none();
    if placeholder {
        env_file::write(&env_file, &env_file::with_key(&existing, ""))?;
    }
    let gitignore_updated = (written || placeholder) && env_file::ensure_ignored(&env_file)?;
    let next = clipboard::next_for(key_status);
    let human = match next {
        "none" => format!(
            "Signed in. An API key ending in {} is in {env_display}; your clipboard was cleared.",
            key_last4.as_deref().unwrap_or_default()
        ),
        "legacy" => format!(
            "Signed in, but no key could be created this way ({key_status}). Create one at \
             {API_KEYS_PAGE} or ask a workspace administrator, and paste it after {API_KEY_ENV}= in \
             {env_display}; your clipboard was cleared."
        ),
        _ => format!(
            "Signed in, but no usable key arrived ({key_status}). Create one at {API_KEYS_PAGE} and \
             paste it after {API_KEY_ENV}= in {env_display}; your clipboard was cleared."
        ),
    };
    emit(
        &output,
        &json!({
            "signed_in": true,
            "key_status": key_status,
            "key_reason": key_reason,
            "next": next,
            "env_file": env_display,
            "env_file_written": written || placeholder,
            "existing_key_kept": !written && existing_key.is_some(),
            "gitignore_updated": gitignore_updated,
            "key_last4": key_last4,
        }),
        &human,
    )
}

fn handle_status(matches: &clap::ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let _scope = api::command_scope("onboard.status");
    let output = output_pipeline(matches)?;
    let env_display = opt_string(matches, "env-file").unwrap_or_else(|| DEFAULT_ENV_FILE.into());
    let exists = std::path::Path::new(&env_display).exists();
    let contents = std::fs::read_to_string(&env_display).ok();
    let from_file = contents
        .as_deref()
        .and_then(env_file::file_key)
        .filter(|k| !k.is_empty());
    let from_env = std::env::var(API_KEY_ENV)
        .ok()
        .filter(|k| !k.trim().is_empty());
    // The runtime loads a `.env` from the directory chain into the process
    // environment before this runs, so a key "from the environment" may really
    // live in that file. Tell the two apart: only a real shell variable is `env`.
    let dotenv = env_file::dotenv_with_key(&current_dir()?);
    let (key, source, dotenv_file) = match (from_env, from_file) {
        (Some(env), Some(file)) if env == file => (Some(file), "env_file", None),
        (Some(env), _) => match dotenv {
            Some((path, key)) if key == env && !same_file(&path, Path::new(&env_display)) => {
                (Some(env), "dotenv", Some(path.display().to_string()))
            }
            _ => (Some(env), "env", None),
        },
        (None, Some(file)) => (Some(file), "env_file", None),
        (None, None) => (None, "none", None),
    };
    // `None` when the API could not be reached, so offline is not reported as rejected.
    let valid = key
        .as_deref()
        .and_then(|k| {
            let client = ctx.http_config().build_client().ok()?;
            auth::account_for(&client, &auth::api_base(ctx), "xi-api-key", k).ok()
        })
        .map(|account| account.is_some());
    let browser = browser::reachable();
    let signed_in = auth::stored_access_token().is_some();
    let verdict = match valid {
        Some(true) => "works",
        Some(false) => "rejected",
        None => "not checked",
    };
    let key_line = key.as_deref().map_or("none".to_string(), |k| {
        let from = match (source, &dotenv_file) {
            ("dotenv", Some(path)) => format!("from {path}"),
            ("env", _) => "from the shell environment".to_string(),
            _ => "from the env file".to_string(),
        };
        format!("ending in {}, {from}, {verdict}", last4(k))
    });
    let human = format!(
        "Signed in: {}\nBrowser:   {}\nEnv file:  {env_display}{}\nAPI key:   {key_line}",
        if signed_in { "yes" } else { "no" },
        if browser {
            "reaches this machine"
        } else {
            "cannot reach this machine (remote)"
        },
        if exists { "" } else { " (missing)" },
    );
    emit(
        &output,
        &json!({
            "signed_in": signed_in,
            "browser_reachable": browser,
            "env_file": env_display,
            "env_file_exists": exists,
            "api_key_present": key.is_some(),
            "api_key_source": source,
            "dotenv_file": dotenv_file,
            "api_key_valid": valid,
            "api_key_last4": key.as_deref().map(last4),
        }),
        &human,
    )
}

fn current_dir() -> Result<PathBuf, CliError> {
    std::env::current_dir()
        .map_err(|e| CliError::Other(anyhow::anyhow!("read the working directory: {e}")))
}

/// Whether two paths name the same file, by canonical path when both exist.
fn same_file(a: &Path, b: &Path) -> bool {
    match (a.canonicalize(), b.canonicalize()) {
        (Ok(a), Ok(b)) => a == b,
        _ => a == b,
    }
}

fn last4(key: &str) -> String {
    key.chars()
        .skip(key.chars().count().saturating_sub(4))
        .collect()
}

fn output_pipeline(matches: &clap::ArgMatches) -> Result<OutputPipeline, CliError> {
    OutputPipeline::from_matches(matches, auth::CLI_NAME)
        .map_err(|e| CliError::Validation(e.to_string()))
}

/// The selected machine format when piped or asked for, prose otherwise.
fn emit(pipeline: &OutputPipeline, machine: &Value, human: &str) -> Result<(), CliError> {
    if pipeline.format.is_machine_readable() {
        pipeline
            .emit(&mut std::io::stdout(), machine, false, true)
            .map_err(|e| CliError::Other(anyhow::anyhow!("{e}")))
    } else {
        println!("{human}");
        Ok(())
    }
}

fn env_file_arg() -> clap::Arg {
    clap::Arg::new("env-file")
        .long("env-file")
        .value_name("PATH")
        .help("The env file holding ELEVENLABS_API_KEY (default: .env.local in the current directory)")
}

fn onboard_command() -> clap::Command {
    clap::Command::new("onboard")
        // Run by the onboarding skill, not typed by people: kept out of `--help`.
        .hide(true)
        .about("Sign in and put a working ElevenLabs API key in this project")
        .long_about(
            "Sign in to ElevenLabs in the browser and store an API key for this project, in one step.\n\n\
             The approval page creates a key scoped to the product you name (or with the default \
             permissions when you name none) and hands it to this command, which writes it to the env \
             file, adds that file to .gitignore, and clears your clipboard. The key is never printed.\n\n\
             Output is JSON when piped or with --format json. `key_status` says what happened and \
             `next` says what to do: `none` when the key is in place, `paste_key` when it has to be \
             pasted into the env file by hand, `legacy` when this account cannot create keys this way.",
        )
        .args_conflicts_with_subcommands(true)
        .arg(
            clap::Arg::new("product")
                .long("product")
                .value_name("SLUG")
                .value_parser(PRODUCTS.to_vec())
                .help("Scope the key to one product, named like its skill (e.g. text-to-speech)"),
        )
        .arg(env_file_arg())
        .arg(
            clap::Arg::new("no-browser")
                .long("no-browser")
                .action(clap::ArgAction::SetTrue)
                .help("Print the sign-in URL instead of opening a browser (also skips the remote-machine check)"),
        )
        .arg(
            clap::Arg::new("replace")
                .long("replace")
                .action(clap::ArgAction::SetTrue)
                .help("Store a new key even if the env file already has one"),
        )
        .arg(
            clap::Arg::new("authorize-url")
                .long("authorize-url")
                .value_name("URL")
                .hide(true)
                .help("A local approval page to open instead of production (development only)"),
        )
        .subcommand(
            clap::Command::new("status")
                .about("Report whether this project is signed in, has a working API key, and can reach a browser")
                .arg(env_file_arg()),
        )
}

pub fn register(app: CliApp) -> CliApp {
    app.command(
        onboard_command(),
        Box::new(|matches, ctx| {
            let ctx = downcast_ctx(ctx)?;
            match matches.subcommand() {
                Some(("status", sub)) => handle_status(sub, ctx),
                _ => handle_onboard(matches, ctx),
            }
        }),
    )
}
