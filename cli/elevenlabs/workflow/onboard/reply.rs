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
    WriteCode,
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
            Next::WriteCode => {
                "Tell the user `say`. Know what they're building first (`details.what_to_build` says when \
                 to ask). Before writing any code, read the ElevenLabs skill for that product: run \
                 `curl -fsSL https://raw.githubusercontent.com/elevenlabs/skills/main/<skill>/SKILL.md`, \
                 where <skill> is one of agents, dubbing, music, sound-effects, speech-engine, \
                 speech-to-text, text-to-speech, voice-changer, voice-isolator, and the same way each \
                 `references/` file it links. Then install the SDK, write the code the way the skill \
                 shows, and run `run` to check it."
            }
            Next::OfferTest => "Ask the user the question in `say`; run `run` only if they say yes.",
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
    "Don't run the app or make your own API calls to try the key unless the user says yes. Once the code is written, run `onboard check` before you finish: it verifies the key and the code.",
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
/// stdout is, since agents run these steps in pseudo-terminals too; prose only
/// when a person asks for it with `--human` or `--format table`.
pub(super) fn emit(reply: &Value, human: bool) -> Result<(), CliError> {
    if !human {
        let text = serde_json::to_string_pretty(reply)
            .map_err(|e| CliError::Other(anyhow::anyhow!("{e}")))?;
        println!("{text}");
        return Ok(());
    }
    println!("{}", reply["say"].as_str().unwrap_or_default());
    if let Some(run) = reply["run"].as_str() {
        println!("\nNext: {run}");
    }
    Ok(())
}
