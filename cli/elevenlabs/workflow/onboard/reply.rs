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
    AskUser,
    Fix,
    Integrate,
    OfferTest,
    Done,
}

impl Next {
    /// What the agent does with the reply. Printed with every reply, so the
    /// flow needs nothing but the CLI's output.
    fn instruction(self) -> &'static str {
        match self {
            Next::Run => "Tell the user `say`, then run `run`.",
            Next::AskUser => {
                "Ask the user what `say` says and wait for their answer, then run `run`, unless they said no."
            }
            Next::Fix => "Do what `say` describes, then run `run`.",
            // The whole of what comes after the key, in one reply: the CLI never
            // reads the code to decide a next step, the coding agent knows the project.
            Next::Integrate => {
                "Tell the user `say`. The key is in place, so the rest is yours: build what the user asked \
                 for with ElevenLabs. Only a request that says what the app should do with ElevenLabs \
                 says what to build; one that asks to add, set up or use ElevenLabs, or for something \
                 general like voice features, doesn't. If theirs doesn't, ask `details.question`, as it \
                 is, and end your turn: write no code and install nothing until they answer, and don't \
                 pick for them. If they only want the SDK, install it (`details.sdk_install`) and create \
                 the client in one small file. Otherwise first run `details.fetch_skills`: it downloads \
                 the ElevenLabs skills into `details.skills_dir`, one folder per skill, outside the \
                 project, so list and read it with shell commands like `ls` and `cat` (file search tools \
                 may not look there). If one fits, read its SKILL.md and the `references/` files it links \
                 before writing any code, and write the code the way the skill shows, not from the SDK's \
                 own files; if none fits, write it without one. Install the SDK. Whatever they asked \
                 for, even nothing new, the app has to get the key: look at how it starts, and if \
                 nothing loads the env file, make it load it the way `details.load_key` says. That is \
                 part of the setup, not new work. If it needs something made in ElevenLabs first, \
                 like an agent, make it yourself with the key (load the env file in the command, never \
                 print it), and when you're done give the user the links in `details.agent_links` with \
                 its ID filled in. When the code is written (or if it already was), run `run`: its \
                 reply says how to finish."
            }
            // The last word before the developer decides, handed over at the moment it is said.
            Next::OfferTest => {
                "Say what you set up, how to run it and anything the user still has to do themselves, \
                 like installing the packages you added. Then end your reply with the question in \
                 `say`, as it is, and end your turn. If they say yes, run `run`: that command is the \
                 test request, so don't run their app in its place. If they say no, run nothing."
            }
            Next::Done => {
                "Tell the user `say`, then anything they still have to do themselves, like installing the \
                 packages you added, in one or two concrete steps. There's nothing left to run."
            }
        }
    }
}

/// Never relaxed by any step, so every reply that starts or finishes the key step carries them.
const RULES: &[&str] = &[
    "Never open, print or cat the env file; run `onboard check` to verify the key.",
    "Never ask for an API key in chat, and never accept one pasted there.",
    "Never put the key in code, or in anything that runs in the browser; the SDK reads ELEVENLABS_API_KEY.",
    "Never run git clean or delete untracked files.",
    "Put anything you download only to read, like a package or an archive, in a temp folder, never in the project.",
    "Don't run the app (or its start script) or make speech to try it until the user says yes to the test request. API calls that what they asked for needs, like creating an agent, are fine.",
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
/// steps, often in pseudo-terminals, and prose would drop the instruction. On
/// one line, so an agent that pipes it through `grep` or `tail` keeps all of it.
pub(super) fn emit(reply: &Value) -> Result<(), CliError> {
    let text = serde_json::to_string(reply)
        .map_err(|e| CliError::Other(anyhow::anyhow!("{e}")))?;
    println!("{text}");
    Ok(())
}
