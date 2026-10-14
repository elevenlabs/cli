//! The one shape every step prints, so an agent follows the flow without
//! interpreting prose: what to do next, the exact command, and what to tell
//! the developer. `say` is always written to the developer; what the agent
//! does is `next`, spelled out in `instruction`, and `run`.

use fern_cli_sdk::error::CliError;
use serde::Serialize;
use serde_json::{Map, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum Next {
    Run,
    Wait,
    AskUser,
    Fix,
    AskWhatToBuild,
    WriteCode,
    LoadKey,
    OfferTest,
    Done,
}

impl Next {
    /// What the agent does with the reply. Printed with every reply, so the
    /// flow needs nothing but the CLI's output.
    fn instruction(self) -> &'static str {
        match self {
            Next::Run => "Tell the user `say`, then run `run`.",
            Next::Wait => {
                "Show the user `say`, then run `run` right away, without ending your turn: it waits while they \
                 approve in the browser."
            }
            Next::AskUser => {
                "Ask the user what `say` says and wait for their answer, then run `run`, unless they said no."
            }
            Next::Fix => "Do what `say` describes, then run `run`.",
            Next::AskWhatToBuild => {
                "Tell the user `say`, then end your turn and wait for their answer to its question. Ask \
                 it even if they already said what they want, even just the SDK: their answer confirms \
                 it. Write no code before they answer. When they have, run `run` before anything else, \
                 without installing or writing anything first: its reply says how to write the code. If \
                 you already asked and they answered, don't ask again, just run `run`."
            }
            Next::WriteCode => {
                "Tell the user `say`, then write what they asked for. If they only want the SDK, install \
                 it (`details.sdk_install`) and create the client in one small file. Otherwise first run \
                 `details.fetch_skills`: it downloads the ElevenLabs skills into `details.skills_dir`, one \
                 folder per skill, outside the project, so list and read it with shell commands like `ls` \
                 and `cat` (file search tools may not look there). If one fits what they want, read its \
                 SKILL.md and the `references/` \
                 files it links before writing any code, and write the code the way the skill shows, \
                 not from the SDK's own files; if none fits, write it without one. Install the SDK, and \
                 make the app load the key the way `details.load_key` says. Then run `run` to check it."
            }
            Next::LoadKey => {
                "Tell the user `say`. The code already uses the SDK, but the app must load the key from \
                 where it is now: do what `details.load_key` says and change nothing else. Then run `run` \
                 to check it."
            }
            Next::OfferTest => {
                "End your reply with the question in `say`, as it is, and stop there: until they answer, \
                 don't run the app, its start script or anything else that uses the key. Run `run` only if \
                 they say yes."
            }
            Next::Done => "Tell the user `say`. There's nothing left to run.",
        }
    }
}

/// Never relaxed by any step, so every reply that starts a flow carries them.
const RULES: &[&str] = &[
    "Never open, print or cat the env file; run `onboard check` to verify it.",
    "Never ask for an API key in chat, and never accept one pasted there.",
    "Never put the key in code; the SDK reads ELEVENLABS_API_KEY.",
    "Never run git clean or delete untracked files.",
    "Don't write or change any code until a reply's `next` is `write_code` or `load_key`.",
    "Put anything you download only to read, like a package or an archive, in a temp folder, never in the project.",
    "Never run the app (its start script or anything else that calls ElevenLabs) or make your own API calls with the key until the user says yes to the test request. Once the code is written, run `onboard check` before you finish: it verifies the key and the code.",
];

#[derive(Debug, Serialize)]
pub(super) struct Reply {
    pub next: Next,
    pub instruction: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<String>,
    pub say: String,
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub rules: &'static [&'static str],
    #[serde(skip_serializing_if = "Map::is_empty")]
    pub details: Map<String, Value>,
}

impl Reply {
    pub fn new(next: Next, say: impl Into<String>) -> Self {
        Self {
            next,
            instruction: next.instruction(),
            run: None,
            say: say.into(),
            rules: &[],
            details: Map::new(),
        }
    }

    /// The next command, `onboard <args>`, spelled the way this run was invoked.
    pub fn run(mut self, args: &str) -> Self {
        self.run = Some(command(args));
        self
    }

    pub fn detail(mut self, key: &str, value: impl Serialize) -> Self {
        self.details.insert(
            key.into(),
            serde_json::to_value(value).unwrap_or(Value::Null),
        );
        self
    }

    pub fn with_rules(mut self) -> Self {
        self.rules = RULES;
        self
    }

    pub fn to_value(&self) -> Value {
        serde_json::to_value(self).unwrap_or(Value::Null)
    }
}

/// `onboard <args>` as the developer's shell would run it: under npx when npx
/// started this process (it sets `npm_command=exec`), so the agent keeps
/// running the same, latest CLI; otherwise the installed `elevenlabs`.
pub(super) fn command(args: &str) -> String {
    let cli = if std::env::var("npm_command").is_ok_and(|c| c == "exec") {
        "npx -y @elevenlabs/cli@latest"
    } else {
        "elevenlabs"
    };
    format!("{cli} onboard {args}")
}

/// Print a reply (or one a background sign-in saved). Always JSON, whatever
/// stdout is and whatever `--human` or `--format` says: only agents run these
/// steps, often in pseudo-terminals, and prose would drop the instruction.
pub(super) fn emit(reply: &Value) -> Result<(), CliError> {
    let text = serde_json::to_string_pretty(reply)
        .map_err(|e| CliError::Other(anyhow::anyhow!("{e}")))?;
    println!("{text}");
    Ok(())
}
