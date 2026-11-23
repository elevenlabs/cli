//! `onboard check` (also `onboard status`): verify the key, and the one reply
//! that hands the rest to the coding agent. The CLI never reads the project's
//! code to decide what comes next: once a working key is in place, the agent
//! is told where it is and what the app needs, and builds from there. On its
//! own the CLI fixes only what is safe: the `.gitignore` entry and the file's
//! permissions.

use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::AppContext;

use super::super::settings;
use super::auth::{check_key, sign_in_config, KeyCheck};
use super::env_file::{self, API_KEY_ENV};
use super::manual::{self, NO_BROWSER};
use super::project::{last4, shell_arg, Key, Project};
use super::reply::{Next, Reply};

pub(super) fn handle(matches: &clap::ArgMatches, ctx: &AppContext) -> Result<Reply, CliError> {
    let project = Project::open(matches)?;
    let reply = verify(ctx, &project)?;
    // `--written`: the code is done, so a working key ends on the test offer.
    if matches.get_flag("written") && reply.next == Next::Integrate {
        return Ok(offer_test(&project, &reply));
    }
    Ok(reply.with_rules())
}

/// The ending, once the code is written: the question the developer answers, and the test
/// request as `run`. `working` is the reply for the key that was just checked.
fn offer_test(project: &Project, working: &Reply) -> Reply {
    let offer = Reply::new(Next::OfferTest, format!("{} {TEST_OFFER}", working.say)).run(&project.next("test"));
    match working.details.get("key_last4") {
        Some(last4) => offer.detail("key_last4", last4),
        None => offer,
    }
}

/// The whole check, for `check` and for `init` when the project already has a
/// key: the key works and git can't commit it, or what to do about it.
pub(super) fn verify(ctx: &AppContext, project: &Project) -> Result<Reply, CliError> {
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
    let ending = last4(&key.value);
    let key_file = key.file(project).map(|f| project.show(f));
    let location = key_file.as_deref().unwrap_or("your shell");
    let say = format!("Your ElevenLabs key ending in {ending} works. It's in {location}.");
    Ok(integrate(project, &ending, key_file.as_deref(), &say)?
        .detail("api_key_source", key.source_name())
        .detail("fixed", fixed))
}

/// Asked only when the developer hasn't said, so the agent never picks a product for them.
pub(super) const WHAT_TO_BUILD: &str =
    "What would you like to build with ElevenLabs, or should I just add the SDK for now?";

/// What the developer is told about the test request before they answer.
const TEST_OFFER: &str = "Want me to send a test request? It makes a short text-to-speech clip with your key and uses a few credits.";

/// The ElevenLabs skills, the whole repo in one download.
const SKILLS_ARCHIVE: &str = "https://github.com/elevenlabs/skills/archive/refs/heads/main.tar.gz";

/// The one reply after a working key is in place: where the key is, what the
/// app needs to get it, where the skills are, and the test request to offer
/// at the end. `key_file` is the file the key is in, as the developer knows
/// it; `None` when it is set in the shell. `run` is the check that ends on the test offer.
pub(super) fn integrate(
    project: &Project,
    key_last4: &str,
    key_file: Option<&str>,
    say: &str,
) -> Result<Reply, CliError> {
    let dir = skills_dir()?;
    let app = app_origin()?;
    let reply = Reply::new(Next::Integrate, say)
        .run(&project.next("check --written"))
        .detail("key_last4", key_last4)
        .detail("question", WHAT_TO_BUILD)
        .detail("sdk_install", project.sdk_install())
        .detail("load_key", project.load_key(key_file))
        .detail("skills_dir", dir.display().to_string())
        .detail("fetch_skills", fetch_skills(&dir, cfg!(windows)))
        .detail(
            "agent_links",
            format!(
                "Its page: {app}/app/agents/agents/<agent_id>. To talk to it, when anyone with the \
                 link may: {app}/app/talk-to?agent_id=<agent_id>."
            ),
        );
    Ok(match key_file {
        Some(file) => reply.detail("env_file", file),
        None => reply,
    })
}

/// The web app the developer's agents live in: their data residency
/// region's, or else the one the sign-in opens.
fn app_origin() -> Result<String, CliError> {
    let authorize = &sign_in_config()?.authorize_url;
    app_origin_for(&settings::read_residency(), authorize)
}

pub(super) fn app_origin_for(residency: &str, authorize_url: &str) -> Result<String, CliError> {
    // The regions `settings::base_url_for` routes to; anything else uses the global API.
    let region = match residency {
        "eu-residency" => Some("eu"),
        "in-residency" => Some("in"),
        "sg-residency" => Some("sg"),
        _ => None,
    };
    if let Some(region) = region {
        return Ok(format!("https://{region}.residency.elevenlabs.io"));
    }
    reqwest::Url::parse(authorize_url)
        .map(|u| u.origin().ascii_serialization())
        .map_err(|e| CliError::Other(anyhow::anyhow!("read the sign-in page's address: {e}")))
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

/// `None` when the key works; otherwise what to do about it.
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
            Some(_) => Reply::new(
                Next::Run,
                format!(
                    "ElevenLabs rejects the key ending in {ending}, so {} to create a new one.",
                    project.approval()
                ),
            )
            .run(&project.next("connect --replace")),
        }),
    })
}
