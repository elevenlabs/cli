//! `onboard test`: the test request the developer said yes to. One short
//! line of text to speech with the project's key, saved where the agent can
//! point them to play it. It never runs on its own: `check` only offers it.

use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::AppContext;
use serde_json::{json, Value};

use super::super::api::api_error_message;
use super::super::say::{run_async as block_on, DEFAULT_MODEL_ID, DEFAULT_VOICE_ID, MP3_FORMAT};
use super::auth;
use super::project::Project;
use super::reply::{Next, Reply};

const TEXT: &str = "Welcome to ElevenLabs";
const PLANS_PAGE: &str = "https://elevenlabs.io/app/subscription";

pub(super) fn handle(matches: &clap::ArgMatches, ctx: &AppContext) -> Result<Reply, CliError> {
    let project = Project::open(matches)?;
    let Some(key) = project.key() else {
        return Ok(Reply::new(
            Next::Run,
            "There's no ElevenLabs API key in this project yet.",
        )
        .run(&project.next("check")));
    };
    // Every outcome ends on what is set up, so the agent's last message says it.
    let location = key
        .file(&project)
        .map(|f| project.show(f))
        .unwrap_or_else(|| "your shell".into());
    let code = if project.sdk_declared() && project.sdk_imported() {
        " and the SDK is in your code"
    } else {
        ""
    };
    let set_up = format!("ElevenLabs is set up: your key is in {location}{code}.");
    let client = ctx.http_config().build_client()?;
    let url = format!(
        "{}/v1/text-to-speech/{DEFAULT_VOICE_ID}",
        auth::api_base(ctx)
    );
    let sent = block_on(
        client
            .post(&url)
            .query(&[("output_format", MP3_FORMAT)])
            .header("xi-api-key", &key.value)
            .json(&json!({ "text": TEXT, "model_id": DEFAULT_MODEL_ID }))
            .timeout(std::time::Duration::from_secs(60))
            .send(),
    );
    let response = match sent {
        Ok(r) => r,
        Err(_) => {
            return Ok(done(format!(
            "I couldn't reach ElevenLabs for the test request; ask me to try again later. {set_up}"
        ))
            .detail("status", Value::Null))
        }
    };
    let status = response.status().as_u16();
    let body = block_on(response.bytes()).unwrap_or_default();
    let error: Value = serde_json::from_slice(&body).unwrap_or(Value::Null);
    // An exhausted quota comes back as a 401 `quota_exceeded`, not only as a 402.
    let no_credits =
        status == 402 || (status == 401 && error["detail"]["status"] == "quota_exceeded");
    let reply = if (200..300).contains(&status) {
        let clip = save(&body)?;
        done(format!(
            "Your first request worked. Play {clip} to hear it. {set_up}"
        ))
        .detail("audio_file", clip)
    } else if no_credits {
        done(format!(
            "The test request needs credits on your account. Add some at {PLANS_PAGE}, then ask me to try again. {set_up}"
        ))
    } else {
        let message = if error.is_null() {
            "no details".to_string()
        } else {
            api_error_message(&error)
        };
        // Only a busy or failing ElevenLabs is worth trying again later.
        let later = if status == 429 || status >= 500 {
            "; ask me to try again later"
        } else {
            ""
        };
        done(format!(
            "The test request failed ({status}: {message}){later}. {set_up}"
        ))
    };
    Ok(reply.detail("status", status))
}

/// The clip in a new file of its own, so nothing already at that path is overwritten.
fn save(audio: &[u8]) -> Result<String, CliError> {
    let fail = |e: std::io::Error| CliError::Other(anyhow::anyhow!("save the test clip: {e}"));
    let mut file = tempfile::Builder::new()
        .prefix("elevenlabs-hello-")
        .suffix(".mp3")
        .tempfile()
        .map_err(fail)?;
    std::io::Write::write_all(&mut file, audio).map_err(fail)?;
    let (_, path) = file.keep().map_err(|e| fail(e.error))?;
    Ok(path.display().to_string())
}

fn done(say: impl Into<String>) -> Reply {
    Reply::new(Next::Done, say)
}
