//! The `agents` command group: init/add/list/status/push/pull/delete/
//! widget/test/branches. Ports v0's `src/agents/`.
//!
//! Implemented so far: `init`. The remaining subcommands (add/list/
//! status/push/pull/delete/widget/test/branches) land in a later step.
//! The `agents templates` subgroup is registered from [`super::templates`].

use std::path::{Path, PathBuf};

use fern_cli_sdk::app::CliApp;
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::AppContext;
use serde_json::Value;

use super::{api, project, settings};

/// Register the `agents` command group.
pub fn register(app: CliApp) -> CliApp {
    app.command_under_typed_with(
        &["agents"],
        clap::Command::new("init").about("Initialize a new agent management project"),
        handle_init,
    )
    .command_under_typed_with(
        &["agents"],
        clap::Command::new("list").about("List all configured agents"),
        handle_list,
    )
    .command_under_typed_with(
        &["agents"],
        clap::Command::new("status").about("Show the status of configured agents"),
        handle_status,
    )
    .command_under_typed_with(
        &["agents"],
        clap::Command::new("delete").about("Delete an agent locally and in ElevenLabs"),
        handle_delete,
    )
    .command_under_typed_with(
        &["agents"],
        clap::Command::new("widget").about("Print an embeddable HTML widget snippet for an agent"),
        handle_widget,
    )
    .command_under_typed_with(
        &["agents", "branches"],
        clap::Command::new("list").about("List branches for an agent"),
        handle_branches_list,
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

// ── list ────────────────────────────────────────────────────────────

#[derive(clap::Args)]
struct NoArgs {}

fn handle_list(_args: NoArgs, _ctx: &AppContext) -> Result<(), CliError> {
    let config = require_agents()?;
    if config.agents.is_empty() {
        println!("No agents configured");
        return Ok(());
    }
    println!("Configured Agents:");
    println!("{}", "=".repeat(50));
    for (i, agent) in config.agents.iter().enumerate() {
        println!("{}. {}", i + 1, agent_display_name(&agent.config));
        println!("   ID: {}", agent.id.as_deref().unwrap_or("No ID"));
        println!("   Config: {}", agent.config);
        println!();
    }
    Ok(())
}

// ── status ──────────────────────────────────────────────────────────

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

// ── delete ──────────────────────────────────────────────────────────

#[derive(clap::Args)]
struct DeleteArgs {
    /// The agent ID to delete (omit with --all).
    agent_id: Option<String>,
    /// Delete every configured agent.
    #[arg(long)]
    all: bool,
    /// Skip the confirmation prompt (for --all).
    #[arg(long)]
    yes: bool,
}

fn handle_delete(args: DeleteArgs, ctx: &AppContext) -> Result<(), CliError> {
    let mut config = require_agents()?;

    if args.all {
        if config.agents.is_empty() {
            println!("No agents found to delete");
            return Ok(());
        }
        println!("\nFound {} agent(s) to delete:", config.agents.len());
        for (i, agent) in config.agents.iter().enumerate() {
            println!(
                "  {}. {} ({})",
                i + 1,
                agent_display_name(&agent.config),
                agent.id.as_deref().unwrap_or("no id")
            );
        }
        if !args.yes {
            println!(
                "\nWARNING: This will delete ALL agents from both local configuration and ElevenLabs."
            );
            if !project::prompt_confirm("Are you sure you want to delete these agents?")? {
                println!("Deletion cancelled");
                return Ok(());
            }
        }
        println!("\nDeleting agents...\n");
        for agent in &config.agents {
            let name = agent_display_name(&agent.config);
            println!(
                "Deleting '{name}' ({})...",
                agent.id.as_deref().unwrap_or("no id")
            );
            match &agent.id {
                Some(id) => match api::delete_agent(ctx, id) {
                    Ok(()) => println!("  ✓ Deleted from ElevenLabs"),
                    Err(e) => eprintln!("  Warning: Failed to delete from ElevenLabs: {e}"),
                },
                None => println!("  Warning: No agent ID found, skipping ElevenLabs deletion"),
            }
            remove_config_file(&agent.config);
        }
        config.agents.clear();
        project::save_agents(&config)?;
        println!("\n✓ Deleted all agents");
        return Ok(());
    }

    let Some(agent_id) = args.agent_id else {
        return Err(CliError::Validation(
            "Provide an agent ID to delete, or pass --all.".to_string(),
        ));
    };

    let index = config
        .agents
        .iter()
        .position(|a| a.id.as_deref() == Some(agent_id.as_str()))
        .ok_or_else(|| {
            CliError::Validation(format!(
                "Agent with ID '{agent_id}' not found in local configuration"
            ))
        })?;

    let removed = config.agents.remove(index);
    let name = agent_display_name(&removed.config);
    println!("Deleting agent '{name}' (ID: {agent_id})...");
    println!("Deleting from ElevenLabs...");
    match api::delete_agent(ctx, &agent_id) {
        Ok(()) => println!("✓ Successfully deleted from ElevenLabs"),
        Err(e) => {
            eprintln!("Warning: Failed to delete from ElevenLabs: {e}");
            println!("Continuing with local deletion...");
        }
    }
    project::save_agents(&config)?;
    println!("✓ Removed '{name}' from agents.json");
    if remove_config_file(&removed.config) {
        println!("✓ Deleted config file: {}", removed.config);
    }
    println!("\n✓ Successfully deleted agent '{name}'");
    Ok(())
}

/// Remove a config file if present; returns whether it was deleted.
fn remove_config_file(config_path: &str) -> bool {
    let path = Path::new(config_path);
    if path.exists() {
        std::fs::remove_file(path).is_ok()
    } else {
        false
    }
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

// ── branches list ───────────────────────────────────────────────────

#[derive(clap::Args)]
struct BranchesListArgs {
    /// The agent whose branches to list.
    #[arg(long)]
    agent: String,
    /// Include archived branches.
    #[arg(long)]
    include_archived: bool,
}

fn handle_branches_list(args: BranchesListArgs, ctx: &AppContext) -> Result<(), CliError> {
    println!("Listing branches for agent: {}...", args.agent);
    let branches = api::list_branches(ctx, &args.agent, args.include_archived)?;
    if branches.is_empty() {
        println!("No branches found for this agent.");
        return Ok(());
    }

    println!(
        "{:<25}{:<40}{:<12}{:<10}LAST UPDATED",
        "NAME", "BRANCH ID", "STATUS", "TRAFFIC"
    );
    println!("{}", "─".repeat(110));
    for branch in &branches {
        let raw_name = branch.get("name").and_then(Value::as_str).unwrap_or("");
        let name = if raw_name.chars().count() > 23 {
            format!("{}...", raw_name.chars().take(20).collect::<String>())
        } else {
            raw_name.to_string()
        };
        let id = branch.get("id").and_then(Value::as_str).unwrap_or("");
        let status = if branch
            .get("is_archived")
            .and_then(Value::as_bool)
            .unwrap_or(false)
        {
            "archived"
        } else {
            "active"
        };
        let traffic = format!(
            "{}%",
            format_percent(
                branch
                    .get("current_live_percentage")
                    .and_then(Value::as_f64)
                    .unwrap_or(0.0)
            )
        );
        let last_updated = branch
            .get("last_committed_at")
            .and_then(Value::as_i64)
            .map(epoch_to_date)
            .unwrap_or_default();
        println!("{name:<25}{id:<40}{status:<12}{traffic:<10}{last_updated}");
    }
    println!("\n{} branch(es) found", branches.len());
    Ok(())
}

/// Format a percentage dropping a trailing `.0` (12.0 → "12", 12.5 → "12.5").
fn format_percent(n: f64) -> String {
    if n.fract() == 0.0 {
        format!("{}", n as i64)
    } else {
        format!("{n}")
    }
}

/// Convert a Unix timestamp (seconds) to `YYYY-MM-DD` (UTC), using the
/// standard civil-from-days algorithm (no external date dependency).
fn epoch_to_date(secs: i64) -> String {
    let days = secs.div_euclid(86_400);
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if month <= 2 { year + 1 } else { year };
    format!("{year:04}-{month:02}-{day:02}")
}
