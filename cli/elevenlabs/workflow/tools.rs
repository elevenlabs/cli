//! The `tools` command group: add/delete/push/pull. Ports v0's
//! `src/tools/`. Tool configs are stored as raw wire JSON and pushed
//! verbatim (create/update wrap them as `{"tool_config": ...}`).

use std::path::Path;

use fern_cli_sdk::app::CliApp;
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::AppContext;
use serde_json::{json, Value};

use super::util::{downcast_ctx, dry_run_flag, opt_string, plan_pull_action, PullAction};
use super::{api, project, verify};

/// Register the `tools` command group.
pub fn register(app: CliApp) -> CliApp {
    app.command_under_typed_with(
        &["tools"],
        clap::Command::new("add")
            .about("Add a new tool: create config, upload to ElevenLabs, and save its ID"),
        handle_add,
    )
    .command_under_typed_with(
        &["tools"],
        clap::Command::new("delete").about("Delete a tool locally and in ElevenLabs"),
        handle_delete,
    )
    .command_under(
        &["tools"],
        push_command(),
        Box::new(|matches, ctx| handle_push(matches, downcast_ctx(ctx)?)),
    )
    .command_under(
        &["tools"],
        pull_command(),
        Box::new(|matches, ctx| handle_pull(matches, downcast_ctx(ctx)?)),
    )
}

// ── shared helpers ──────────────────────────────────────────────────

fn require_tools() -> Result<project::ToolsConfig, CliError> {
    if !Path::new(project::TOOLS_FILE).exists() {
        return Err(CliError::Validation(
            "tools.json not found. Run 'elevenlabs agents init' first.".to_string(),
        ));
    }
    project::load_tools()
}

fn tool_name_from_config(config_path: &str) -> Option<String> {
    project::read_value_in_project(config_path)
        .ok()
        .and_then(|v| v.get("name").and_then(Value::as_str).map(String::from))
}

/// Extract a tool id from a create/get/list item, tolerating `id`,
/// `tool_id`, and `toolId` spellings.
fn tool_id_of(value: &Value) -> Option<String> {
    ["id", "tool_id", "toolId"]
        .iter()
        .find_map(|k| value.get(*k).and_then(Value::as_str))
        .map(String::from)
}

fn default_webhook_tool(name: &str) -> Value {
    json!({
        "name": name,
        "description": format!("{name} webhook tool"),
        "type": "webhook",
        "api_schema": {
            "url": "https://api.example.com/webhook",
            "method": "POST",
            "request_body_schema": {
                "type": "object",
                "description": "Request body for the webhook",
                "properties": {}
            },
            "request_headers": { "Content-Type": "application/json" }
        },
        "response_timeout_secs": 30,
        "dynamic_variables": { "dynamic_variable_placeholders": {} },
        "assignments": [],
        "disable_interruptions": false,
        "force_pre_tool_speech": false
    })
}

fn default_client_tool(name: &str) -> Value {
    json!({
        "name": name,
        "description": format!("{name} client tool"),
        "type": "client",
        "expects_response": false,
        "response_timeout_secs": 30,
        "parameters": {
            "type": "object",
            "description": "Parameters for the client tool",
            "properties": {}
        },
        "dynamic_variables": { "dynamic_variable_placeholders": {} }
    })
}

// ── add ─────────────────────────────────────────────────────────────

#[derive(clap::Args)]
struct AddArgs {
    /// Name of the tool to create.
    name: String,
    /// Tool type: webhook or client.
    #[arg(long = "type", default_value = "webhook")]
    tool_type: String,
    /// Custom output path for the config file.
    #[arg(long = "config-path")]
    config_path: Option<String>,
}

fn handle_add(args: AddArgs, ctx: &AppContext) -> Result<(), CliError> {
    let config = match args.tool_type.as_str() {
        "webhook" => default_webhook_tool(&args.name),
        "client" => default_client_tool(&args.name),
        other => {
            return Err(CliError::Validation(format!(
                "Unknown tool type '{other}'. Use 'webhook' or 'client'."
            )))
        }
    };

    // Load or initialize tools.json.
    let mut registry = if Path::new(project::TOOLS_FILE).exists() {
        project::load_tools()?
    } else {
        let registry = project::ToolsConfig::default();
        project::save_tools(&registry)?;
        println!("Created {}", project::TOOLS_FILE);
        registry
    };

    println!(
        "Creating {} tool '{}' in ElevenLabs...",
        args.tool_type, args.name
    );
    let response = api::create_tool(ctx, &config)?;
    let tool_id = tool_id_of(&response).ok_or_else(|| {
        CliError::Other(anyhow::anyhow!(
            "Tool create response did not contain an id: {response}"
        ))
    })?;
    println!(
        "Created {} tool in ElevenLabs with ID: {tool_id}",
        args.tool_type
    );

    let config_path = match &args.config_path {
        Some(path) => path.clone(),
        None => project::generate_unique_filename(project::TOOL_CONFIGS_DIR, &args.name, ".json")
            .display()
            .to_string(),
    };
    project::write_json_in_project(&config_path, &config)?;
    println!("Created config file: {config_path}");

    registry.tools.push(project::ToolDefinition {
        tool_type: args.tool_type.clone(),
        config: config_path.clone(),
        id: Some(tool_id),
    });
    project::save_tools(&registry)?;
    println!("Added tool '{}' to tools.json", args.name);
    println!("Edit {config_path} to customize your tool, then run 'elevenlabs tools push' to update");
    Ok(())
}

// ── delete ──────────────────────────────────────────────────────────

#[derive(clap::Args)]
struct DeleteArgs {
    /// The tool ID to delete (omit with --all).
    tool_id: Option<String>,
    /// Delete every configured tool.
    #[arg(long)]
    all: bool,
    /// Skip the confirmation prompt (for --all).
    #[arg(long)]
    yes: bool,
}

fn handle_delete(args: DeleteArgs, ctx: &AppContext) -> Result<(), CliError> {
    let mut registry = require_tools()?;

    if args.all {
        if registry.tools.is_empty() {
            println!("No tools found to delete");
            return Ok(());
        }
        println!("\nFound {} tool(s) to delete:", registry.tools.len());
        for (i, tool) in registry.tools.iter().enumerate() {
            let name = tool_name_from_config(&tool.config)
                .or_else(|| tool.id.clone())
                .unwrap_or_else(|| "Unknown".to_string());
            println!(
                "  {}. {name} ({})",
                i + 1,
                tool.id.as_deref().unwrap_or("no id")
            );
        }
        if !args.yes {
            println!(
                "\nWARNING: This will delete ALL tools from both local configuration and ElevenLabs."
            );
            if !project::prompt_confirm("Are you sure you want to delete these tools?")? {
                println!("Deletion cancelled");
                return Ok(());
            }
        }
        println!("\nDeleting tools...\n");
        for tool in &registry.tools {
            let name = tool_name_from_config(&tool.config)
                .or_else(|| tool.id.clone())
                .unwrap_or_else(|| "Unknown".to_string());
            println!(
                "Deleting '{name}' ({})...",
                tool.id.as_deref().unwrap_or("no id")
            );
            match &tool.id {
                Some(id) => match api::delete_tool(ctx, id) {
                    Ok(()) => println!("  ✓ Deleted from ElevenLabs"),
                    Err(e) => eprintln!("  Warning: Failed to delete from ElevenLabs: {e}"),
                },
                None => println!("  Warning: No tool ID found, skipping ElevenLabs deletion"),
            }
            remove_config_file(&tool.config);
        }
        registry.tools.clear();
        project::save_tools(&registry)?;
        println!("\n✓ Deleted all tools");
        return Ok(());
    }

    let Some(tool_id) = args.tool_id else {
        return Err(CliError::Validation(
            "Provide a tool ID to delete, or pass --all.".to_string(),
        ));
    };

    let index = registry
        .tools
        .iter()
        .position(|t| t.id.as_deref() == Some(tool_id.as_str()))
        .ok_or_else(|| {
            CliError::Validation(format!(
                "Tool with ID '{tool_id}' not found in local configuration"
            ))
        })?;
    let removed = registry.tools.remove(index);
    let name = tool_name_from_config(&removed.config).unwrap_or_else(|| tool_id.clone());

    println!("Deleting tool '{name}' (ID: {tool_id})...");
    println!("Deleting from ElevenLabs...");
    match api::delete_tool(ctx, &tool_id) {
        Ok(()) => println!("✓ Successfully deleted from ElevenLabs"),
        Err(e) => {
            eprintln!("Warning: Failed to delete from ElevenLabs: {e}");
            println!("Continuing with local deletion...");
        }
    }
    project::save_tools(&registry)?;
    println!("✓ Removed '{name}' from tools.json");
    if remove_config_file(&removed.config) {
        println!("✓ Deleted config file: {}", removed.config);
    }
    println!("\n✓ Successfully deleted tool '{name}'");
    Ok(())
}

/// Delete a config file, refusing paths that escape the project or point at a
/// symlink. A hostile index file previously turned this into an arbitrary
/// unlink; failures are surfaced rather than swallowed.
fn remove_config_file(config_path: &str) -> bool {
    match project::remove_in_project(config_path) {
        Ok(removed) => removed,
        Err(e) => {
            eprintln!("  Warning: {e}");
            false
        }
    }
}

// ── push ────────────────────────────────────────────────────────────

fn push_command() -> clap::Command {
    clap::Command::new("push")
        .about("Push local tool configs to ElevenLabs")
        .arg(
            clap::Arg::new("tool")
                .long("tool")
                .help("Push only the tool with this ID"),
        )
}

fn handle_push(matches: &clap::ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let tool_filter = opt_string(matches, "tool");
    let dry_run = dry_run_flag(matches);

    let mut registry = require_tools()?;
    let indices: Vec<usize> = registry
        .tools
        .iter()
        .enumerate()
        .filter(|(_, t)| {
            tool_filter
                .as_ref()
                .map_or(true, |id| t.id.as_deref() == Some(id.as_str()))
        })
        .map(|(i, _)| i)
        .collect();
    if let Some(id) = &tool_filter {
        if indices.is_empty() {
            return Err(CliError::Validation(format!(
                "Tool with ID '{id}' not found in configuration"
            )));
        }
    }

    println!("Pushing {} tool(s) to ElevenLabs...", indices.len());
    let mut changes_made = false;

    for idx in indices {
        let config_path = registry.tools[idx].config.clone();
        let current_id = registry.tools[idx].id.clone();

        if config_path.is_empty() {
            println!("Warning: No config path specified");
            continue;
        }
        if !Path::new(&config_path).exists() {
            println!("Warning: Config file not found: {config_path}");
            continue;
        }
        let tool_config = match project::read_value_in_project(&config_path) {
            Ok(v) => v,
            Err(e) => {
                println!("Error reading config from {config_path}: {e}");
                continue;
            }
        };
        let name = tool_config
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or("Unnamed Tool")
            .to_string();

        println!("{name}: Will push (force override)");
        if dry_run {
            println!("[DRY RUN] Would update tool: {name}");
            continue;
        }

        match &current_id {
            None => match api::create_tool(ctx, &tool_config) {
                Ok(response) => match tool_id_of(&response) {
                    Some(new_id) => {
                        println!("Created tool {name} (ID: {new_id})");
                        verify::verify_tool_push(&name, &tool_config, &response);
                        registry.tools[idx].id = Some(new_id);
                        changes_made = true;
                    }
                    None => println!(
                        "Error: no tool ID in the API response for {name}; not saving to tools.json"
                    ),
                },
                Err(e) => println!("Error processing {name}: {e}"),
            },
            Some(id) => match api::update_tool(ctx, id, &tool_config) {
                Ok(response) => {
                    println!("Updated tool {name} (ID: {id})");
                    verify::verify_tool_push(&name, &tool_config, &response);
                    changes_made = true;
                }
                Err(e) => println!("Error processing {name}: {e}"),
            },
        }
    }

    if changes_made {
        project::save_tools(&registry)?;
    }
    Ok(())
}

// ── pull ────────────────────────────────────────────────────────────

fn pull_command() -> clap::Command {
    clap::Command::new("pull")
        .about("Pull tool configs from ElevenLabs")
        .arg(
            clap::Arg::new("tool")
                .long("tool")
                .help("Pull only the tool with this ID"),
        )
        .arg(
            clap::Arg::new("output-dir")
                .long("output-dir")
                .default_value("tool_configs")
                .help("Directory to write config files into"),
        )
        .arg(
            clap::Arg::new("update")
                .long("update")
                .action(clap::ArgAction::SetTrue)
                .help("Update existing tools only; skip new ones"),
        )
        .arg(
            clap::Arg::new("all")
                .long("all")
                .action(clap::ArgAction::SetTrue)
                .help("Pull everything (new and existing)"),
        )
}

fn handle_pull(matches: &clap::ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let tool_filter = opt_string(matches, "tool");
    let output_dir =
        opt_string(matches, "output-dir").unwrap_or_else(|| "tool_configs".to_string());
    let dry_run = dry_run_flag(matches);
    let update = matches.get_flag("update");
    let all = matches.get_flag("all");

    println!("Pulling tools from ElevenLabs...");

    let mut registry = if Path::new(project::TOOLS_FILE).exists() {
        project::load_tools()?
    } else {
        println!(
            "{} not found. Creating initial tools configuration...",
            project::TOOLS_FILE
        );
        let registry = project::ToolsConfig::default();
        project::save_tools(&registry)?;
        registry
    };

    // Build the remote work list: (tool_id, name).
    let remote: Vec<(String, String)> = if let Some(tool) = &tool_filter {
        println!("Pulling tool with ID: {tool}...");
        let details = api::get_tool(ctx, tool).map_err(|e| {
            CliError::Validation(format!("Failed to fetch tool with ID '{tool}': {e}"))
        })?;
        let id = tool_id_of(&details).unwrap_or_else(|| tool.clone());
        let name = details
            .get("tool_config")
            .and_then(|c| c.get("name"))
            .and_then(Value::as_str)
            .ok_or_else(|| CliError::Validation(format!("Tool with ID '{tool}' has no name")))?
            .to_string();
        println!("Found tool: {name}");
        vec![(id, name)]
    } else {
        println!("Pulling all tools from ElevenLabs...");
        let list = api::list_tools(ctx)?;
        if list.is_empty() {
            println!("No tools found in your ElevenLabs workspace.");
            return Ok(());
        }
        println!("Found {} tool(s)", list.len());
        list.iter()
            .filter_map(|t| {
                let id = tool_id_of(t)?;
                let name = t
                    .get("tool_config")
                    .and_then(|c| c.get("name"))
                    .and_then(Value::as_str)?;
                Some((id, name.to_string()))
            })
            .collect()
    };

    // Plan create / update / skip.
    let mut plan: Vec<(PullAction, String, String, Option<usize>)> = Vec::new();
    let (mut n_create, mut n_update, mut n_skip) = (0usize, 0usize, 0usize);
    for (id, name) in &remote {
        let existing = registry.tools.iter().position(|t| t.id.as_deref() == Some(id.as_str()));
        let action = plan_pull_action(existing.is_some(), update, all);
        match action {
            PullAction::Create => n_create += 1,
            PullAction::Update => n_update += 1,
            PullAction::Skip => n_skip += 1,
        }
        plan.push((action, id.clone(), name.clone(), existing));
    }

    println!("\nPlan: {n_create} create, {n_update} update, {n_skip} skip");
    if n_skip > 0 && !update && !all {
        if n_create == 0 {
            println!("\n💡 Tip: Use --update to update existing tools or --all to pull everything");
        } else {
            println!("\n💡 Tip: Use --all to also update existing tools");
        }
    }

    if !dry_run && (n_create > 0 || n_update > 0) && !project::prompt_confirm("Proceed?")? {
        println!("Pull cancelled");
        return Ok(());
    }

    let mut processed = 0;
    for (action, id, name, existing_idx) in &plan {
        if *action == PullAction::Skip {
            println!("⊘ Skipping '{name}' (already exists, use --update to overwrite)");
            continue;
        }
        if dry_run {
            let verb = if *action == PullAction::Update { "update" } else { "pull" };
            println!("[DRY RUN] Would {verb} tool: {name} (ID: {id})");
            continue;
        }

        let verb = if *action == PullAction::Update {
            "↻ Updating"
        } else {
            "+ Pulling"
        };
        println!("{verb} config for '{name}'...");

        let details = match api::get_tool(ctx, id) {
            Ok(v) => v,
            Err(e) => {
                println!("  ✗ Error pulling tool '{name}': {e}");
                continue;
            }
        };
        let Some(tool_config) = details.get("tool_config").cloned() else {
            println!("  ✗ Warning: No tool_config found for '{name}' - skipping");
            continue;
        };
        let tool_type = tool_config
            .get("type")
            .and_then(Value::as_str)
            .unwrap_or("unknown")
            .to_string();

        match existing_idx {
            Some(i) => {
                let cfg_path = registry.tools[*i].config.clone();
                project::write_json_in_project(&cfg_path, &tool_config)?;
                println!("  ✓ Updated '{name}' (config: {cfg_path})");
            }
            None => {
                let cfg_path = project::generate_unique_filename(&output_dir, name, ".json")
                    .display()
                    .to_string();
                project::write_json_in_project(&cfg_path, &tool_config)?;
                registry.tools.push(project::ToolDefinition {
                    tool_type: tool_type.clone(),
                    config: cfg_path.clone(),
                    id: Some(id.clone()),
                });
                println!("  ✓ Added '{name}' (config: {cfg_path}, type: {tool_type})");
            }
        }
        processed += 1;
    }

    if !dry_run && processed > 0 {
        project::save_tools(&registry)?;
        println!("\nUpdated {}", project::TOOLS_FILE);
    }

    if dry_run {
        println!("\n[DRY RUN] Would process {} tool(s)", n_create + n_update);
    } else {
        println!("\n✓ Summary: {n_create} created, {n_update} updated, {n_skip} skipped");
        if processed > 0 {
            println!(
                "You can now edit the config files in '{output_dir}/' and use them in your agents"
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_id_tolerates_every_spelling_the_api_uses() {
        assert_eq!(tool_id_of(&json!({ "id": "t1" })), Some("t1".to_string()));
        assert_eq!(tool_id_of(&json!({ "tool_id": "t2" })), Some("t2".to_string()));
        assert_eq!(tool_id_of(&json!({ "toolId": "t3" })), Some("t3".to_string()));
        assert_eq!(tool_id_of(&json!({ "name": "no id here" })), None);
    }

    #[test]
    fn tool_id_prefers_id_when_several_are_present() {
        assert_eq!(
            tool_id_of(&json!({ "id": "first", "tool_id": "second" })),
            Some("first".to_string())
        );
    }

    #[test]
    fn default_tools_carry_their_type_and_name() {
        let webhook = default_webhook_tool("My Hook");
        assert_eq!(webhook["name"], json!("My Hook"));
        assert_eq!(webhook["type"], json!("webhook"));
        assert_eq!(webhook["api_schema"]["method"], json!("POST"));

        let client = default_client_tool("My Client");
        assert_eq!(client["name"], json!("My Client"));
        assert_eq!(client["type"], json!("client"));
        assert_eq!(client["expects_response"], json!(false));
    }

    #[test]
    fn default_webhook_preserves_header_names_verbatim() {
        // Header names are user-facing keys; a casing transform would break them.
        let webhook = default_webhook_tool("H");
        assert_eq!(
            webhook["api_schema"]["request_headers"]["Content-Type"],
            json!("application/json")
        );
    }
}
