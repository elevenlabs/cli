# ElevenLabs CLI — Agents as Code

![hero](./assets/Cover.png)

Command-line interface for the [ElevenLabs platform](https://elevenlabs.io/docs/agents-platform/overview).

The CLI does two things:

- **Full API access** — every ElevenLabs API endpoint is available as a subcommand (`elevenlabs <resource> <method>`).
- **Agents as Code** — manage Conversational AI agents from local configuration files, with templates, branches, and push/pull sync.

## Table of contents

- [Installation](#installation)
- [Authentication](#authentication)
- [Quick start](#quick-start)
- [Agents as Code](#agents-as-code)
- [Data residency](#data-residency)
- [UI components](#ui-components)
- [Usage](#usage)
- [Documentation](#documentation)
- [Advanced](#advanced)
  - [Common flags](#common-flags)
  - [Environment variables](#environment-variables)
  - [Output formats](#output-formats)
  - [Shell completion](#shell-completion)
- [Development](#development)

## Installation

> **Pre-release.** This version hasn't been published yet, so there are no
> prebuilt binaries to download — build from source as below. The installers
> documented at the bottom of this section start working with the first release.

### Build from source (alpha testers)

Install the [Rust toolchain](https://rustup.rs/), then:

```bash
git clone https://github.com/elevenlabs/cli.git
cd cli
cargo build --release
```

The binary lands at `./target/release/elevenlabs`:

```bash
./target/release/elevenlabs --help
```

To run it as `elevenlabs` from anywhere, either put it on your `PATH`:

```bash
# macOS / Linux — adjust to a directory already on your PATH
ln -sf "$PWD/target/release/elevenlabs" /usr/local/bin/elevenlabs
```

…or install it with cargo, which copies the binary into `~/.cargo/bin`:

```bash
cargo install --path .
```

Rebuild after pulling (`git pull && cargo build --release`) to pick up changes.

### Prebuilt binaries (once released)

These will work from the first tagged release onward:

```bash
# macOS / Linux
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/elevenlabs/cli/releases/latest/download/elevenlabs-installer.sh | sh
```

```powershell
# Windows
powershell -ExecutionPolicy ByPass -c "irm https://github.com/elevenlabs/cli/releases/latest/download/elevenlabs-installer.ps1 | iex"
```

## Authentication

This API requires authentication. Run `elevenlabs --help` for details.

## Quick start

List available commands:

```bash
elevenlabs --help
```

Call an API endpoint:

```bash
elevenlabs <resource> <method>
```

Run `elevenlabs <resource> --help` to see available methods for a resource.

## Agents as Code

Manage Conversational AI agents from local configuration files. `elevenlabs agents init` scaffolds a project; agent configs live as JSON on disk and sync to ElevenLabs. Pulled configs are stored as raw wire JSON and pushed back verbatim, so they round-trip losslessly.

### Project structure

```
your_project/
├── agents.json         # Agent registry: ids + branch mappings → config paths
├── tools.json          # Tool registry
├── tests.json          # Test registry
├── agent_configs/      # Agent configuration files
├── tool_configs/       # Tool configuration files
└── test_configs/       # Test configuration files
```

### Commands

```bash
# Scaffold a new project (pass a path, or --override to reset an existing one)
elevenlabs agents init [path] [--override]

# Create an agent from a template (or an existing file), upload it, and register it
elevenlabs agents add <name> [--template <template>] [--output-path <path>]
elevenlabs agents add [name] --from-file <path>

# Show the status of locally-configured agents
elevenlabs agents status

# Sync configs with ElevenLabs (push force-overrides main + registered branches)
elevenlabs agents push [--agent <agent_id>] [--branch <name|id>] [--version-description <text>] [--dry-run]
elevenlabs agents pull [--agent <agent_id>] [--branch <name|id>] [--all-branches] [--update] [--all] [--dry-run]

# List available agent templates, or print one's full configuration
elevenlabs agents templates list
elevenlabs agents templates show <template>

# Print an embeddable HTML widget snippet for an agent
elevenlabs agents widget embed <agent_id>

# Run the tests attached to an agent (polls to completion; exits non-zero on failure)
elevenlabs agents test <agent_id>
```

### Two surfaces in one namespace

`elevenlabs agents` holds two kinds of command, and it helps to know which you're using:

| | Workflow commands | API commands |
|---|---|---|
| **What** | `init`, `add`, `status`, `push`, `pull`, `test`, `templates`, `widget embed` | `create`, `get`, `list`, `update`, `delete`, `duplicate`, `run_tests`, and subgroups like `branches`, `tools`, `tests`, `conversations` |
| **Operates on** | Your local project files, syncing them with ElevenLabs | The API directly — one command, one request |
| **Arguments** | Positional, e.g. `agents test <agent_id>` | Flags, e.g. `agents get --agent-id <id>` |
| **Output** | Progress text | The API response (`--format json\|table\|yaml\|csv`) |

The API commands own the primitive names, so the workflow only adds verbs the API doesn't have. Two consequences worth knowing:

- **Listing** — `agents list` is the API's list of agents in your workspace. For what's configured *locally*, use `agents status`.
- **Deleting** — `agents delete --agent-id <id>` deletes remotely (API command). It does **not** remove the local config file or its `agents.json` entry; delete those yourself.

> **Migrating from v0 (`@elevenlabs/cli`):** v0's `agents list` showed local config and `agents delete <id>` took a positional and cleaned up locally. In v1 both names belong to the API surface — use `agents status` for the local view, and `--agent-id` for delete. v0's `agents widget <id>` is now `agents widget embed <id>`, since `agents widget` is an API subgroup.

### Tools

Manage the webhook and client tools your agents reference. Tools are tracked in `tools.json` with configs under `tool_configs/`.

```bash
# Create a webhook or client tool, upload it, and register it in tools.json
elevenlabs tools add <name> [--type webhook|client] [--config-path <path>]

# Sync tool configs with ElevenLabs
elevenlabs tools push [--tool <tool_id>] [--dry-run]
elevenlabs tools pull [--tool <tool_id>] [--output-dir tool_configs] [--update] [--all] [--dry-run]

# Delete a tool locally and in ElevenLabs
elevenlabs tools delete <tool_id>
elevenlabs tools delete --all
```

### Tests

Manage agent tests, tracked in `tests.json` with configs under `test_configs/`. Attach them to an agent's `platform_settings.testing.attached_tests` and run them with `elevenlabs agents test <agent_id>`.

```bash
# Create a test from a template, upload it, and register it in tests.json
elevenlabs tests add <name> [--template basic-llm|tool|conversation-flow|customer-service]
elevenlabs tests templates list

# Sync test configs with ElevenLabs
elevenlabs tests push [--test <test_id>] [--config-dir test_configs] [--dry-run]
elevenlabs tests pull [--test <test_id>] [--output-dir test_configs] [--update] [--all] [--dry-run]

# Delete a test locally and in ElevenLabs
elevenlabs tests delete <test_id>
elevenlabs tests delete --all
```

`tests push` also **auto-discovers** untracked configs: it scans `--config-dir` recursively for `.json` files that look like tests (a `chat_history` array or a `success_condition` string) and registers them in `tests.json` before pushing, so you can drop a config in and push without editing the index by hand.

### Templates

Pre-built starting configurations for `agents add`, listed by `elevenlabs agents templates list` (inspect one with `agents templates show <template>`):

| Template | Description |
|----------|-------------|
| `default` | Complete configuration with all available fields and sensible defaults |
| `minimal` | Minimal configuration with only essential fields |
| `voice-only` | Optimized for voice-only conversations |
| `text-only` | Optimized for text-only conversations |
| `customer-service` | Pre-configured for customer service scenarios |
| `assistant` | General purpose AI assistant configuration |

## Data residency

Select the region your requests are routed to. The setting is stored in `~/.elevenlabs/config.json` and applies to **every** command:

```bash
elevenlabs residency                 # show the current region and its base URL
elevenlabs residency eu-residency    # switch region
```

| Region | Base URL |
|--------|----------|
| `global` (default) | `https://api.elevenlabs.io` |
| `us` | `https://api.us.elevenlabs.io` |
| `eu-residency` | `https://api.eu.residency.elevenlabs.io` |
| `in-residency` | `https://api.in.residency.elevenlabs.io` |
| `sg-residency` | `https://api.sg.residency.elevenlabs.io` |

`--base-url` and `ELEVENLABS_BASE_URL` take precedence when you need a one-off override. The region also sets the `server-location` attribute emitted by `agents widget`.

## UI components

Install [ElevenLabs UI](https://ui.elevenlabs.io) components into your project (delegates to `shadcn`, so Node.js/npm is required):

```bash
elevenlabs components add                    # all components
elevenlabs components add conversation-bar
```

## Usage

Every API resource appears as a subcommand (e.g. `elevenlabs <resource> <method>`). Run `elevenlabs <resource> --help` to see available methods.

Provide request parameters as flags or as JSON:

```bash
elevenlabs <resource> <method> --json '{"key": "value"}'
```

## Documentation

See [reference.md](./reference.md) for the full command reference.

## Advanced

### Common flags

These flags are available on every operation:

| Flag | Description |
|------|-------------|
| `--dry-run` | Validate the request locally and print the HTTP request without sending it |
| `--json <JSON\|->` | Supply a request body as JSON (or `-` to read stdin) |
| `--params <JSON>` | Merge extra parameters as JSON (overrides individual flags) |
| `--format <json\|table\|yaml\|csv>` | Output format (default `json`) |
| `--output <PATH>` | Write binary responses to a file |
| `--base-url <URL>` | Override the API base URL |
| `--page-all` | Auto-paginate and stream results as NDJSON |
| `--page-limit <N>` | Max pages to fetch when auto-paginating (default `10`) |
| `-q, --quiet` | Suppress stdout output on success (errors still go to stderr) |

### Environment variables

| Variable | Description |
|----------|-------------|
| `ELEVENLABS_BASE_URL` | Override the API base URL |
| `ELEVENLABS_CA_BUNDLE` | Path to PEM file with extra trust roots (or `SSL_CERT_FILE`) |
| `ELEVENLABS_INSECURE=1` | Skip TLS verification (debugging only) |
| `ELEVENLABS_PROXY` | HTTP(S) proxy URL |
| `ELEVENLABS_TIMEOUT_SECS` | Total request timeout in seconds |

Standard environment variables (`HTTPS_PROXY` / `HTTP_PROXY` / `NO_PROXY` / `SSL_CERT_FILE`) are also honored.

### Output formats

Use the global `--format` flag to control output. Supported values: `json` (default), `table`, `yaml`, `csv`.

```bash
# Pipe JSON output through jq
elevenlabs <resource> <method> --format json | jq

# Machine-readable catalog of every operation
elevenlabs --help --format json | jq 'length'
```

### Shell completion

Generate shell completion scripts:

```bash
elevenlabs completion <bash|zsh|fish|powershell>
```


## Development

The CLI is generated by [Fern](https://buildwithfern.com) from the ElevenLabs OpenAPI spec, with the agents-as-code workflow layered on top as hand-written commands in `cli/elevenlabs/workflow/`. Those files, along with this README and `.github/workflows/ci.yml`, are listed in `.fernignore` so regeneration can't overwrite them.

### Tests

```bash
cargo test                                              # framework + workflow + wire tests
cargo test --manifest-path elevenlabs-sdk/Cargo.toml    # generated SDK crate
cargo test --manifest-path elevenlabs-types/Cargo.toml  # generated types crate
```

The generated crates are path dependencies rather than workspace members, so a plain `cargo test` skips them — hence the separate invocations (CI runs all three).

| Suite | What it covers |
|-------|----------------|
| `cargo test --test wire_test` | Generated wire tests: each stands up an in-process mock server, drives one endpoint through the CLI, and asserts the request and rendered response. No network. |
| `cargo test --bin elevenlabs` | Unit tests for the hand-written workflow (config round-tripping, templates, pull planning, residency, …) |
| `cargo test --test e2e_smoke` | Live end-to-end smoke test — **opt-in**, see below |

### Live end-to-end smoke test

`tests/e2e_smoke.rs` runs the full workflow against a real account: `init` → `add` → `push` → `pull` → `delete`, asserting that a config survives a push/pull round-trip byte-for-byte.

> ⚠️ **It creates and deletes agents. Use a dedicated, empty test account — never a production one.**

It's opt-in and skips unless `ELEVENLABS_E2E_API_KEY` is set. That variable is deliberately *not* `ELEVENLABS_API_KEY`, so a shell with your normal credentials exported can't accidentally mutate a live workspace:

```bash
ELEVENLABS_E2E_API_KEY=xi-... cargo test --test e2e_smoke -- --nocapture
```

It cleans up the agent it creates, including on failure; if deletion fails it prints the id to remove by hand.
