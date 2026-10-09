//! `onboard check` (also `onboard status`): the one way to verify the setup.
//! It stops at the first problem and says how to fix it, fixing on its own
//! only what is safe: the `.gitignore` entry and the file's permissions.

use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::AppContext;

use super::super::say::DEFAULT_VOICE_ID;
use super::auth::{check_key, KeyCheck};
use super::connect;
use super::env_file::{self, API_KEY_ENV};
use super::manual::{self, NO_BROWSER};
use super::project::{last4, shell_arg, Key, Project};
use super::reply::{Next, Reply};
use super::session::Session;

pub(super) fn handle(matches: &clap::ArgMatches, ctx: &AppContext) -> Result<Reply, CliError> {
    let project = Project::open(matches)?;
    let Some(key) = project.key() else {
        return no_key(&project);
    };
    if let Some(reply) = key_problem(ctx, &project, &key)? {
        return Ok(reply);
    }
    let mut fixed = Vec::new();
    if let Some(file) = key.file(&project) {
        let name = project.show(file);
        if env_file::is_tracked(file) {
            return Ok(Reply::new(
                Next::Fix,
                format!(
                    "git tracks {name}, so the key could be committed. Untrack it with `git rm --cached {}` \
                     (the file stays on disk). If it was ever committed and pushed, replace the key too.",
                    shell_arg(&name)
                ),
            )
            .run(&project.next("check")));
        }
        match env_file::ensure_ignored(file) {
            Ok(true) => fixed.push(format!("added {name} to .gitignore")),
            Ok(false) => {}
            Err(e) => return Ok(Reply::new(Next::Fix, e.to_string()).run(&project.next("check"))),
        }
        if env_file::make_private(file)? {
            fixed.push(format!("made {name} readable only by you"));
        }
    }
    let declared = project.sdk_declared();
    let imported = project.sdk_imported();
    let ending = last4(&key.value);
    let key_file = key.file(&project).map(|f| project.show(f));
    let reply = if !declared || !imported {
        write_code(
            &project,
            &ending,
            key_file.as_deref(),
            "Your key works. Next I'll add the ElevenLabs SDK to the project and use it in the code.",
        )
    } else {
        let location = key_file.as_deref().unwrap_or("your shell");
        Reply::new(
            Next::OfferTest,
            format!(
                "ElevenLabs is set up: a working key ending in {ending} in {location}, and the SDK in your code. \
                 Want me to send a test request? It makes a short text-to-speech clip with your key and uses a few credits."
            ),
        )
        .run(&project.next("test"))
        .detail("key_last4", &ending)
    };
    Ok(reply
        .detail("api_key_source", key.source_name())
        .detail("sdk_declared", declared)
        .detail("sdk_imported", imported)
        .detail("fixed", fixed))
}

/// What the agent needs to write the integration, from `connect` or `check`.
/// `key_file` is the file the key is in, as the developer knows it; `None`
/// when it is set in the shell.
pub(super) fn write_code(
    project: &Project,
    key_last4: &str,
    key_file: Option<&str>,
    say: impl Into<String>,
) -> Reply {
    let found = match key_file {
        Some(file) => format!("It's in {file}: {}.", project.loading(file)),
        None => "It's set in your shell, so the app gets it from the environment.".into(),
    };
    let reply = Reply::new(Next::WriteCode, say)
        .run(&project.next("check"))
        .detail("key_last4", key_last4)
        .detail("sdk_install", project.sdk_install())
        .detail(
            "load_key",
            format!(
                "The SDK reads {API_KEY_ENV} from the environment; never put the key in the code. {found}"
            ),
        )
        .detail("voice_id", DEFAULT_VOICE_ID)
        .detail(
            "what_to_build",
            "What the user asked for. If neither the conversation nor the project shows it, ask them before writing any code; don't pick a product for them.",
        );
    match key_file {
        Some(file) => reply.detail("env_file", file),
        None => reply,
    }
}

/// No key anywhere: the developer is mid-paste, or it's time to sign in.
pub(super) fn no_key(project: &Project) -> Result<Reply, CliError> {
    // A sign-in is still open for this project: wait on it, don't start another.
    // (No home folder means no sign-in could have started.)
    if let Ok(session) = Session::for_project(&project.dir) {
        if session.running() {
            return Ok(connect::waiting(&session, project, false));
        }
    }
    let file = &project.env_display;
    let placeholder = env_file::read(&project.env_file)
        .ok()
        .and_then(|c| env_file::file_key(&c))
        .is_some();
    if placeholder {
        return Ok(Reply::new(
            Next::AskUser,
            format!(
                "{API_KEY_ENV} in {file} is still empty. Paste the key after the = (never into this chat), and tell me when it's done."
            ),
        )
        .run(&project.next("check")));
    }
    if project.remote() {
        return manual::reply(project, &project.env_file, NO_BROWSER);
    }
    Ok(Reply::new(
        Next::Run,
        format!(
            "There's no ElevenLabs API key in this project yet, so {} to create one.",
            project.approval()
        ),
    )
    .run(&project.next("connect")))
}

/// `None` when the key works; otherwise what to do about it. Shared with `init`.
pub(super) fn key_problem(
    ctx: &AppContext,
    project: &Project,
    key: &Key,
) -> Result<Option<Reply>, CliError> {
    let ending = last4(&key.value);
    Ok(match check_key(ctx, &key.value) {
        KeyCheck::Works => None,
        KeyCheck::Unanswered(why) => Some(
            Reply::new(
                Next::AskUser,
                format!("I couldn't check your key: {why}. Tell me when you'd like me to try again."),
            )
            .run(&project.next("check")),
        ),
        KeyCheck::Rejected => Some(match key.file(project) {
            None => Reply::new(
                Next::AskUser,
                format!(
                    "{API_KEY_ENV} is set in your shell (ending in {ending}), and ElevenLabs rejects it. Unset it \
                     (and remove it from your shell profile), then tell me to check again."
                ),
            )
            .run(&project.next("check")),
            Some(file) if project.remote() => manual::reply(
                project,
                file,
                &format!("ElevenLabs rejects the key ending in {ending}."),
            )?,
            Some(file) => Reply::new(
                Next::Run,
                format!(
                    "ElevenLabs rejects the key ending in {ending}, so {} to create a new one.",
                    project.approval()
                ),
            )
            .run(&project.next(&format!(
                "connect --replace --env-file {}",
                shell_arg(&project.show(file))
            ))),
        }),
    })
}
