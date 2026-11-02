//! `onboard check` (also `onboard status`): the one way to verify the setup.
//! It stops at the first problem and says how to fix it, fixing on its own
//! only what is safe: the `.gitignore` entry and the file's permissions.

use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::AppContext;
use serde_json::Value;

use super::super::say::run_async as block_on;
use super::super::settings;
use super::auth::{api_base, check_key, sign_in_config, KeyCheck};
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
    let wired = (declared && imported) || project.embeds_agent_widget();
    let reads_key = wired && project.reads_key();
    let file = key_file.as_deref();
    // The code reads the key, but it only gets it if the app loads the file it's in.
    let unloaded = key
        .file(project)
        .filter(|f| reads_key && !project.loads_key_file(f));
    let reply = match (wired, answered, unloaded) {
        (true, _, Some(unloaded)) => {
            let name = project.show(unloaded);
            let say = format!("Your key works and the code uses the SDK, but nothing in the app loads {name} yet.");
            load_key(project, &ending, &name, say)
        }
        (true, _, None) => match own_agent(ctx, project, &key) {
            Some(agent) => agent_ready(&ending, file, &agent, reads_key)?,
            None => offer_test(project, &ending, file),
        },
        (false, false, _) => ask_what_to_build(project, &ending, file, "Your key works."),
        (false, true, _) => write_code(project, &ending, file)?,
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
            "ElevenLabs is set up: a working key ending in {ending} in {location}, and ElevenLabs in your code. \
             Want me to send a test request? It makes a short text-to-speech clip with your key and uses a few credits."
        ),
    )
    .run(&project.next("test"))
    .detail("key_last4", ending)
}

/// An agent the code names that this key can open.
struct Agent {
    id: String,
    name: String,
    access: Access,
}

/// Who can start a conversation with an agent.
#[derive(PartialEq)]
enum Access {
    /// Anyone with its link: the talk-to page works.
    Public,
    /// Only the sites on its allowlist, which the talk-to page isn't.
    Allowlist,
    /// Only with a signed URL or token from a server holding the key.
    Private,
}

/// How many of the agent IDs in the code to look up before giving up.
const AGENT_LOOKUPS: usize = 3;

/// The first agent the code names that belongs to the key's account, so a
/// sample's or another account's agent is never offered. `None` when there
/// is none, or the key can't read agents.
fn own_agent(ctx: &AppContext, project: &Project, key: &Key) -> Option<Agent> {
    let ids = project.agent_ids(AGENT_LOOKUPS, key.file(project));
    if ids.is_empty() {
        return None;
    }
    let client = ctx.http_config().build_client().ok()?;
    let base = api_base(ctx);
    for id in ids {
        let sent = block_on(
            client
                .get(format!("{base}/v1/convai/agents/{id}"))
                .header("xi-api-key", &key.value)
                .timeout(std::time::Duration::from_secs(10))
                .send(),
        );
        let Ok(response) = sent else { return None };
        let status = response.status();
        // The key can't read agents at all, so no other ID will do better.
        if status == 401 || status == 403 {
            return None;
        }
        if !status.is_success() {
            continue;
        }
        let Ok(body) = block_on(response.json::<Value>()) else {
            continue;
        };
        let auth = &body["platform_settings"]["auth"];
        let access = if auth["enable_auth"] == Value::Bool(true) {
            Access::Private
        } else if auth["allowlist"].as_array().is_some_and(|a| !a.is_empty()) {
            Access::Allowlist
        } else {
            Access::Public
        };
        return Some(Agent {
            name: body["name"].as_str().unwrap_or_default().trim().to_string(),
            access,
            id,
        });
    }
    None
}

/// The code has the developer's agent in it: the test is talking to it,
/// which the developer does in the browser, so there's nothing left to run.
/// `reads_key`: the app has code that reads the key, which a private agent
/// needs to sign each conversation.
fn agent_ready(
    ending: &str,
    key_file: Option<&str>,
    agent: &Agent,
    reads_key: bool,
) -> Result<Reply, CliError> {
    let app = app_origin()?;
    let page = format!("{app}/app/agents/agents/{}", agent.id);
    let talk_to = format!("{app}/app/talk-to?agent_id={}", agent.id);
    let location = key_file.unwrap_or("your shell");
    let named = if agent.name.is_empty() {
        "your agent".to_string()
    } else {
        format!("your agent \"{}\"", agent.name)
    };
    let next = match agent.access {
        Access::Public => format!("Talk to it at {talk_to}, and change it at {page}."),
        Access::Allowlist => format!(
            "Only the sites on its allowlist can connect to it, so try it from its page: {page}."
        ),
        Access::Private if reads_key => {
            format!("It needs a signed URL to talk to it, so try it from its page: {page}.")
        }
        Access::Private => format!(
            "It requires authentication, so your page can't connect to it yet: turn authentication \
             off on its page ({page}), or have your server request a signed URL for each conversation."
        ),
    };
    let reply = Reply::new(
        Next::Done,
        format!("ElevenLabs is set up: a working key ending in {ending} in {location}, and {named} in your code. {next}"),
    )
    .detail("key_last4", ending)
    .detail("agent_id", &agent.id)
    .detail("agent_url", page);
    Ok(if agent.access == Access::Public {
        reply.detail("talk_to_url", talk_to)
    } else {
        reply
    })
}

/// The web app the developer's agents live in: their data residency
/// region's, or else the one the sign-in opens.
fn app_origin() -> Result<String, CliError> {
    let authorize = &sign_in_config()?.authorize_url;
    app_origin_for(&settings::read_residency(), authorize)
}

pub(super) fn app_origin_for(residency: &str, authorize_url: &str) -> Result<String, CliError> {
    if let Some(region) = residency.strip_suffix("-residency") {
        return Ok(format!("https://{region}.residency.elevenlabs.io"));
    }
    reqwest::Url::parse(authorize_url)
        .map(|u| u.origin().ascii_serialization())
        .map_err(|e| CliError::Other(anyhow::anyhow!("read the sign-in page's address: {e}")))
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
                "connect --replace --env-path {}",
                shell_arg(&project.show(file))
            ))),
        }),
    })
}
