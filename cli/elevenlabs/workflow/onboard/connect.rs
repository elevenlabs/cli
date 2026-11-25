//! The key step, for `onboard init` (and `onboard connect`, its older name):
//! sign in through the browser exactly as `auth login` does, then store the
//! API key the approval page handed over on the clipboard. All this adds to
//! the CLI's own sign-in is what happens once it completes.

use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::AppContext;

use super::super::api;
use super::super::telemetry;
use super::super::util::opt_string;
use super::auth;
use super::check;
use super::clipboard::{Clip, Clipboard, Marker};
use super::env_file::{self, API_KEY_ENV};
use super::manual::{self, NO_BROWSER};
use super::project::{last4, KeySource, Project};
use super::reply::{Next, Reply};

/// How long `auth login`'s flow waits for the approval page.
const SIGN_IN_MINUTES: u64 = 5;

/// `step` is the command as it was run (`init` or `connect`), which is what
/// its requests are counted as until the sign-in starts.
pub(super) fn handle(
    step: &'static str,
    matches: &clap::ArgMatches,
    ctx: &AppContext,
) -> Result<Reply, CliError> {
    let project = Project::open(matches)?;
    let replace = matches.get_flag("replace");
    {
        let _scope = api::command_scope(step);
        if let Some(reply) = preflight(ctx, matches, &project, replace)? {
            return Ok(reply.with_rules());
        }
    }
    // The browser the sign-in opens reads this environment, and by now the
    // runtime has loaded the project's `.env` into it.
    for key in super::added_since_startup() {
        std::env::remove_var(key);
    }
    // The sign-in and the key's check count as `connect`, whichever step ran them.
    let _scope = api::command_scope("onboard.connect");
    Ok(sign_in_and_store(matches, ctx, &project)
        .unwrap_or_else(|e| failed(&project, &e.to_string(), replace, matches.get_flag("again")))
        .with_rules())
}

fn connect_step(replace: bool) -> &'static str {
    if replace {
        "connect --replace"
    } else {
        "connect"
    }
}

/// Everything that is settled before a browser opens: a file that can't hold
/// a key, a key that is already there, a machine no browser reaches.
fn preflight(
    ctx: &AppContext,
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
    match project.key() {
        // A key already there: check it now, so this reply is what comes next.
        Some(_) if !replace => return check::verify(ctx, project).map(Some),
        Some(key) if key.source == KeySource::Shell => return Ok(Some(
            Reply::new(
                Next::AskUser,
                format!(
                    "{API_KEY_ENV} is set in your shell, and it wins over any env file. Unset it \
                         (and remove it from your shell profile), then tell me to try again."
                ),
            )
            .run(&project.next("connect --replace")),
        )),
        _ => {}
    }
    if project.remote() {
        return manual::reply(project, &project.env_file, NO_BROWSER).map(Some);
    }
    Ok(None)
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

impl Outcome {
    /// The fixed word a sign-in that ended this way is reported as.
    pub(super) fn label(&self) -> &'static str {
        match self {
            Outcome::Store(_) => "created",
            Outcome::NotPermitted => "not-permitted",
            Outcome::Manual { status, .. } => match *status {
                "clipboard_invalid" => "wrong-account",
                "could_not_verify" => "unverified",
                "not_available" => "not-available",
                "failed" => "page-failed",
                "clipboard_unreadable" => "clipboard-error",
                _ => "no-key",
            },
        }
    }
}

/// With telemetry on, one request that says how the sign-in ended. A sign-in
/// that stored no key makes no other request, so this is the only trace of why.
fn report(ctx: &AppContext, token: &str, outcome: &Outcome) {
    if !telemetry::enabled() {
        return;
    }
    let _scope = api::tag_scope(&format!("signin/{}", outcome.label()));
    if let Ok(client) = ctx.http_config().build_client() {
        let bearer = format!("Bearer {token}");
        let _ = auth::account_for(&client, &auth::api_base(ctx), "authorization", &bearer);
    }
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
            Err(e) => {
                eprintln!("The key on the clipboard couldn't be checked: {e}");
                Outcome::Manual {
                    status: "could_not_verify",
                    why: "the key couldn't be checked, so it wasn't stored",
                }
            }
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
        Clip::Unreadable => Outcome::Manual {
            status: "clipboard_unreadable",
            why: "this command couldn't read your clipboard. If it still holds the key \
                  the approval page copied (it starts with sk_), skip step 1",
        },
    }
}

/// Sign in as `auth login` does, then move the key from the clipboard into the env file.
fn sign_in_and_store(
    matches: &clap::ArgMatches,
    ctx: &AppContext,
    project: &Project,
) -> Result<Reply, CliError> {
    let client = ctx.http_config().build_client()?;
    // The sign-in itself prints the link. This is for the agent, which reads it
    // while the command waits: ending its turn stops the command, and then
    // nothing is left to receive the approval.
    eprintln!(
        "Waiting up to {SIGN_IN_MINUTES} minutes for the approval in the browser. Keep this command \
         running until it finishes, and don't end your turn while it waits: this command is what \
         receives the approval, so stopping it cancels the sign-in."
    );
    let again = matches.get_flag("again");
    let token = match auth::sign_in(ctx, &authorize_url(matches)?, project.no_browser, again) {
        Ok(token) => token,
        Err(error) => {
            // The page may have copied a key before the sign-in failed; nothing
            // can vouch for it now, so it is cleared, not stored.
            Clipboard::open().discard_key();
            return Ok(failed(
                project,
                &error.to_string(),
                matches.get_flag("replace"),
                again,
            ));
        }
    };
    let mut clipboard = Clipboard::open();
    let clip = clipboard.read_handoff();
    let copied = match &clip {
        Clip::Key(key) => key.clone(),
        Clip::Marker(m) => m.text(),
        Clip::Nothing | Clip::Unreadable => String::new(),
    };
    let outcome = outcome_of(clip, |key| {
        auth::key_matches_sign_in(&client, &auth::api_base(ctx), &token, key)
    });
    let reply = reply_for(project, &outcome);
    // Whatever the outcome, even if storing failed, a key read here never stays on the clipboard.
    clipboard.clear_if(&copied);
    report(ctx, &token, &outcome);
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
            let say = format!(
                "Signed in. Your new key ending in {} is in {file}, and your clipboard was cleared.",
                last4(key)
            );
            match env_file::ensure_ignored(path) {
                // The key is in place, and everything after it is one reply.
                Ok(_) => check::integrate(project, &last4(key), Some(file), &say)?
                    .detail("key_status", "created")
                    .detail("gitignored", true),
                // Git could still commit the key's file, so that is fixed before any code is written.
                Err(e) => Reply::new(Next::Fix, format!("{say} {e}"))
                    .run(&project.next("check"))
                    .detail("key_status", "created")
                    .detail("gitignored", false),
            }
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
/// `again` is set when this run was itself the retry of a timed-out approval.
pub(super) fn failed(project: &Project, error: &str, replace: bool, again: bool) -> Reply {
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
    } else if error.contains("Timed out waiting") && !again {
        // One retry without asking, marked so a second timeout goes back to the developer.
        return Reply::new(
            Next::Run,
            format!(
                "The approval page wasn't completed within {SIGN_IN_MINUTES} minutes, so {} again.",
                project.approval()
            ),
        )
        .run(&project.next(&format!("{} --again", connect_step(replace))))
        .detail("key_status", "sign_in_failed");
    } else if error.contains("Timed out waiting") {
        // Twice in a row: the developer isn't there, so an agent never retries without end.
        Reply::new(
            Next::AskUser,
            format!(
                "The approval page wasn't completed in another {SIGN_IN_MINUTES} minutes. Tell me when \
                 you're ready to approve, and {} again.",
                project.approval()
            ),
        )
    } else {
        // An error in the callback is whatever was sent to the local port, so its text is never repeated.
        let error = if error.contains("returned error:") {
            "the approval page reported an error"
        } else {
            error
        };
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
