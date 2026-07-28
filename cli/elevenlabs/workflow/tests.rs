//! The `tests` command group: add/delete/push/pull, with push-time
//! auto-discovery of untracked test config files. Ports v0's `src/tests/`.
//!
//! Test configs are stored as raw wire JSON and pushed verbatim.

use std::path::Path;

use fern_cli_sdk::app::CliApp;
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::AppContext;
use serde_json::{json, Value};

use super::util::{downcast_ctx, dry_run_flag, opt_string};
use super::{api, project};

/// Register the `tests` command group.
pub fn register(app: CliApp) -> CliApp {
    app.command_under_typed_with(
        &["tests"],
        clap::Command::new("add")
            .about("Add a new test: create config, upload to ElevenLabs, and save its ID"),
        handle_add,
    )
    .command_under_typed_with(
        &["tests"],
        clap::Command::new("delete").about("Delete a test locally and in ElevenLabs"),
        handle_delete,
    )
    .command_under(
        &["tests"],
        push_command(),
        Box::new(|matches, ctx| handle_push(matches, downcast_ctx(ctx)?)),
    )
    .command_under(
        &["tests"],
        pull_command(),
        Box::new(|matches, ctx| handle_pull(matches, downcast_ctx(ctx)?)),
    )
    .command_under_typed_with(
        &["tests", "templates"],
        clap::Command::new("list").about("List available test templates"),
        handle_templates_list,
    )
}

// ── Templates ───────────────────────────────────────────────────────

/// `(name, description)` for every built-in test template, in display
/// order. Mirrors v0's `getTestTemplateOptions()`.
pub const TEST_TEMPLATE_OPTIONS: &[(&str, &str)] = &[
    ("basic-llm", "Basic LLM response test with simple user input"),
    ("tool", "Tool usage test to verify agent calls specific tools"),
    ("conversation-flow", "Multi-turn conversation flow test"),
    (
        "customer-service",
        "Customer service scenario with empathy testing",
    ),
];

fn basic_llm_template(name: &str) -> Value {
    json!({
        "name": name,
        "chat_history": [
            { "role": "user", "time_in_call_secs": 1, "message": "Hello" }
        ],
        "success_condition": "The agent responds in a helpful and professional manner",
        "success_examples": [
            { "response": "Hello! How can I help you today?", "type": "success" },
            { "response": "Hi there! I'm here to assist you.", "type": "success" }
        ],
        "failure_examples": [
            { "response": "I don't understand", "type": "failure" },
            { "response": "Error", "type": "failure" }
        ],
        "type": "llm"
    })
}

fn tool_template(name: &str) -> Value {
    // v0 defaults: tool name "example_tool", id "tool_123" — edit the config
    // after creation to point at a real tool.
    let tool_name = "example_tool";
    json!({
        "name": name,
        "chat_history": [
            { "role": "user", "time_in_call_secs": 1, "message": "Please use the tool" }
        ],
        "success_condition": format!("The agent successfully calls the {tool_name} tool"),
        "success_examples": [
            { "response": format!("I'll use the {tool_name} tool to help you with that."), "type": "success" }
        ],
        "failure_examples": [
            { "response": "I don't have access to that tool", "type": "failure" }
        ],
        "tool_call_parameters": {
            "parameters": [
                { "eval": { "type": "anything" }, "path": "tool_name" }
            ],
            "referenced_tool": { "id": "tool_123", "type": "webhook" },
            "verify_absence": false
        },
        "type": "tool"
    })
}

fn conversation_flow_template(name: &str) -> Value {
    json!({
        "name": name,
        "chat_history": [
            { "role": "user", "time_in_call_secs": 1, "message": "Hello, I need help with my account" },
            { "role": "agent", "time_in_call_secs": 3, "message": "I'd be happy to help you with your account. What specific issue are you experiencing?" },
            { "role": "user", "time_in_call_secs": 5, "message": "I can't log in" }
        ],
        "success_condition": "The agent provides helpful troubleshooting steps for login issues",
        "success_examples": [
            { "response": "Let me help you troubleshoot your login issue. First, please try resetting your password.", "type": "success" }
        ],
        "failure_examples": [
            { "response": "I don't know how to help with that", "type": "failure" }
        ],
        "type": "llm"
    })
}

fn customer_service_template(name: &str) -> Value {
    json!({
        "name": name,
        "chat_history": [
            { "role": "user", "time_in_call_secs": 1, "message": "I'm frustrated with my recent order. It arrived damaged and I want a refund." }
        ],
        "success_condition": "The agent responds with empathy, acknowledges the issue, and offers a solution",
        "success_examples": [
            { "response": "I'm really sorry to hear about the damaged order. I understand how frustrating that must be. Let me help you get this resolved right away with a full refund.", "type": "success" }
        ],
        "failure_examples": [
            { "response": "That's not my problem", "type": "failure" },
            { "response": "You'll have to contact someone else", "type": "failure" }
        ],
        "type": "llm"
    })
}

/// Build a test template by name. Ports v0's `getTestTemplateByName`.
pub fn test_template_by_name(name: &str, template_type: &str) -> Result<Value, CliError> {
    match template_type {
        "basic-llm" => Ok(basic_llm_template(name)),
        "tool" => Ok(tool_template(name)),
        "conversation-flow" => Ok(conversation_flow_template(name)),
        "customer-service" => Ok(customer_service_template(name)),
        other => {
            let available: Vec<&str> = TEST_TEMPLATE_OPTIONS.iter().map(|(n, _)| *n).collect();
            Err(CliError::Validation(format!(
                "Unknown test template type '{other}'. Available: {}",
                available.join(", ")
            )))
        }
    }
}

#[derive(clap::Args)]
struct TemplatesListArgs {}

fn handle_templates_list(_args: TemplatesListArgs, _ctx: &AppContext) -> Result<(), CliError> {
    println!("Available test templates:");
    println!("{}", "=".repeat(50));
    for (name, description) in TEST_TEMPLATE_OPTIONS {
        println!("\n{name}");
        println!("   {description}");
    }
    println!("\nUse 'elevenlabs tests add <name> --template <template_name>' to create a test");
    Ok(())
}

// ── Shared helpers ──────────────────────────────────────────────────

fn require_tests() -> Result<project::TestsConfig, CliError> {
    if !Path::new(project::TESTS_FILE).exists() {
        return Err(CliError::Validation(
            "tests.json not found. Run 'elevenlabs agents init' first.".to_string(),
        ));
    }
    project::load_tests()
}

fn test_name_from_config(config_path: &str) -> Option<String> {
    project::read_value(Path::new(config_path))
        .ok()
        .and_then(|v| v.get("name").and_then(Value::as_str).map(String::from))
}

/// Extract a test id from a create/get/list item, tolerating `id` and
/// `test_id` spellings.
fn test_id_of(value: &Value) -> Option<String> {
    ["id", "test_id"]
        .iter()
        .find_map(|k| value.get(*k).and_then(Value::as_str))
        .map(String::from)
}

/// Does this JSON look like a test config? Ports v0's `isTestConfigFile`:
/// a `chat_history` array or a `success_condition` string.
fn looks_like_test_config(config: &Value) -> bool {
    config.get("chat_history").map(Value::is_array) == Some(true)
        || config.get("success_condition").map(Value::is_string) == Some(true)
}

/// Register any `.json` under `config_dir` that isn't already tracked and
/// structurally looks like a test. Returns how many were added. Ports v0's
/// `registerDiscoveredTestConfigs`.
fn register_discovered(registry: &mut project::TestsConfig, config_dir: &str) -> usize {
    let tracked: std::collections::HashSet<std::path::PathBuf> = registry
        .tests
        .iter()
        .filter_map(|t| std::fs::canonicalize(&t.config).ok())
        .collect();

    let mut added = 0;
    for candidate in project::discover_json_files(config_dir) {
        let canonical = std::fs::canonicalize(&candidate).ok();
        if let Some(c) = &canonical {
            if tracked.contains(c) {
                continue;
            }
        }
        let config = match project::read_value(Path::new(&candidate)) {
            Ok(v) => v,
            Err(e) => {
                println!("Warning: Skipping unreadable test config {candidate}: {e}");
                continue;
            }
        };
        if !looks_like_test_config(&config) {
            println!("Warning: Skipping non-test JSON file {candidate}");
            continue;
        }
        registry.tests.push(project::TestDefinition {
            config: candidate,
            test_type: config
                .get("type")
                .and_then(Value::as_str)
                .map(String::from),
            id: config.get("id").and_then(Value::as_str).map(String::from),
        });
        added += 1;
    }
    added
}

fn remove_config_file(config_path: &str) -> bool {
    let path = Path::new(config_path);
    path.exists() && std::fs::remove_file(path).is_ok()
}

// ── add ─────────────────────────────────────────────────────────────

#[derive(clap::Args)]
struct AddArgs {
    /// Name of the test to create.
    name: String,
    /// Template to use (basic-llm, tool, conversation-flow, customer-service).
    #[arg(long, default_value = "basic-llm")]
    template: String,
    /// Custom output path for the config file.
    #[arg(long = "config-path")]
    config_path: Option<String>,
}

fn handle_add(args: AddArgs, ctx: &AppContext) -> Result<(), CliError> {
    let config = test_template_by_name(&args.name, &args.template)?;

    let mut registry = if Path::new(project::TESTS_FILE).exists() {
        project::load_tests()?
    } else {
        let registry = project::TestsConfig::default();
        project::save_tests(&registry)?;
        println!("Created {}", project::TESTS_FILE);
        registry
    };

    println!("Creating test '{}' in ElevenLabs...", args.name);
    let response = api::create_test(ctx, &config)?;
    let test_id = test_id_of(&response).ok_or_else(|| {
        CliError::Other(anyhow::anyhow!(
            "Test create response did not contain an id: {response}"
        ))
    })?;
    println!("Created test in ElevenLabs with ID: {test_id}");

    let config_path = match &args.config_path {
        Some(path) => path.clone(),
        None => project::generate_unique_filename(project::TEST_CONFIGS_DIR, &args.name, ".json")
            .display()
            .to_string(),
    };
    project::write_json(Path::new(&config_path), &config)?;
    println!("Created config file: {config_path} (template: {})", args.template);

    registry.tests.push(project::TestDefinition {
        config: config_path.clone(),
        test_type: config
            .get("type")
            .and_then(Value::as_str)
            .map(String::from),
        id: Some(test_id),
    });
    project::save_tests(&registry)?;
    println!("Added test '{}' to tests.json", args.name);
    println!("Edit {config_path} to customize your test, then run 'elevenlabs tests push' to update");
    Ok(())
}

// ── delete ──────────────────────────────────────────────────────────

#[derive(clap::Args)]
struct DeleteArgs {
    /// The test ID to delete (omit with --all).
    test_id: Option<String>,
    /// Delete every configured test.
    #[arg(long)]
    all: bool,
    /// Skip the confirmation prompt (for --all).
    #[arg(long)]
    yes: bool,
}

fn handle_delete(args: DeleteArgs, ctx: &AppContext) -> Result<(), CliError> {
    let mut registry = require_tests()?;

    if args.all {
        if registry.tests.is_empty() {
            println!("No tests found to delete");
            return Ok(());
        }
        println!("\nFound {} test(s) to delete:", registry.tests.len());
        for (i, test) in registry.tests.iter().enumerate() {
            let name = test_name_from_config(&test.config)
                .or_else(|| test.id.clone())
                .unwrap_or_else(|| "Unknown".to_string());
            println!(
                "  {}. {name} ({})",
                i + 1,
                test.id.as_deref().unwrap_or("no id")
            );
        }
        if !args.yes {
            println!(
                "\nWARNING: This will delete ALL tests from both local configuration and ElevenLabs."
            );
            if !project::prompt_confirm("Are you sure you want to delete these tests?")? {
                println!("Deletion cancelled");
                return Ok(());
            }
        }
        println!("\nDeleting tests...\n");
        for test in &registry.tests {
            let name = test_name_from_config(&test.config)
                .or_else(|| test.id.clone())
                .unwrap_or_else(|| "Unknown".to_string());
            println!(
                "Deleting '{name}' ({})...",
                test.id.as_deref().unwrap_or("no id")
            );
            match &test.id {
                Some(id) => match api::delete_test(ctx, id) {
                    Ok(()) => println!("  ✓ Deleted from ElevenLabs"),
                    Err(e) => eprintln!("  Warning: Failed to delete from ElevenLabs: {e}"),
                },
                None => println!("  Warning: No test ID found, skipping ElevenLabs deletion"),
            }
            remove_config_file(&test.config);
        }
        registry.tests.clear();
        project::save_tests(&registry)?;
        println!("\n✓ Deleted all tests");
        return Ok(());
    }

    let Some(test_id) = args.test_id else {
        return Err(CliError::Validation(
            "Provide a test ID to delete, or pass --all.".to_string(),
        ));
    };

    let index = registry
        .tests
        .iter()
        .position(|t| t.id.as_deref() == Some(test_id.as_str()))
        .ok_or_else(|| {
            CliError::Validation(format!(
                "Test with ID '{test_id}' not found in local configuration"
            ))
        })?;
    let removed = registry.tests.remove(index);
    let name = test_name_from_config(&removed.config).unwrap_or_else(|| test_id.clone());

    println!("Deleting test '{name}' (ID: {test_id})...");
    println!("Deleting from ElevenLabs...");
    match api::delete_test(ctx, &test_id) {
        Ok(()) => println!("✓ Successfully deleted from ElevenLabs"),
        Err(e) => {
            eprintln!("Warning: Failed to delete from ElevenLabs: {e}");
            println!("Continuing with local deletion...");
        }
    }
    project::save_tests(&registry)?;
    println!("✓ Removed '{name}' from tests.json");
    if remove_config_file(&removed.config) {
        println!("✓ Deleted config file: {}", removed.config);
    }
    println!("\n✓ Successfully deleted test '{name}'");
    Ok(())
}

// ── push ────────────────────────────────────────────────────────────

fn push_command() -> clap::Command {
    clap::Command::new("push")
        .about("Push local test configs to ElevenLabs")
        .arg(
            clap::Arg::new("test")
                .long("test")
                .help("Push only the test with this ID"),
        )
        .arg(
            clap::Arg::new("config-dir")
                .long("config-dir")
                .default_value("test_configs")
                .help("Directory scanned for untracked test configs to auto-register"),
        )
}

fn handle_push(matches: &clap::ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let test_filter = opt_string(matches, "test");
    let config_dir =
        opt_string(matches, "config-dir").unwrap_or_else(|| "test_configs".to_string());
    let dry_run = dry_run_flag(matches);

    let tests_file_exists = Path::new(project::TESTS_FILE).exists();
    let mut registry = if tests_file_exists {
        project::load_tests()?
    } else {
        project::TestsConfig::default()
    };

    // Auto-register any untracked test configs before pushing.
    let discovered = register_discovered(&mut registry, &config_dir);
    if discovered > 0 {
        println!("Discovered {discovered} test config(s) in {config_dir}");
    }
    if !tests_file_exists && registry.tests.is_empty() {
        return Err(CliError::Validation(format!(
            "tests.json not found and no test configs found in {config_dir}. \
             Run 'elevenlabs tests add' first."
        )));
    }

    let indices: Vec<usize> = registry
        .tests
        .iter()
        .enumerate()
        .filter(|(_, t)| {
            test_filter
                .as_ref()
                .map_or(true, |id| t.id.as_deref() == Some(id.as_str()))
        })
        .map(|(i, _)| i)
        .collect();
    if let Some(id) = &test_filter {
        if indices.is_empty() {
            return Err(CliError::Validation(format!(
                "Test with ID '{id}' not found in configuration"
            )));
        }
    }

    println!("Pushing {} test(s) to ElevenLabs...", indices.len());
    let mut changes_made = discovered > 0 && !dry_run;

    for idx in indices {
        let config_path = registry.tests[idx].config.clone();
        let current_id = registry.tests[idx].id.clone();

        if !Path::new(&config_path).exists() {
            println!("Warning: Config file not found: {config_path}");
            continue;
        }
        let test_config = match project::read_value(Path::new(&config_path)) {
            Ok(v) => v,
            Err(e) => {
                println!("Error reading config from {config_path}: {e}");
                continue;
            }
        };
        let name = test_config
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or("Unnamed Test")
            .to_string();

        println!("{name}: Will push (force override)");
        if dry_run {
            let verb = if current_id.is_some() { "update" } else { "create" };
            println!("[DRY RUN] Would {verb} test: {name}");
            continue;
        }

        match &current_id {
            None => match api::create_test(ctx, &test_config) {
                Ok(response) => match test_id_of(&response) {
                    Some(new_id) => {
                        println!("Created test {name} (ID: {new_id})");
                        registry.tests[idx].id = Some(new_id);
                        changes_made = true;
                    }
                    None => println!(
                        "Error: no test ID in the API response for {name}; not saving to tests.json"
                    ),
                },
                Err(e) => println!("Error processing {name}: {e}"),
            },
            Some(id) => match api::update_test(ctx, id, &test_config) {
                Ok(_) => {
                    println!("Updated test {name} (ID: {id})");
                    changes_made = true;
                }
                Err(e) => println!("Error processing {name}: {e}"),
            },
        }
    }

    if changes_made {
        project::save_tests(&registry)?;
    }
    Ok(())
}

// ── pull ────────────────────────────────────────────────────────────

fn pull_command() -> clap::Command {
    clap::Command::new("pull")
        .about("Pull test configs from ElevenLabs")
        .arg(
            clap::Arg::new("test")
                .long("test")
                .help("Pull only the test with this ID"),
        )
        .arg(
            clap::Arg::new("output-dir")
                .long("output-dir")
                .default_value("test_configs")
                .help("Directory to write config files into"),
        )
        .arg(
            clap::Arg::new("update")
                .long("update")
                .action(clap::ArgAction::SetTrue)
                .help("Update existing tests only; skip new ones"),
        )
        .arg(
            clap::Arg::new("all")
                .long("all")
                .action(clap::ArgAction::SetTrue)
                .help("Pull everything (new and existing)"),
        )
}

#[derive(Clone, Copy, PartialEq)]
enum PullAction {
    Create,
    Update,
    Skip,
}

fn handle_pull(matches: &clap::ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let test_filter = opt_string(matches, "test");
    let output_dir =
        opt_string(matches, "output-dir").unwrap_or_else(|| "test_configs".to_string());
    let dry_run = dry_run_flag(matches);
    let update = matches.get_flag("update");
    let all = matches.get_flag("all");

    println!("Pulling tests from ElevenLabs...");

    let mut registry = if Path::new(project::TESTS_FILE).exists() {
        project::load_tests()?
    } else {
        println!(
            "{} not found. Creating initial tests configuration...",
            project::TESTS_FILE
        );
        let registry = project::TestsConfig::default();
        project::save_tests(&registry)?;
        registry
    };

    // Build the remote work list: (test_id, name).
    let remote: Vec<(String, String)> = if let Some(test) = &test_filter {
        println!("Pulling test with ID: {test}...");
        let details = api::get_test(ctx, test).map_err(|e| {
            CliError::Validation(format!("Failed to fetch test with ID '{test}': {e}"))
        })?;
        let id = test_id_of(&details).unwrap_or_else(|| test.clone());
        let name = details
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        println!("Found test: {name}");
        vec![(id, name)]
    } else {
        println!("Pulling all tests from ElevenLabs...");
        let list = api::list_tests(ctx)?;
        if list.is_empty() {
            println!("No tests found in your ElevenLabs workspace.");
            return Ok(());
        }
        println!("Found {} test(s)", list.len());
        list.iter()
            .filter_map(|t| {
                let id = test_id_of(t)?;
                let name = t.get("name").and_then(Value::as_str).unwrap_or("").to_string();
                Some((id, name))
            })
            .collect()
    };

    // Plan create / update / skip.
    let mut plan: Vec<(PullAction, String, String, Option<usize>)> = Vec::new();
    let (mut n_create, mut n_update, mut n_skip) = (0usize, 0usize, 0usize);
    for (id, name) in &remote {
        let existing = registry
            .tests
            .iter()
            .position(|t| t.id.as_deref() == Some(id.as_str()));
        let action = match existing {
            Some(_) if update || all => PullAction::Update,
            Some(_) => PullAction::Skip,
            None if update => PullAction::Skip,
            None => PullAction::Create,
        };
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
            println!("\n💡 Tip: Use --update to update existing tests or --all to pull everything");
        } else {
            println!("\n💡 Tip: Use --all to also update existing tests");
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
            println!("[DRY RUN] Would {verb} test: {name} (ID: {id})");
            continue;
        }

        let verb = if *action == PullAction::Update {
            "↻ Updating"
        } else {
            "+ Pulling"
        };
        println!("{verb} config for '{name}'...");

        let config = match api::get_test(ctx, id) {
            Ok(v) => v,
            Err(e) => {
                println!("  ✗ Error pulling test '{name}': {e}");
                continue;
            }
        };
        let test_type = config
            .get("type")
            .and_then(Value::as_str)
            .map(String::from);

        match existing_idx {
            Some(i) => {
                let cfg_path = registry.tests[*i].config.clone();
                project::write_json(Path::new(&cfg_path), &config)?;
                println!("  ✓ Updated '{name}' (config: {cfg_path})");
            }
            None => {
                let cfg_path = project::generate_unique_filename(&output_dir, name, ".json")
                    .display()
                    .to_string();
                project::write_json(Path::new(&cfg_path), &config)?;
                registry.tests.push(project::TestDefinition {
                    config: cfg_path.clone(),
                    test_type,
                    id: Some(id.clone()),
                });
                println!("  ✓ Added '{name}' (config: {cfg_path})");
            }
        }
        processed += 1;
    }

    if !dry_run && processed > 0 {
        project::save_tests(&registry)?;
        println!("\nUpdated {}", project::TESTS_FILE);
    }

    if dry_run {
        println!("\n[DRY RUN] Would process {} test(s)", n_create + n_update);
    } else {
        println!("\n✓ Summary: {n_create} created, {n_update} updated, {n_skip} skipped");
        if processed > 0 {
            println!(
                "You can now edit the config files in '{output_dir}/' and attach them to your agents"
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_template_builds_with_the_given_name() {
        for (name, _) in TEST_TEMPLATE_OPTIONS {
            let t = test_template_by_name("My Test", name).expect("template should build");
            assert_eq!(t["name"], json!("My Test"));
            assert!(t.get("success_condition").is_some());
        }
    }

    #[test]
    fn unknown_template_errors() {
        assert!(test_template_by_name("My Test", "nope").is_err());
    }

    #[test]
    fn tool_template_carries_tool_call_parameters() {
        let t = test_template_by_name("T", "tool").unwrap();
        assert_eq!(t["type"], json!("tool"));
        assert!(t["tool_call_parameters"]["referenced_tool"]["id"].is_string());
    }

    #[test]
    fn conversation_flow_is_multi_turn() {
        let t = test_template_by_name("T", "conversation-flow").unwrap();
        assert_eq!(t["chat_history"].as_array().unwrap().len(), 3);
    }

    #[test]
    fn recognizes_test_configs_by_shape() {
        assert!(looks_like_test_config(&json!({ "chat_history": [] })));
        assert!(looks_like_test_config(&json!({ "success_condition": "x" })));
        // Not tests: an agent config, or a tool config.
        assert!(!looks_like_test_config(
            &json!({ "name": "A", "conversation_config": {} })
        ));
        assert!(!looks_like_test_config(
            &json!({ "name": "T", "type": "webhook", "api_schema": {} })
        ));
    }
}
