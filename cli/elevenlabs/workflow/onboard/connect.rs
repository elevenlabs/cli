//! `onboard connect`: sign in through the browser and store the API key the
//! approval page hands over on the clipboard. The sign-in runs in a
//! background [`Session`], so a slow approval never depends on how long the
//! agent lets a command run.

use std::time::Duration;

use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::AppContext;
use serde_json::Value;

use super::super::util::opt_string;
use super::auth;
use super::check::write_code;
use super::clipboard::{Clip, Clipboard, Marker};
use super::env_file::{self, API_KEY_ENV};
use super::manual::{self, NO_BROWSER};
use super::project::{last4, shell_arg, KeySource, Project};
use super::reply::{command, Next, Reply};
use super::session::{Session, LAUNCH_FLAG, SIGN_IN_MINUTES, WORKER_FLAG};

/// How long one `connect` waits for the approval before handing back to the
/// agent; well under the time agents let a command run.
pub(super) const WAIT: Duration = Duration::from_secs(25);

/// `None` for the launcher and the worker, which print nothing.
pub(super) fn handle(
    matches: &clap::ArgMatches,
    ctx: &AppContext,
) -> Result<Option<Value>, CliError> {
    let project = Project::open(matches)?;
    let session = Session::for_project(&project.dir)?;
    let replace = matches.get_flag("replace");
    if matches.get_flag(LAUNCH_FLAG) {
        session.launch_worker()?;
        return Ok(None);
    }
    if matches.get_flag(WORKER_FLAG) {
        // Another worker already signing in for this project wins.
        let Some(lock) = session.take()? else {
            return Ok(None);
        };
        // The browser the sign-in opens reads this environment, and by now the
        // runtime has loaded the project's `.env` into it. That also drops the
        // request tag `dispatch` set, so it is set again.
        for key in super::added_since_startup() {
            std::env::remove_var(key);
        }
        let _scope = super::super::api::command_scope("onboard.connect");
        let reply = sign_in_and_store(matches, ctx, &project, || session.signed_in(&lock))
            .unwrap_or_else(|e| failed(&project, &e.to_string(), replace));
        session.finish(&lock, &reply.to_value())?;
        return Ok(None);
    }
    let waiting = matches.get_flag("wait");
    if !waiting {
        if let Some(reply) = preflight(matches, &project, replace)? {
            return Ok(Some(reply.to_value()));
        }
        // Waits on the sign-in already running for this request, if there is one.
        session.start(&format!("{}\n{replace}", project.env_file.display()))?;
    }
    // Without a browser, the developer needs the link as soon as there is one.
    let link_first = !waiting && project.no_browser;
    Ok(Some(wait(&session, &project, WAIT, replace, link_first)))
}

fn connect_step(replace: bool) -> &'static str {
    if replace {
        "connect --replace"
    } else {
        "connect"
    }
}

/// Everything that can be refused before a browser opens.
fn preflight(
    matches: &clap::ArgMatches,
    project: &Project,
    replace: bool,
) -> Result<Option<Reply>, CliError> {
    if let Some(problem) = project.env_file_problem() {
        return Ok(Some(
            Reply::new(Next::Fix, problem).run(&project.next(connect_step(replace))),
        ));
    }
    authorize_url(matches)?;
    if let Some(key) = project.key() {
        match (&key.source, replace) {
            (_, false) => {
                return Ok(Some(
                    Reply::new(
                        Next::Run,
                        format!(
                            "This project already has an ElevenLabs key (ending in {}).",
                            last4(&key.value)
                        ),
                    )
                    .run(&project.next("check")),
                ))
            }
            (KeySource::Shell, true) => {
                return Ok(Some(
                    Reply::new(
                        Next::AskUser,
                        format!(
                            "{API_KEY_ENV} is set in your shell, and it wins over any env file. Unset it \
                             (and remove it from your shell profile), then tell me to try again."
                        ),
                    )
                    .run(&project.next("connect --replace")),
                ))
            }
            // The new key has to replace the one in the `.env` loaders read.
            (KeySource::Dotenv(path), true) => {
                let file = project.show(path);
                return Ok(Some(
                    Reply::new(Next::Run, format!("The key is in {file}, so the new one goes there."))
                        .run(&project.next(&format!("connect --replace --env-file {}", shell_arg(&file)))),
                ));
            }
            (KeySource::EnvFile, true) => {}
        }
    }
    if project.remote() {
        return manual::reply(project, &project.env_file, NO_BROWSER).map(Some);
    }
    Ok(None)
}

/// The worker's reply, or what to do while there is none yet. `link_first`
/// returns as soon as the approval link is known.
pub(super) fn wait(
    session: &Session,
    project: &Project,
    budget: Duration,
    replace: bool,
    link_first: bool,
) -> Value {
    let retry = project.next(connect_step(replace));
    if let Some(mut result) = session.wait(budget, |s| link_first && s.url().is_some()) {
        // Written by another project's `connect`, which can't know this one's flags.
        if result["details"]["key_status"] == "sign_in_replaced" {
            result["run"] = command(&retry).into();
        }
        return result;
    }
    if session.running() {
        return waiting(session, project, replace).to_value();
    }
    Reply::new(
        Next::Run,
        "The sign-in isn't running anymore, so I'll start it again.",
    )
    .run(&retry)
    .to_value()
}

/// The approval is still open: wait on it, with the link in case no tab opened.
pub(super) fn waiting(session: &Session, project: &Project, replace: bool) -> Reply {
    let link = session
        .url()
        .map(|url| format!(" If no browser tab opened, open this link: {url}"))
        .unwrap_or_default();
    Reply::new(
        Next::Wait,
        format!("Waiting for you to approve ElevenLabs in the browser.{link}"),
    )
    .run(&project.next(&format!("{} --wait", connect_step(replace))))
}

/// Only a local page may replace production: the developer types a password there.
fn authorize_url(matches: &clap::ArgMatches) -> Result<String, CliError> {
    match opt_string(matches, "authorize-url") {
        None => Ok(auth::sign_in_config()?.authorize_url.clone()),
        Some(url) if auth::loopback(&url).is_some() => Ok(url),
        Some(_) => Err(CliError::Validation(
            "--authorize-url must point at a local (loopback) page.".into(),
        )),
    }
}

/// What the approval page left on the clipboard came to.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum Outcome {
    /// A key for the signed-in account: store it.
    Store(String),
    /// The workspace role can't create keys: an admin has to.
    NotPermitted,
    /// The developer adds a key by hand. `status` names the case; `why` explains it.
    Manual {
        status: &'static str,
        why: &'static str,
    },
}

/// `verdict` is whether a key on the clipboard belongs to the signed-in account.
pub(super) fn outcome_of(
    clip: Clip,
    verdict: impl FnOnce(&str) -> Result<bool, CliError>,
) -> Outcome {
    match clip {
        Clip::Key(key) => match verdict(&key) {
            Ok(true) => Outcome::Store(key),
            Ok(false) => Outcome::Manual {
                status: "clipboard_invalid",
                why: "the key on the clipboard belongs to a different account",
            },
            Err(_) => Outcome::Manual {
                status: "could_not_verify",
                why: "the key couldn't be checked, so it wasn't stored",
            },
        },
        Clip::Marker(Marker::NotPermitted) => Outcome::NotPermitted,
        Clip::Marker(Marker::NotAvailable) => Outcome::Manual {
            status: Marker::NotAvailable.status(),
            why: "this account can't create a key this way",
        },
        Clip::Marker(Marker::Failed) => Outcome::Manual {
            status: Marker::Failed.status(),
            why: "the approval page didn't hand a key over",
        },
        Clip::Nothing => Outcome::Manual {
            status: "no_key",
            why: "no key reached this command",
        },
    }
}

/// The worker: sign in, then move the key from the clipboard into the env file.
fn sign_in_and_store(
    matches: &clap::ArgMatches,
    ctx: &AppContext,
    project: &Project,
    on_signed_in: impl FnOnce(),
) -> Result<Reply, CliError> {
    let client = ctx.http_config().build_client()?;
    let signed_in = auth::sign_in(ctx, &authorize_url(matches)?, project.no_browser);
    if let Err(error) = signed_in {
        // The page may have copied a key before the callback failed; nothing
        // can vouch for it now, so it is cleared, not stored.
        Clipboard::open().discard_key();
        return Ok(failed(
            project,
            &error.to_string(),
            matches.get_flag("replace"),
        ));
    }
    on_signed_in();
    let mut clipboard = Clipboard::open();
    let clip = clipboard.read();
    let copied = match &clip {
        Clip::Key(key) => key.clone(),
        Clip::Marker(m) => m.text(),
        Clip::Nothing => String::new(),
    };
    let outcome = outcome_of(clip, |key| {
        auth::key_matches_sign_in(&client, &auth::api_base(ctx), key)
    });
    let reply = reply_for(project, &outcome);
    // Whatever the outcome, and even if storing failed, a key never stays on the clipboard.
    clipboard.clear_if(&copied);
    reply
}

/// What the agent is told for each outcome, after storing the key if there is one.
pub(super) fn reply_for(project: &Project, outcome: &Outcome) -> Result<Reply, CliError> {
    let path = &project.env_file;
    // Read now: the file may have changed while the browser was open.
    let existing = env_file::read(path)?;
    let old_key = env_file::file_key(&existing).filter(|k| !k.is_empty());
    let file = &project.env_display;
    Ok(match outcome {
        Outcome::Store(key) => {
            env_file::write(path, &env_file::with_key(&existing, key))?;
            // The key is stored either way; `check` re-applies this and reports a failure.
            let gitignored = env_file::ensure_ignored(path).is_ok();
            let say = format!(
                "Signed in. Your new key ending in {} is in {file}, and your clipboard was cleared.",
                last4(key)
            );
            // A replaced key in an app that already uses the SDK: nothing to write.
            let replaced = old_key.is_some();
            let reply = if replaced && project.sdk_declared() && project.sdk_imported() {
                Reply::new(Next::Run, say).run(&project.next("check"))
            } else {
                write_code(project, &last4(key), Some(file), say)
            };
            reply
                .with_rules()
                .detail("key_status", "created")
                .detail("gitignored", gitignored)
        }
        Outcome::NotPermitted => {
            manual::prepare(path)?;
            Reply::new(
                Next::AskUser,
                format!(
                    "You're signed in, but your workspace role can't create API keys. Ask a workspace admin for a key, \
                     paste it after {API_KEY_ENV}= in {file} (never into this chat), and tell me when it's done."
                ),
            )
            .run(&project.next("check"))
            .detail("key_status", "not_permitted")
            .detail("env_file", file)
        }
        Outcome::Manual { status, why } => {
            let kept = old_key
                .map(|k| {
                    format!(
                        " The old key (ending in {}) is still in {file}; replace it.",
                        last4(&k)
                    )
                })
                .unwrap_or_default();
            manual::reply(
                project,
                path,
                &format!("You're signed in, but {why}.{kept}"),
            )?
            .detail("key_status", *status)
        }
    })
}

/// A sign-in that ended before the clipboard step. `error` is the sign-in
/// flow's message; a declined approval comes back as an `access_denied` error.
pub(super) fn failed(project: &Project, error: &str, replace: bool) -> Reply {
    let retry = project.next(connect_step(replace));
    let reply = if error.contains("returned error: access_denied") {
        Reply::new(
            Next::AskUser,
            format!(
                "The approval was declined. Tell me if you want to try again, or create a key at {} and paste it \
                 after {API_KEY_ENV}= in {} yourself (never into this chat).",
                manual::API_KEYS_PAGE,
                project.env_display
            ),
        )
    } else if error.contains("Timed out waiting") {
        Reply::new(
            Next::Run,
            format!(
                "The approval page wasn't completed within {SIGN_IN_MINUTES} minutes, so {} again.",
                project.approval()
            ),
        )
    } else {
        Reply::new(
            Next::Run,
            format!(
                "The sign-in didn't finish ({error}). If it fails again, create a key at {} and paste it after \
                 {API_KEY_ENV}= in {} instead.",
                manual::API_KEYS_PAGE,
                project.env_display
            ),
        )
    };
    reply.run(&retry).detail("key_status", "sign_in_failed")
}
