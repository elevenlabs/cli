//! `onboard check` (also `onboard status`): the one way to verify the setup.
//! It stops at the first problem and says how to fix it, fixing on its own
//! only what is safe: the `.gitignore` entry and the file's permissions.

use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::AppContext;

use super::auth::{check_key, KeyCheck};
use super::connect;
use super::env_file::{self, API_KEY_ENV};
use super::manual::{self, NO_BROWSER};
use super::project::{last4, shell_arg, Key, Project};
use super::reply::{Next, Reply};
use super::session::Session;

pub(super) fn handle(matches: &clap::ArgMatches, ctx: &AppContext) -> Result<Reply, CliError> {
    let answered = matches
        .try_get_one::<bool>("answered")
        .ok()
        .flatten()
        .copied()
        .unwrap_or(false);
    verify(ctx, &Project::open(matches)?, answered)
}

/// The whole check, for `check` and for `init` when the project already has a
/// key, so the next step comes straight from it. `answered`: the developer has
/// said what to build (`--answered`, which the question's `run` passes).
pub(super) fn verify(ctx: &AppContext, project: &Project, answered: bool) -> Result<Reply, CliError> {
    let Some(key) = project.key() else {
        return no_key(project);
    };
    if let Some(reply) = key_problem(ctx, project, &key)? {
        return Ok(reply);
    }
    let mut fixed = Vec::new();
    if let Some(file) = key.file(project) {
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
    let key_file = key.file(project).map(|f| project.show(f));
    let wired = declared && imported;
    let file = key_file.as_deref();
    let reply = match (wired, answered) {
        (true, _) => offer_test(project, &ending, file),
        (false, false) => ask_what_to_build(project, &ending, file, "Your key works."),
        (false, true) => write_code(project, &ending, file)?,
    };
    Ok(reply
        .detail("api_key_source", key.source_name())
        .detail("sdk_declared", declared)
        .detail("sdk_imported", imported)
        .detail("fixed", fixed))
}

/// Everything is in place: offer the test request.
fn offer_test(project: &Project, ending: &str, key_file: Option<&str>) -> Reply {
    let location = key_file.unwrap_or("your shell");
    Reply::new(
        Next::OfferTest,
        format!(
            "ElevenLabs is set up: a working key ending in {ending} in {location}, and the SDK in your code. \
             Want me to send a test request? It makes a short text-to-speech clip with your key and uses a few credits."
        ),
    )
    .run(&project.next("test"))
    .detail("key_last4", ending)
}

/// Asked every time, so the agent never picks a product for the developer.
pub(super) const WHAT_TO_BUILD: &str =
    "What would you like to build with ElevenLabs, or should I just add the SDK for now?";

/// The ElevenLabs skills, the whole repo in one download.
const SKILLS_ARCHIVE: &str = "https://github.com/elevenlabs/skills/archive/refs/heads/main.tar.gz";

/// The question, from `connect` or `check`, whenever the code doesn't use the
/// SDK yet: `say` ends with [`WHAT_TO_BUILD`]. Its `run` passes `--answered`,
/// so the next reply, right after the answer, is how to write the code.
/// `key_file` is the file the key is in, as the developer knows it; `None`
/// when it is set in the shell.
pub(super) fn ask_what_to_build(project: &Project, key_last4: &str, key_file: Option<&str>, say: &str) -> Reply {
    let reply = Reply::new(Next::AskWhatToBuild, format!("{say} {WHAT_TO_BUILD}"))
        .run(&project.next("check --answered"))
        .detail("key_last4", key_last4);
    match key_file {
        Some(file) => reply.detail("env_file", file),
        None => reply,
    }
}

/// What the agent needs to write the integration once the developer has said
/// what they want: the SDK, a new temp folder for the skills, and the key.
fn write_code(project: &Project, key_last4: &str, key_file: Option<&str>) -> Result<Reply, CliError> {
    let dir = skills_dir()?;
    let reply = Reply::new(
        Next::WriteCode,
        "Next I'll add the ElevenLabs SDK to the project and use it in the code.",
    )
    .detail("sdk_install", project.sdk_install())
    .detail("skills_dir", dir.display().to_string())
    .detail("fetch_skills", fetch_skills(&dir, cfg!(windows)));
    // Still `--answered`: code that doesn't use the SDK yet is no reason to ask again. The rules
    // come again with the code, a turn after the reply that started the flow.
    Ok(key_details(reply, project, key_last4, key_file, "check --answered").with_rules())
}

/// A first key for an app that already uses the SDK: nothing to build, only
/// the key to load.
pub(super) fn load_key(project: &Project, key_last4: &str, key_file: &str, say: String) -> Reply {
    key_details(Reply::new(Next::LoadKey, say), project, key_last4, Some(key_file), "check").with_rules()
}

/// Where the key is and how the app gets it, then `step`.
fn key_details(reply: Reply, project: &Project, key_last4: &str, key_file: Option<&str>, step: &str) -> Reply {
    let found = match key_file {
        Some(file) => format!("It's in {file}. {}", project.loading(file)),
        None => "It's set in your shell, so the app gets it from the environment.".into(),
    };
    let reply = reply
        .run(&project.next(step))
        .detail("key_last4", key_last4)
        .detail(
            "load_key",
            format!(
                "The SDK reads {API_KEY_ENV} from the environment; never put the key in the code. {found}"
            ),
        );
    match key_file {
        Some(file) => reply.detail("env_file", file),
        None => reply,
    }
}

/// A new folder for the skills in the system's temp folder, never the
/// project, so nothing lands in the developer's repo. Its name is random, so
/// no one can have put anything there first.
fn skills_dir() -> Result<std::path::PathBuf, CliError> {
    tempfile::Builder::new()
        .prefix("elevenlabs-skills-")
        .tempdir()
        .map(tempfile::TempDir::keep)
        .map_err(|e| CliError::Other(anyhow::anyhow!("create a folder for the ElevenLabs skills: {e}")))
}

/// The command that downloads the skills into `dir`, one folder per skill.
/// Two commands, not `curl | tar`: Windows PowerShell 5.1 corrupts binary
/// pipes, and its `curl` is another program, hence `curl.exe`.
pub(super) fn fetch_skills(dir: &std::path::Path, windows: bool) -> String {
    let archive = dir.join("skills.tar.gz");
    if windows {
        let (archive, dir) = (archive.display(), dir.display());
        format!(
            "curl.exe -fsSL -o \"{archive}\" {SKILLS_ARCHIVE}; tar -xzf \"{archive}\" -C \"{dir}\" --strip-components=1"
        )
    } else {
        let archive = shell_arg(&archive.display().to_string());
        let dir = shell_arg(&dir.display().to_string());
        format!(
            "curl -fsSL -o {archive} {SKILLS_ARCHIVE} && tar -xzf {archive} -C {dir} --strip-components=1"
        )
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
