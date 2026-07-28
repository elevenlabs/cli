//! The `agents` command group: init/add/status/push/pull/widget/test.
//! Ports v0's `src/agents/`.
//!
//! These are the config-oriented verbs the raw API doesn't provide. The
//! primitives (`create`/`get`/`list`/`update`/`delete`/`run_tests`, and the
//! `branches` subgroup) come from the generated API commands in the same
//! `agents` group, so the workflow deliberately does not redefine them.
//! The `agents templates` subgroup is registered from [`super::templates`].

use std::path::{Path, PathBuf};
use std::time::Duration;

use fern_cli_sdk::app::CliApp;
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::AppContext;
use serde_json::{json, Value};

use super::util::{downcast_ctx, dry_run_flag, opt_string, plan_pull_action, PullAction};
use super::{api, project, settings, templates, verify};

/// Register the `agents` command group.
pub fn register(app: CliApp) -> CliApp {
    app.command_under_typed_with(
        &["agents"],
        clap::Command::new("init").about("Initialize a new agent management project"),
        handle_init,
    )
    .command_under_typed_with(
        &["agents"],
        clap::Command::new("add")
            .about("Add a new agent: create config, upload to ElevenLabs, and save its ID"),
        handle_add,
    )
    .command_under_typed_with(
        &["agents"],
        clap::Command::new("status").about("Show the status of configured agents"),
        handle_status,
    )
    // push/pull use the non-typed API so they can read the framework's
    // global `--dry-run` flag (which would collide with a typed field).
    .command_under(
        &["agents"],
        push_command(),
        Box::new(|matches, ctx| handle_push(matches, downcast_ctx(ctx)?)),
    )
    .command_under(
        &["agents"],
        pull_command(),
        Box::new(|matches, ctx| handle_pull(matches, downcast_ctx(ctx)?)),
    )
    // Nested under the generated `agents widget` subgroup rather than
    // replacing it: `widget get` (config) and `widget avatar` are API
    // commands, and a custom leaf named `widget` would shadow the whole group.
    .command_under_typed_with(
        &["agents", "widget"],
        clap::Command::new("embed")
            .about("Print an embeddable HTML widget snippet for an agent"),
        handle_widget,
    )
    .command_under_typed_with(
        &["agents"],
        clap::Command::new("test").about("Run the tests attached to an agent"),
        handle_test,
    )
}

// ── Shared helpers ──────────────────────────────────────────────────

/// Load `agents.json`, erroring with v0's "run init first" hint when it
/// is missing.
fn require_agents() -> Result<project::AgentsConfig, CliError> {
    if !Path::new(project::AGENTS_FILE).exists() {
        return Err(CliError::Validation(
            "agents.json not found. Run 'elevenlabs agents init' first.".to_string(),
        ));
    }
    project::load_agents()
}

/// Read an agent's display name from its config file. Ports v0's
/// `getAgentName` (Unknown when unreadable, Unnamed when nameless).
fn agent_display_name(config_path: &str) -> String {
    match project::read_value(Path::new(config_path)) {
        Ok(value) => value
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or("Unnamed Agent")
            .to_string(),
        Err(_) => "Unknown Agent".to_string(),
    }
}

// ── init ────────────────────────────────────────────────────────────

#[derive(clap::Args)]
struct InitArgs {
    /// Path to initialize the project in.
    #[arg(default_value = ".")]
    path: String,
    /// Override existing files and recreate config dirs from scratch.
    #[arg(long)]
    r#override: bool,
}

fn handle_init(args: InitArgs, _ctx: &AppContext) -> Result<(), CliError> {
    let root = PathBuf::from(&args.path);
    let abs = std::env::current_dir()
        .map(|cwd| cwd.join(&root))
        .unwrap_or_else(|_| root.clone());
    println!("Initializing project in {}", abs.display());
    if args.r#override {
        println!("⚠ Override mode: existing files will be overwritten");
    }

    std::fs::create_dir_all(&root).map_err(io_err("create project directory", &root))?;

    init_index_file(
        &root.join(project::AGENTS_FILE),
        project::AGENTS_FILE,
        args.r#override,
        &project::AgentsConfig::default(),
    )?;
    init_index_file(
        &root.join(project::TOOLS_FILE),
        project::TOOLS_FILE,
        args.r#override,
        &project::ToolsConfig::default(),
    )?;
    init_index_file(
        &root.join(project::TESTS_FILE),
        project::TESTS_FILE,
        args.r#override,
        &project::TestsConfig::default(),
    )?;

    for dir in [
        project::AGENT_CONFIGS_DIR,
        project::TOOL_CONFIGS_DIR,
        project::TEST_CONFIGS_DIR,
    ] {
        let dir_path = root.join(dir);
        let existed = dir_path.exists();
        if args.r#override && existed {
            std::fs::remove_dir_all(&dir_path).map_err(io_err("remove directory", &dir_path))?;
        }
        std::fs::create_dir_all(&dir_path).map_err(io_err("create directory", &dir_path))?;
        if !args.r#override && existed {
            println!("Created directory: {dir} (already existed)");
        } else {
            println!("Created directory: {dir}");
        }
    }

    let env_path = root.join(".env.example");
    if !args.r#override && env_path.exists() {
        println!(".env.example already exists (skipped)");
    } else {
        std::fs::write(
            &env_path,
            "# ElevenLabs API Key\nELEVENLABS_API_KEY=your_api_key_here\n",
        )
        .map_err(io_err("write .env.example", &env_path))?;
        println!("Created .env.example");
    }

    print_next_steps();
    Ok(())
}

fn init_index_file<T: serde::Serialize>(
    path: &Path,
    name: &str,
    overwrite: bool,
    default: &T,
) -> Result<(), CliError> {
    if !overwrite && path.exists() {
        println!("{name} already exists (skipped)");
    } else {
        project::write_json(path, default)?;
        println!("Created {name}");
    }
    Ok(())
}

fn print_next_steps() {
    println!("\nProject initialized successfully!");
    println!("Next steps:");
    println!("1. Set your ElevenLabs API key: elevenlabs auth login");
    println!("2. Create an agent: elevenlabs agents add \"My Agent\" --template default");
    println!("3. Create tools: elevenlabs tools add \"My Webhook\" --type webhook");
    println!("4. Create tests: elevenlabs tests add \"My Test\" --template basic-llm");
    println!(
        "5. Push to ElevenLabs: elevenlabs agents push && elevenlabs tools push && elevenlabs tests push"
    );
    println!("6. Run tests: elevenlabs agents test \"My Agent\"");
    println!("\nBranch workflow (CI/CD):");
    println!("  Pull all branches: elevenlabs agents pull --all --all-branches");
    println!("  Push all (main + branches): elevenlabs agents push");
}

/// Build a closure that maps an [`std::io::Error`] into a [`CliError`]
/// with a consistent, contextual message.
fn io_err(action: &str, path: &Path) -> impl FnOnce(std::io::Error) -> CliError {
    let action = action.to_string();
    let path = path.display().to_string();
    move |e| CliError::Other(anyhow::anyhow!("Could not {action} {path}: {e}"))
}

// ── add ─────────────────────────────────────────────────────────────

#[derive(clap::Args)]
struct AddArgs {
    /// Name of the agent to create.
    name: Option<String>,
    /// Custom output path for the config file.
    #[arg(long = "output-path")]
    output_path: Option<String>,
    /// Template to use (default, minimal, voice-only, text-only,
    /// customer-service, assistant).
    #[arg(long)]
    template: Option<String>,
    /// Create the agent from an existing config file.
    #[arg(long = "from-file")]
    from_file: Option<String>,
}

fn handle_add(args: AddArgs, ctx: &AppContext) -> Result<(), CliError> {
    if args.from_file.is_some() && args.template.is_some() {
        return Err(CliError::Validation(
            "Cannot use both --from-file and --template options together".to_string(),
        ));
    }
    if args.name.is_none() && args.from_file.is_none() {
        return Err(CliError::Validation(
            "Agent name is required in non-interactive mode".to_string(),
        ));
    }

    let mut registry = require_agents()?;

    // Build the agent config (from a file, or from a template).
    let (agent_config, agent_name): (Value, String) = if let Some(from_file) = &args.from_file {
        println!("Loading agent config from '{from_file}'...");
        let mut cfg = project::read_value(Path::new(from_file)).map_err(|e| {
            CliError::Validation(format!("Error reading config file '{from_file}': {e}"))
        })?;
        let name = args
            .name
            .clone()
            .or_else(|| cfg.get("name").and_then(Value::as_str).map(String::from))
            .ok_or_else(|| {
                CliError::Validation(
                    "Config file has no 'name' and no name was provided".to_string(),
                )
            })?;
        // An explicitly-provided name overrides the file's name.
        if args.name.is_some() {
            cfg["name"] = json!(name);
        }
        println!("Loaded config for agent: {name}");
        (cfg, name)
    } else {
        let name = args.name.clone().ok_or_else(|| {
            CliError::Validation("Agent name is required when using templates".to_string())
        })?;
        let template_type = args.template.clone().unwrap_or_else(|| "default".to_string());
        let cfg = templates::template_by_name(&name, &template_type)?;
        (cfg, name)
    };

    // Create in ElevenLabs first to obtain an ID.
    println!("Creating agent '{agent_name}' in ElevenLabs...");
    let agent_id = api::create_agent(ctx, &agent_config)?;
    println!("Created agent in ElevenLabs with ID: {agent_id}");

    // Resolve the on-disk config path (custom or generated from the name).
    let config_path = match &args.output_path {
        Some(path) => path.clone(),
        None => project::generate_unique_filename(project::AGENT_CONFIGS_DIR, &agent_name, ".json")
            .display()
            .to_string(),
    };
    project::write_json(Path::new(&config_path), &agent_config)?;
    match &args.from_file {
        Some(from_file) => println!("Created config file: {config_path} (from: {from_file})"),
        None => println!(
            "Created config file: {config_path} (template: {})",
            args.template.as_deref().unwrap_or("default")
        ),
    }

    registry.agents.push(project::AgentDefinition {
        config: config_path.clone(),
        id: Some(agent_id),
        branch_id: None,
        version_id: None,
        branches: None,
    });
    project::save_agents(&registry)?;
    println!("Added agent '{agent_name}' to agents.json");
    println!(
        "Edit {config_path} to customize your agent, then run 'elevenlabs agents push' to update"
    );
    Ok(())
}

// ── status ──────────────────────────────────────────────────────────

#[derive(clap::Args)]
struct NoArgs {}

fn handle_status(_args: NoArgs, _ctx: &AppContext) -> Result<(), CliError> {
    let config = require_agents()?;
    if config.agents.is_empty() {
        println!("No agents configured");
        return Ok(());
    }
    println!("Agent Status:");
    println!("{}", "=".repeat(50));
    for agent in &config.agents {
        println!("\n{}", agent_display_name(&agent.config));
        println!("   Config: {}", agent.config);
        println!(
            "   Agent ID: {}",
            agent.id.as_deref().unwrap_or("Not created yet")
        );
        if let Some(branch_id) = &agent.branch_id {
            println!("   Branch ID: {branch_id}");
        }
        if let Some(version_id) = &agent.version_id {
            println!("   Version ID: {version_id}");
        }
        let config_path = Path::new(&agent.config);
        if !config_path.exists() {
            println!("   Status: Config file not found");
        } else {
            match project::read_value(config_path) {
                Ok(_) if agent.id.is_some() => println!("   Status: Created (use push to update)"),
                Ok(_) => println!("   Status: Not pushed yet"),
                Err(e) => println!("   Status: Config error: {e}"),
            }
        }
    }
    Ok(())
}

// ── widget ──────────────────────────────────────────────────────────

#[derive(clap::Args)]
struct WidgetArgs {
    /// The agent ID to generate a widget for.
    agent_id: String,
}

fn handle_widget(args: WidgetArgs, _ctx: &AppContext) -> Result<(), CliError> {
    let config = require_agents()?;
    let agent = config
        .agents
        .iter()
        .find(|a| a.id.as_deref() == Some(args.agent_id.as_str()))
        .ok_or_else(|| {
            CliError::Validation(format!(
                "Agent with ID '{}' not found in configuration",
                args.agent_id
            ))
        })?;

    let residency = settings::read_residency();
    let mut html = format!("<elevenlabs-convai agent-id=\"{}\"", args.agent_id);
    // Isolated regions need the server-location attribute; global/us don't.
    if residency != "global" && residency != "us" {
        html.push_str(&format!(" server-location=\"{residency}\""));
    }
    html.push_str(
        "></elevenlabs-convai>\n<script src=\"https://unpkg.com/@elevenlabs/convai-widget-embed\" \
         async type=\"text/javascript\"></script>",
    );

    let name = agent_display_name(&agent.config);
    println!("HTML Widget for agent '{name}' (residency: {residency}):");
    println!("{}", "=".repeat(60));
    println!("{html}");
    println!("{}", "=".repeat(60));
    println!("Agent ID: {}", args.agent_id);
    Ok(())
}

// ── test ────────────────────────────────────────────────────────────

#[derive(clap::Args)]
struct TestArgs {
    /// The agent ID whose attached tests to run.
    agent: String,
}

fn handle_test(args: TestArgs, ctx: &AppContext) -> Result<(), CliError> {
    let config = require_agents()?;
    let agent = config
        .agents
        .iter()
        .find(|a| a.id.as_deref() == Some(args.agent.as_str()))
        .ok_or_else(|| {
            CliError::Validation(format!(
                "Agent with ID '{}' not found in configuration",
                args.agent
            ))
        })?;

    let config_path = Path::new(&agent.config);
    if !config_path.exists() {
        let name = agent_display_name(&agent.config);
        return Err(CliError::Validation(format!(
            "Config file not found for agent '{name}': {}",
            agent.config
        )));
    }
    let agent_config = project::read_value(config_path)?;
    let agent_name = agent_config
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or("Unnamed Agent")
        .to_string();

    let test_ids: Vec<String> = agent_config
        .get("platform_settings")
        .and_then(|p| p.get("testing"))
        .and_then(|t| t.get("attached_tests"))
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(|t| t.get("test_id").and_then(Value::as_str).map(String::from))
                .collect()
        })
        .unwrap_or_default();

    if test_ids.is_empty() {
        return Err(CliError::Validation(format!(
            "No tests attached to agent '{agent_name}'. Add tests to the agent's testing configuration."
        )));
    }

    println!("Running {} test(s) for agent '{agent_name}'...", test_ids.len());
    println!();

    let invocation = api::run_tests_on_agent(ctx, &args.agent, &test_ids, None)?;
    let invocation_id = invocation
        .get("id")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            CliError::Other(anyhow::anyhow!(
                "Test invocation response did not contain an id: {invocation}"
            ))
        })?
        .to_string();
    println!("Test invocation started (ID: {invocation_id})");
    println!("Waiting for tests to complete...");
    println!();

    // Poll every 5s, up to 5 minutes (60 attempts).
    const MAX_ATTEMPTS: u32 = 60;
    for _ in 0..MAX_ATTEMPTS {
        std::thread::sleep(Duration::from_secs(5));
        let status = api::get_test_invocation(ctx, &invocation_id)?;
        let test_runs = status
            .get("test_runs")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let all_complete = !test_runs.is_empty()
            && test_runs.iter().all(|run| {
                matches!(
                    run.get("status").and_then(Value::as_str),
                    Some("passed") | Some("failed")
                )
            });
        if !all_complete {
            continue;
        }

        println!("Test Results:");
        println!("{}", "=".repeat(50));
        let (mut passed, mut failed) = (0u32, 0u32);
        for run in &test_runs {
            let status = run.get("status").and_then(Value::as_str).unwrap_or("");
            let mark = if status == "passed" { "✓" } else { "✗" };
            let name = run
                .get("test_name")
                .and_then(Value::as_str)
                .or_else(|| run.get("test_id").and_then(Value::as_str))
                .unwrap_or("Unknown");
            println!("{mark} {name}: {status}");
            if status == "passed" {
                passed += 1;
            } else {
                failed += 1;
            }
        }
        println!("{}", "=".repeat(50));
        println!(
            "Total: {} | Passed: {passed} | Failed: {failed}",
            test_runs.len()
        );
        if failed > 0 {
            std::process::exit(1);
        }
        return Ok(());
    }

    eprintln!("Tests did not complete within the timeout period.");
    std::process::exit(1);
}

// ── push ────────────────────────────────────────────────────────────

fn push_command() -> clap::Command {
    clap::Command::new("push")
        .about("Push local agent configs to ElevenLabs")
        .arg(
            clap::Arg::new("agent")
                .long("agent")
                .help("Push only the agent with this ID"),
        )
        .arg(
            clap::Arg::new("branch")
                .long("branch")
                .help("Push to a specific branch (name or agtbrch_ id)"),
        )
        .arg(
            clap::Arg::new("version-description")
                .long("version-description")
                .help("Version description recorded with the update"),
        )
}

fn handle_push(matches: &clap::ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let agent = opt_string(matches, "agent");
    let branch = opt_string(matches, "branch");
    let version_description = opt_string(matches, "version-description");
    let dry_run = dry_run_flag(matches);

    let mut registry = require_agents()?;

    let indices: Vec<usize> = registry
        .agents
        .iter()
        .enumerate()
        .filter(|(_, a)| {
            agent
                .as_ref()
                .map_or(true, |id| a.id.as_deref() == Some(id.as_str()))
        })
        .map(|(i, _)| i)
        .collect();
    if let Some(agent_id) = &agent {
        if indices.is_empty() {
            return Err(CliError::Validation(format!(
                "Agent with ID {agent_id} not found in agents.json"
            )));
        }
    }

    println!("Pushing {} agent(s) to ElevenLabs...", indices.len());
    let mut changes_made = false;

    for idx in indices {
        let config_path = registry.agents[idx].config.clone();
        let current_id = registry.agents[idx].id.clone();
        let branches = registry.agents[idx].branches.clone();

        if config_path.is_empty() {
            println!("Warning: No config path found for agent");
            continue;
        }
        if !Path::new(&config_path).exists() {
            println!("Warning: Config file not found: {config_path}");
            continue;
        }
        let agent_config = match project::read_value(Path::new(&config_path)) {
            Ok(v) => v,
            Err(e) => {
                println!("Error reading config for {config_path}: {e}");
                continue;
            }
        };
        let name = agent_config
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or("Unnamed Agent")
            .to_string();

        println!("{name}: Will push (force override)");

        if dry_run {
            println!("[DRY RUN] Would update agent: {name}");
            if let Some(branch) = &branch {
                println!("  [DRY RUN] Would push to branch '{branch}'");
            } else if let (Some(brs), Some(_)) = (&branches, &current_id) {
                for branch_name in brs.keys() {
                    println!("  [DRY RUN] Would push branch '{branch_name}'");
                }
            }
            continue;
        }

        // Resolve a target branch id when --branch is given.
        let branch_id: Option<String> = match (&branch, &current_id) {
            (Some(branch), Some(id)) => match api::resolve_branch_id(ctx, id, branch) {
                Ok(bid) => {
                    println!("Pushing to branch: {branch}");
                    Some(bid)
                }
                Err(e) => {
                    println!("Error processing {name}: {e}");
                    continue;
                }
            },
            _ => None,
        };

        match &current_id {
            None => match api::create_agent(ctx, &agent_config) {
                Ok(new_id) => {
                    println!("Created agent {name} (ID: {new_id})");
                    registry.agents[idx].id = Some(new_id.clone());
                    changes_made = true;
                    verify::verify_agent_push(ctx, &name, &new_id, &agent_config, None);
                }
                Err(e) => {
                    println!("Error processing {name}: {e}");
                    continue;
                }
            },
            Some(id) => match api::update_agent(
                ctx,
                id,
                &agent_config,
                version_description.as_deref(),
                branch_id.as_deref(),
            ) {
                Ok(result) => {
                    println!("Updated agent {name} (ID: {id})");
                    if let Some(v) = result.version_id {
                        registry.agents[idx].version_id = Some(v);
                    }
                    if let Some(b) = result.branch_id {
                        registry.agents[idx].branch_id = Some(b);
                    }
                    changes_made = true;
                    verify::verify_agent_push(ctx, &name, id, &agent_config, branch_id.as_deref());
                }
                Err(e) => {
                    println!("Error processing {name}: {e}");
                    continue;
                }
            },
        }

        // Auto-push every registered branch config, unless a specific
        // --branch was targeted.
        if branch.is_none() {
            if let (Some(brs), Some(id)) = (branches, current_id) {
                for (branch_name, branch_def) in &brs {
                    if !Path::new(&branch_def.config).exists() {
                        println!(
                            "  Warning: Branch config file not found: {}",
                            branch_def.config
                        );
                        continue;
                    }
                    let branch_config = match project::read_value(Path::new(&branch_def.config)) {
                        Ok(v) => v,
                        Err(e) => {
                            println!("  ✗ Error pushing branch '{branch_name}': {e}");
                            continue;
                        }
                    };
                    println!("  Pushing branch '{branch_name}'...");
                    match api::update_agent(
                        ctx,
                        &id,
                        &branch_config,
                        version_description.as_deref(),
                        Some(&branch_def.branch_id),
                    ) {
                        Ok(result) => {
                            if let Some(v) = result.version_id {
                                if let Some(entry) = registry.agents[idx]
                                    .branches
                                    .as_mut()
                                    .and_then(|m| m.get_mut(branch_name))
                                {
                                    entry.version_id = Some(v);
                                }
                            }
                            println!("  ✓ Pushed branch '{branch_name}'");
                            verify::verify_agent_push(
                                ctx,
                                &format!("{name} (branch '{branch_name}')"),
                                &id,
                                &branch_config,
                                Some(&branch_def.branch_id),
                            );
                        }
                        Err(e) => println!("  ✗ Error pushing branch '{branch_name}': {e}"),
                    }
                }
            }
        }
    }

    if changes_made {
        project::save_agents(&registry)?;
    }
    Ok(())
}

// ── pull ────────────────────────────────────────────────────────────

fn pull_command() -> clap::Command {
    clap::Command::new("pull")
        .about("Pull agent configs from ElevenLabs")
        .arg(
            clap::Arg::new("agent")
                .long("agent")
                .help("Pull only the agent with this ID"),
        )
        .arg(
            clap::Arg::new("branch")
                .long("branch")
                .help("Pull from a specific branch (requires --agent)"),
        )
        .arg(
            clap::Arg::new("all-branches")
                .long("all-branches")
                .action(clap::ArgAction::SetTrue)
                .help("Pull every (non-archived) branch for each agent"),
        )
        .arg(
            clap::Arg::new("output-dir")
                .long("output-dir")
                .default_value("agent_configs")
                .help("Directory to write config files into"),
        )
        .arg(
            clap::Arg::new("update")
                .long("update")
                .action(clap::ArgAction::SetTrue)
                .help("Update existing agents only; skip new ones"),
        )
        .arg(
            clap::Arg::new("all")
                .long("all")
                .action(clap::ArgAction::SetTrue)
                .help("Pull everything (new and existing)"),
        )
}

fn handle_pull(matches: &clap::ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let agent = opt_string(matches, "agent");
    let branch = opt_string(matches, "branch");
    let all_branches = matches.get_flag("all-branches");
    let output_dir = opt_string(matches, "output-dir").unwrap_or_else(|| "agent_configs".to_string());
    let dry_run = dry_run_flag(matches);
    let update = matches.get_flag("update");
    let all = matches.get_flag("all");

    if branch.is_some() && agent.is_none() {
        return Err(CliError::Validation(
            "--branch requires --agent to be specified, since branch names are per-agent."
                .to_string(),
        ));
    }

    println!("Pulling agents from ElevenLabs...");

    let branch_id: Option<String> = match (&branch, &agent) {
        (Some(branch), Some(agent)) => {
            println!("Pulling from branch: {branch}");
            Some(api::resolve_branch_id(ctx, agent, branch)?)
        }
        _ => None,
    };

    let mut registry = if Path::new(project::AGENTS_FILE).exists() {
        project::load_agents()?
    } else {
        println!(
            "{} not found. Creating initial agents configuration...",
            project::AGENTS_FILE
        );
        let registry = project::AgentsConfig::default();
        project::save_agents(&registry)?;
        registry
    };

    // Build the remote work list: (agent_id, name).
    let remote: Vec<(String, String)> = if let Some(agent) = &agent {
        println!("Pulling agent with ID: {agent}...");
        let details = api::get_agent(ctx, agent, branch_id.as_deref()).map_err(|e| {
            CliError::Validation(format!("Failed to fetch agent with ID '{agent}': {e}"))
        })?;
        let id = details
            .get("agent_id")
            .and_then(Value::as_str)
            .unwrap_or(agent)
            .to_string();
        let name = details
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        println!("Found agent: {name}");
        vec![(id, name)]
    } else {
        println!("Pulling all agents from ElevenLabs...");
        let list = api::list_agents(ctx, None)?;
        if list.is_empty() {
            println!("No agents found in your ElevenLabs workspace.");
            return Ok(());
        }
        println!("Found {} agent(s)", list.len());
        list.iter()
            .filter_map(|a| {
                let id = a
                    .get("agent_id")
                    .or_else(|| a.get("agentId"))
                    .and_then(Value::as_str)?;
                let name = a.get("name").and_then(Value::as_str).unwrap_or("").to_string();
                Some((id.to_string(), name))
            })
            .collect()
    };

    // Plan: create / update / skip.
    let mut plan: Vec<(PullAction, String, String, Option<usize>)> = Vec::new();
    let (mut n_create, mut n_update, mut n_skip) = (0usize, 0usize, 0usize);
    for (id, name) in &remote {
        let existing = registry.agents.iter().position(|a| a.id.as_deref() == Some(id.as_str()));
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
            println!("\n💡 Tip: Use --update to update existing agents or --all to pull everything");
        } else {
            println!("\n💡 Tip: Use --all to also update existing agents");
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
            println!("[DRY RUN] Would {verb} agent: {name} (ID: {id})");
            continue;
        }

        let verb = if *action == PullAction::Update {
            "↻ Updating"
        } else {
            "+ Pulling"
        };
        println!("{verb} config for '{name}'...");

        let live = match api::get_agent(ctx, id, branch_id.as_deref()) {
            Ok(v) => v,
            Err(e) => {
                println!("  ✗ Error pulling agent '{name}': {e}");
                continue;
            }
        };
        let config = build_pulled_config(name, &live);
        let version_id = live.get("version_id").and_then(Value::as_str).map(String::from);
        let live_branch_id = live.get("branch_id").and_then(Value::as_str).map(String::from);

        let entry_idx = match existing_idx {
            Some(i) => {
                let cfg_path = registry.agents[*i].config.clone();
                project::write_json(Path::new(&cfg_path), &config)?;
                if let Some(v) = &version_id {
                    registry.agents[*i].version_id = Some(v.clone());
                }
                if let Some(b) = &live_branch_id {
                    registry.agents[*i].branch_id = Some(b.clone());
                }
                println!("  ✓ Updated '{name}' (config: {cfg_path})");
                *i
            }
            None => {
                let cfg_path =
                    project::generate_unique_filename(&output_dir, name, ".json")
                        .display()
                        .to_string();
                project::write_json(Path::new(&cfg_path), &config)?;
                registry.agents.push(project::AgentDefinition {
                    config: cfg_path.clone(),
                    id: Some(id.clone()),
                    branch_id: live_branch_id.clone(),
                    version_id: version_id.clone(),
                    branches: None,
                });
                println!("  ✓ Added '{name}' (config: {cfg_path})");
                registry.agents.len() - 1
            }
        };

        // --branch: persist the branch config alongside the main one.
        if let (Some(branch), Some(bid)) = (&branch, &branch_id) {
            let branch_path =
                project::generate_unique_filename(&output_dir, &format!("{name}.{branch}"), ".json")
                    .display()
                    .to_string();
            project::write_json(Path::new(&branch_path), &config)?;
            let branches = registry.agents[entry_idx]
                .branches
                .get_or_insert_with(Default::default);
            branches.insert(
                branch.clone(),
                project::BranchDefinition {
                    config: branch_path.clone(),
                    branch_id: bid.clone(),
                    version_id: version_id.clone(),
                },
            );
            println!("  ✓ Stored branch '{branch}' config ({branch_path})");
        }

        if all_branches {
            pull_all_branches(ctx, id, name, entry_idx, &mut registry, &output_dir)?;
        }

        processed += 1;
    }

    if !dry_run && processed > 0 {
        project::save_agents(&registry)?;
        println!("\nUpdated {}", project::AGENTS_FILE);
    }

    if dry_run {
        println!("\n[DRY RUN] Would process {} agent(s)", n_create + n_update);
    } else {
        println!("\n✓ Summary: {n_create} created, {n_update} updated, {n_skip} skipped");
        if processed > 0 {
            println!(
                "You can now edit the config files in '{}/' and run 'elevenlabs agents push' to update",
                output_dir
            );
        }
    }
    Ok(())
}

/// Pull every non-archived, non-main branch for an agent into config files
/// tracked under its `agents.json` entry. Ports v0's `pullAllBranches`.
fn pull_all_branches(
    ctx: &AppContext,
    agent_id: &str,
    agent_name: &str,
    entry_idx: usize,
    registry: &mut project::AgentsConfig,
    output_dir: &str,
) -> Result<(), CliError> {
    println!("  Fetching branches for '{agent_name}'...");
    let branches = api::list_branches(ctx, agent_id, false)?;
    if branches.is_empty() {
        println!("  No branches found for '{agent_name}'");
        return Ok(());
    }

    let parent_branch_id = registry.agents[entry_idx].branch_id.clone();
    for branch in &branches {
        if branch.get("is_archived").and_then(Value::as_bool).unwrap_or(false) {
            continue;
        }
        let bid = branch.get("id").and_then(Value::as_str).unwrap_or("");
        let bname = branch.get("name").and_then(Value::as_str).unwrap_or("");
        // Skip the main branch (matches the agent's branch_id, or is named
        // "main" when the agent has no branch_id).
        let is_main = Some(bid) == parent_branch_id.as_deref()
            || (parent_branch_id.is_none() && bname == "main");
        if is_main {
            continue;
        }

        let live = match api::get_agent(ctx, agent_id, Some(bid)) {
            Ok(v) => v,
            Err(e) => {
                println!("  ✗ Error pulling branch '{bname}': {e}");
                continue;
            }
        };
        let branch_config = build_pulled_config(agent_name, &live);
        let version_id = live.get("version_id").and_then(Value::as_str).map(String::from);

        let existing_path = registry.agents[entry_idx]
            .branches
            .as_ref()
            .and_then(|m| m.get(bname))
            .map(|b| b.config.clone());
        let branch_path = existing_path.unwrap_or_else(|| {
            project::generate_unique_filename(output_dir, &format!("{agent_name}.{bname}"), ".json")
                .display()
                .to_string()
        });
        project::write_json(Path::new(&branch_path), &branch_config)?;

        let map = registry.agents[entry_idx]
            .branches
            .get_or_insert_with(Default::default);
        map.insert(
            bname.to_string(),
            project::BranchDefinition {
                config: branch_path.clone(),
                branch_id: bid.to_string(),
                version_id,
            },
        );
        println!("  ✓ Branch '{bname}' ({branch_path})");
    }

    let count = registry.agents[entry_idx]
        .branches
        .as_ref()
        .map(|m| m.len())
        .unwrap_or(0);
    if count > 0 {
        println!("  {count} branch(es) stored");
    }
    Ok(())
}

/// Extract the on-disk config (name + conversation_config + platform_settings
/// + tags [+ workflow]) from a live agent response. Ports the config
/// assembly in v0's `pull-impl.ts`.
fn build_pulled_config(name: &str, live: &Value) -> Value {
    let mut obj = serde_json::Map::new();
    obj.insert("name".to_string(), json!(name));
    obj.insert(
        "conversation_config".to_string(),
        live.get("conversation_config").cloned().unwrap_or_else(|| json!({})),
    );
    obj.insert(
        "platform_settings".to_string(),
        live.get("platform_settings").cloned().unwrap_or_else(|| json!({})),
    );
    obj.insert(
        "tags".to_string(),
        live.get("tags").cloned().unwrap_or_else(|| json!([])),
    );
    if let Some(workflow) = live.get("workflow") {
        if !workflow.is_null() {
            obj.insert("workflow".to_string(), workflow.clone());
        }
    }
    Value::Object(obj)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pulled_config_keeps_only_the_on_disk_shape() {
        // Server-side metadata (agent_id, version_id, …) must not leak into the
        // config file — ids live in agents.json, so a pulled config stays
        // pushable as-is.
        let live = json!({
            "agent_id": "agent_1",
            "version_id": "ver_1",
            "conversation_config": { "agent": { "language": "en" } },
            "platform_settings": { "auth": { "enable_auth": false } },
            "tags": ["a"]
        });
        let config = build_pulled_config("My Agent", &live);
        assert_eq!(config["name"], json!("My Agent"));
        assert_eq!(config["conversation_config"]["agent"]["language"], json!("en"));
        assert_eq!(config["tags"], json!(["a"]));
        assert!(config.get("agent_id").is_none());
        assert!(config.get("version_id").is_none());
    }

    #[test]
    fn pulled_config_fills_defaults_for_missing_blocks() {
        let config = build_pulled_config("A", &json!({}));
        assert_eq!(config["conversation_config"], json!({}));
        assert_eq!(config["platform_settings"], json!({}));
        assert_eq!(config["tags"], json!([]));
    }

    #[test]
    fn workflow_is_included_only_when_present_and_non_null() {
        let with = build_pulled_config("A", &json!({ "workflow": { "nodes": [] } }));
        assert_eq!(with["workflow"], json!({ "nodes": [] }));

        // A null workflow would be rejected on push, so it's omitted entirely.
        let null = build_pulled_config("A", &json!({ "workflow": Value::Null }));
        assert!(null.get("workflow").is_none());

        let absent = build_pulled_config("A", &json!({}));
        assert!(absent.get("workflow").is_none());
    }
}
