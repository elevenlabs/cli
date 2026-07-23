//! Built-in agent templates and the `agents templates` command group.
//! Ports v0's `src/agents/templates.ts`.

use fern_cli_sdk::app::CliApp;
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::AppContext;
use serde_json::{json, Value};

/// `(name, description)` for every built-in agent template, in display
/// order. Mirrors v0's `getTemplateOptions()`.
pub const TEMPLATE_OPTIONS: &[(&str, &str)] = &[
    (
        "default",
        "Complete configuration with all available fields and sensible defaults",
    ),
    ("minimal", "Minimal configuration with only essential fields"),
    ("voice-only", "Optimized for voice-only conversations"),
    ("text-only", "Optimized for text-only conversations"),
    (
        "customer-service",
        "Pre-configured for customer service scenarios",
    ),
    ("assistant", "General purpose AI assistant configuration"),
];

// ── Template bodies ─────────────────────────────────────────────────

/// The complete default agent configuration as raw JSON, with an empty
/// `name`/`prompt` filled in by [`default_template`]. Stored as a parsed
/// string rather than the `json!` macro because the structure is deep
/// enough to blow the macro's recursion limit (and the crate root, where
/// `#![recursion_limit]` would live, is generated).
const DEFAULT_TEMPLATE_JSON: &str = r##"{
  "name": "",
  "conversation_config": {
    "asr": { "quality": "high", "provider": "scribe_realtime", "user_input_audio_format": "pcm_16000", "keywords": [] },
    "turn": { "turn_timeout": 7.0, "silence_end_call_timeout": -1.0, "mode": "turn" },
    "tts": {
      "model_id": "eleven_flash_v2_5",
      "voice_id": "cjVigY5qzO86Huf0OWal",
      "supported_voices": [],
      "agent_output_audio_format": "pcm_16000",
      "optimize_streaming_latency": 3,
      "stability": 0.5,
      "speed": 1.0,
      "similarity_boost": 0.8,
      "pronunciation_dictionary_locators": []
    },
    "conversation": { "text_only": false, "max_duration_seconds": 600, "client_events": ["audio", "interruption"] },
    "language_presets": {},
    "agent": {
      "first_message": "",
      "language": "en",
      "dynamic_variables": { "dynamic_variable_placeholders": {} },
      "prompt": {
        "prompt": "",
        "llm": "gemini-2.5-flash",
        "temperature": 0.0,
        "max_tokens": -1,
        "tool_ids": [],
        "mcp_server_ids": [],
        "native_mcp_server_ids": [],
        "knowledge_base": [],
        "ignore_default_personality": false,
        "rag": {
          "enabled": false,
          "embedding_model": "e5_mistral_7b_instruct",
          "max_vector_distance": 0.6,
          "max_documents_length": 50000,
          "max_retrieved_rag_chunks_count": 20
        },
        "custom_llm": null
      }
    }
  },
  "platform_settings": {
    "auth": { "enable_auth": false, "allowlist": [], "shareable_token": null },
    "evaluation": { "criteria": [] },
    "widget": {
      "variant": "full",
      "placement": "bottom-right",
      "expandable": "never",
      "avatar": { "type": "orb", "color_1": "#2792dc", "color_2": "#9ce6e6" },
      "feedback_mode": "none",
      "bg_color": "#ffffff",
      "text_color": "#000000",
      "btn_color": "#000000",
      "btn_text_color": "#ffffff",
      "border_color": "#e1e1e1",
      "focus_color": "#000000",
      "shareable_page_show_terms": true,
      "show_avatar_when_collapsed": false,
      "disable_banner": false,
      "mic_muting_enabled": false,
      "transcript_enabled": false,
      "text_input_enabled": true,
      "text_contents": {
        "main_label": null, "start_call": null, "new_call": null, "end_call": null,
        "mute_microphone": null, "change_language": null, "collapse": null, "expand": null,
        "copied": null, "accept_terms": null, "dismiss_terms": null, "listening_status": null,
        "speaking_status": null, "connecting_status": null, "input_label": null,
        "input_placeholder": null, "user_ended_conversation": null, "agent_ended_conversation": null,
        "conversation_id": null, "error_occurred": null, "copy_id": null
      },
      "language_selector": false,
      "supports_text_only": true,
      "language_presets": {},
      "styles": {
        "base": null, "base_hover": null, "base_active": null, "base_border": null,
        "base_subtle": null, "base_primary": null, "base_error": null, "accent": null,
        "accent_hover": null, "accent_active": null, "accent_border": null, "accent_subtle": null,
        "accent_primary": null, "overlay_padding": null, "button_radius": null, "input_radius": null,
        "bubble_radius": null, "sheet_radius": null, "compact_sheet_radius": null, "dropdown_sheet_radius": null
      },
      "border_radius": null, "btn_radius": null, "action_text": null, "start_call_text": null,
      "end_call_text": null, "expand_text": null, "listening_text": null, "speaking_text": null,
      "shareable_page_text": null, "terms_text": null, "terms_html": null, "terms_key": null,
      "override_link": null, "custom_avatar_path": null
    },
    "data_collection": {},
    "overrides": {
      "conversation_config_override": {
        "tts": { "voice_id": false },
        "conversation": { "text_only": true },
        "agent": { "first_message": false, "language": false, "prompt": { "prompt": false } }
      },
      "custom_llm_extra_body": false,
      "enable_conversation_initiation_client_data_from_webhook": false
    },
    "call_limits": { "agent_concurrency_limit": -1, "daily_limit": 100000, "bursting_enabled": true },
    "privacy": {
      "record_voice": true, "retention_days": -1, "delete_transcript_and_pii": false,
      "delete_audio": false, "apply_to_existing_conversations": false, "zero_retention_mode": false
    },
    "workspace_overrides": {
      "webhooks": { "post_call_webhook_id": null },
      "conversation_initiation_client_data_webhook": null
    },
    "safety": { "is_blocked_ivc": false, "is_blocked_non_ivc": false, "ignore_safety_evaluation": false },
    "testing": { "attached_tests": [] },
    "ban": null
  },
  "tags": []
}"##;

/// The complete default agent configuration. Ports
/// `getDefaultAgentTemplate`.
fn default_template(name: &str) -> Value {
    let mut t: Value =
        serde_json::from_str(DEFAULT_TEMPLATE_JSON).expect("DEFAULT_TEMPLATE_JSON is valid JSON");
    t["name"] = Value::String(name.to_string());
    t["conversation_config"]["agent"]["prompt"]["prompt"] =
        Value::String(format!("You are {name}, a helpful AI assistant."));
    t
}

/// Minimal configuration — ports `getMinimalAgentTemplate`.
fn minimal_template(name: &str) -> Value {
    json!({
        "name": name,
        "conversation_config": {
            "agent": {
                "prompt": {
                    "prompt": format!("You are {name}, a helpful AI assistant."),
                    "llm": "gemini-2.5-flash",
                    "temperature": 0.0
                },
                "language": "en"
            },
            "conversation": {
                "text_only": false
            },
            "tts": {
                "model_id": "eleven_flash_v2_5",
                "voice_id": "cjVigY5qzO86Huf0OWal"
            }
        },
        "platform_settings": {},
        "tags": []
    })
}

/// Voice-only — ports `getVoiceOnlyTemplate`.
fn voice_only_template(name: &str) -> Value {
    let mut t = default_template(name);
    t["conversation_config"]["conversation"]["text_only"] = json!(false);
    t["platform_settings"]["widget"]["supports_text_only"] = json!(false);
    t["platform_settings"]["widget"]["text_input_enabled"] = json!(false);
    t
}

/// Text-only — ports `getTextOnlyTemplate`.
fn text_only_template(name: &str) -> Value {
    let mut t = default_template(name);
    t["conversation_config"]["conversation"]["text_only"] = json!(true);
    t["platform_settings"]["widget"]["supports_text_only"] = json!(true);
    t["platform_settings"]["overrides"]["conversation_config_override"]["conversation"]
        ["text_only"] = json!(false);
    t
}

/// Customer service — ports `getCustomerServiceTemplate`.
fn customer_service_template(name: &str) -> Value {
    let mut t = default_template(name);
    t["conversation_config"]["agent"]["prompt"]["prompt"] = json!(format!(
        "You are {name}, a helpful customer service representative. You are professional, \
         empathetic, and focused on solving customer problems efficiently."
    ));
    t["conversation_config"]["agent"]["prompt"]["temperature"] = json!(0.1);
    t["conversation_config"]["conversation"]["max_duration_seconds"] = json!(1800);
    t["platform_settings"]["call_limits"]["daily_limit"] = json!(10000);
    t["platform_settings"]["evaluation"]["criteria"] = json!([
        "Helpfulness",
        "Professionalism",
        "Problem Resolution",
        "Response Time"
    ]);
    t["tags"] = json!(["customer-service"]);
    t
}

/// General assistant — ports `getAssistantTemplate`.
fn assistant_template(name: &str) -> Value {
    let mut t = default_template(name);
    t["conversation_config"]["agent"]["prompt"]["prompt"] = json!(format!(
        "You are {name}, a knowledgeable and helpful AI assistant. You can help with a wide \
         variety of tasks including answering questions, providing explanations, helping with \
         analysis, and creative tasks."
    ));
    t["conversation_config"]["agent"]["prompt"]["temperature"] = json!(0.3);
    t["conversation_config"]["agent"]["prompt"]["max_tokens"] = json!(1000);
    t["tags"] = json!(["assistant", "general-purpose"]);
    t
}

/// Build a template by name. Ports `getTemplateByName`; errors on an
/// unknown template type with the list of available ones.
pub fn template_by_name(name: &str, template_type: &str) -> Result<Value, CliError> {
    match template_type {
        "default" => Ok(default_template(name)),
        "minimal" => Ok(minimal_template(name)),
        "voice-only" => Ok(voice_only_template(name)),
        "text-only" => Ok(text_only_template(name)),
        "customer-service" => Ok(customer_service_template(name)),
        "assistant" => Ok(assistant_template(name)),
        other => {
            let available: Vec<&str> = TEMPLATE_OPTIONS.iter().map(|(n, _)| *n).collect();
            Err(CliError::Validation(format!(
                "Unknown template type '{other}'. Available: {}",
                available.join(", ")
            )))
        }
    }
}

// ── Commands ────────────────────────────────────────────────────────

#[derive(clap::Args)]
struct TemplatesListArgs {}

fn handle_list(_args: TemplatesListArgs, _ctx: &AppContext) -> Result<(), CliError> {
    println!("Available agent templates:");
    println!("{}", "=".repeat(50));
    for (name, description) in TEMPLATE_OPTIONS {
        println!("\n{name}");
        println!("   {description}");
    }
    println!(
        "\nUse 'elevenlabs agents add <name> --template <template_name>' \
         to create an agent with a specific template"
    );
    Ok(())
}

#[derive(clap::Args)]
struct TemplatesShowArgs {
    /// Template name to display.
    template: String,
}

fn handle_show(args: TemplatesShowArgs, _ctx: &AppContext) -> Result<(), CliError> {
    let template = template_by_name("Example", &args.template)?;
    println!("Template: {}", args.template);
    println!("{}", "=".repeat(50));
    // v0 prints the template config with a 2-space indent.
    println!(
        "{}",
        serde_json::to_string_pretty(&template)
            .map_err(|e| CliError::Other(anyhow::anyhow!("Could not render template: {e}")))?
    );
    Ok(())
}

/// Register the `agents templates` command group.
pub fn register(app: CliApp) -> CliApp {
    app.command_under_typed_with(
        &["agents", "templates"],
        clap::Command::new("list").about("List available agent templates"),
        handle_list,
    )
    .command_under_typed_with(
        &["agents", "templates"],
        clap::Command::new("show").about("Show a template's full configuration"),
        handle_show,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_by_name_covers_all_options() {
        for (name, _) in TEMPLATE_OPTIONS {
            let t = template_by_name("Example", name).expect("template should build");
            assert_eq!(t["name"], json!("Example"));
        }
    }

    #[test]
    fn unknown_template_errors() {
        assert!(template_by_name("Example", "nope").is_err());
    }

    #[test]
    fn text_only_flips_conversation_flag() {
        let t = template_by_name("Example", "text-only").unwrap();
        assert_eq!(
            t["conversation_config"]["conversation"]["text_only"],
            json!(true)
        );
        assert_eq!(
            t["platform_settings"]["overrides"]["conversation_config_override"]["conversation"]
                ["text_only"],
            json!(false)
        );
    }

    #[test]
    fn customer_service_sets_tags_and_criteria() {
        let t = template_by_name("Example", "customer-service").unwrap();
        assert_eq!(t["tags"], json!(["customer-service"]));
        assert_eq!(
            t["conversation_config"]["agent"]["prompt"]["temperature"],
            json!(0.1)
        );
    }
}
