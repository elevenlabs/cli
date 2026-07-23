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
- [Usage](#usage)
- [Documentation](#documentation)
- [Advanced](#advanced)
  - [Common flags](#common-flags)
  - [Environment variables](#environment-variables)
  - [Output formats](#output-formats)
  - [Shell completion](#shell-completion)

## Installation

### Shell (macOS / Linux)

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/elevenlabs/cli/releases/latest/download/elevenlabs-installer.sh | sh
```

### PowerShell (Windows)

```powershell
powershell -ExecutionPolicy ByPass -c "irm https://github.com/elevenlabs/cli/releases/latest/download/elevenlabs-installer.ps1 | iex"
```

### Build from source

If you prefer to build from source, install the [Rust toolchain](https://rustup.rs/) and run:

```bash
cargo build --release
./target/release/elevenlabs --help
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

# Inspect locally-configured agents
elevenlabs agents list
elevenlabs agents status

# List available agent templates
elevenlabs agents templates list

# Print an embeddable HTML widget snippet for an agent
elevenlabs agents widget <agent_id>

# List an agent's branches (e.g. staging vs production)
elevenlabs agents branches list --agent <agent_id> [--include-archived]

# Delete an agent locally and in ElevenLabs
elevenlabs agents delete <agent_id>
elevenlabs agents delete --all
```

> Additional agents-as-code commands (`add`, `push`, `pull`, `test`) and the `tools` / `tests` command groups are being ported from v0 and will land in follow-up changes.

### Templates

Pre-built starting configurations, listed by `elevenlabs agents templates list`:

| Template | Description |
|----------|-------------|
| `default` | Complete configuration with all available fields and sensible defaults |
| `minimal` | Minimal configuration with only essential fields |
| `voice-only` | Optimized for voice-only conversations |
| `text-only` | Optimized for text-only conversations |
| `customer-service` | Pre-configured for customer service scenarios |
| `assistant` | General purpose AI assistant configuration |

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

