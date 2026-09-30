---
name: verify-elevenlabs
description: "Drive the ElevenLabs CLI (the elevenlabs binary in this repo) the way a user does — build it, run an offline command in an isolated home and project directory, and capture the terminal transcript. Use when proving agents init, agent templates, agents status, say config, or residency. Other checkouts (the xi web app, the Fern docs site, the skills catalog) are out of scope for this skill."
---

# Verify the ElevenLabs CLI

The user-facing surface this skill drives is the `elevenlabs` command-line binary in this repository. A user installs it and runs commands in a terminal. This skill builds that binary and drives the commands that work with no API key and no network: scaffolding an agents-as-code project, listing templates, reading local status, and reading or writing local `say` and residency settings.

Other ElevenLabs surfaces are not driven here:

- The xi monorepo is the elevenlabs.io web app and its backend. Starting it needs the nix dev shell and service credentials. This skill does not launch it.
- The elevenlabs-dx repository is the public docs site. `pnpm run dev` serves it on port 3000. This skill does not launch it.
- The skills repository is a catalog of agent skills. It has no process to start.

`scripts/verify-agents-as-code.sh` is a different tool. It creates and deletes real agents in a live workspace. Do not run it from this skill.

The helper is `.cursor/skills/verify-elevenlabs/bin/verify-elevenlabs`. From the repo root:

```bash
.cursor/skills/verify-elevenlabs/bin/verify-elevenlabs launch
.cursor/skills/verify-elevenlabs/bin/verify-elevenlabs doctor
.cursor/skills/verify-elevenlabs/bin/verify-elevenlabs run --name <name> -- <elevenlabs-args>
.cursor/skills/verify-elevenlabs/bin/verify-elevenlabs cleanup
```

## Launch

Launch builds the binary once, then each drive is a short-lived command in its own tmux session. There is no server to keep up.

Preconditions on the machine:

- `rustup` can provide stable Rust. This repo's lockfile pulls crates that need Cargo edition 2024 (Rust 1.85 or newer). Rust 1.83 fails while parsing `time-core`. If `cargo +stable` is missing, install it with `rustup toolchain install stable --profile minimal`.
- `pkg-config` and OpenSSL headers are installed. `openssl-sys` fails the build without them. On Ubuntu: `sudo apt-get install -y pkg-config libssl-dev`.
- `tmux` is on `PATH`. When `/exec-daemon/tmux.portal.conf` exists, the helper passes `-f` with that file.

Command:

```bash
.cursor/skills/verify-elevenlabs/bin/verify-elevenlabs launch
```

What that runs, from the repo root:

```bash
cargo +stable build --bin elevenlabs --locked
```

The debug binary is `$CARGO_TARGET_DIR/debug/elevenlabs`. The helper picks `CARGO_TARGET_DIR` in this order:

1. `CARGO_TARGET_DIR` if it is already set and the directory is writable and executable.
2. `<repo>/target` when that filesystem has at least 4 GiB free and can execute a file.
3. `/mnt/verify-elevenlabs-target` when that directory exists, is executable, and has at least 4 GiB free.
4. `/dev/shm/verify-elevenlabs-target` on the same conditions.

`/dev/shm` is often mounted `noexec`. A build there fails with `Permission denied` while running a build script. When the repo filesystem is short on space, mount an executable tmpfs and rerun launch:

```bash
sudo mkdir -p /mnt/verify-elevenlabs-target
sudo mount -t tmpfs -o rw,nosuid,nodev,exec,size=12G tmpfs /mnt/verify-elevenlabs-target
sudo chown "$(id -u):$(id -g)" /mnt/verify-elevenlabs-target
```

Ready means stdout contains a line `ready: elevenlabs <version> run=<id>` and the binary's `--version` matches that version. Launch also prints `binary:`, `target_dir:`, `home:`, `work:`, and `evidence:`.

Launch creates, and never touches the real user home:

- scratch at `/tmp/verify-elevenlabs-runs/<id>/{home,work,state.env,sessions,OWNED}`
- evidence at `$VERIFY_EVIDENCE_DIR` if set, otherwise `/tmp/verify-elevenlabs-evidence/<id>`

Pass `--run-id <id>` to reuse an id. Omit it and launch generates `run-<utc>-<pid>` and records it in `/tmp/verify-elevenlabs-runs/latest`. Concurrent runs must each pass their own `--run-id` on every later command. `latest` only points at the most recent launch.

Teardown is `verify-elevenlabs cleanup` for that same run id. Cleanup does not delete the cargo target directory or the evidence directory.

## Doctor

Run doctor before every drive and whenever a command looks wrong. It is read-only: it does not build, does not create files, and does not call the API.

```bash
.cursor/skills/verify-elevenlabs/bin/verify-elevenlabs doctor
```

Doctor exits 0 only when all of these are true:

- `state.env` for this run exists and was written by launch (`OWNED` marker present).
- `binary` is executable and is exactly `$TARGET_DIR/debug/elevenlabs` from that state.
- `$BINARY --version`, with `HOME` set to the run's home and `ELEVENLABS_API_KEY` and `ELEVENLABS_BASE_URL` unset, prints the version string launch recorded.
- `home` and `work` exist, both live under `/tmp/verify-elevenlabs-runs/<id>/`, and `home` is not the real user home recorded at launch.
- The state says the API key is not forwarded (`API_KEY_FORWARDED=0`).
- `tmux` is on `PATH`.

There is no listen port and no long-running process. Doctor prints `port=none`. A binary that answers `--version` is the "process up" check. Stdout on success:

```text
ok
run_id=<id>
binary=<path>
version=elevenlabs <version>
target_dir=<path>
home=<path>
work=<path>
evidence=<path>
api_key=unset
port=none
```

If the parent shell has `ELEVENLABS_API_KEY` set, doctor warns on stderr and still passes. `run` does not forward that variable.

Do not drive a run whose doctor fails. Do not drive the user's real `~/.elevenlabs` or the repo's `.verify-project` directory.

## Drive

The harness is `verify-elevenlabs run`. It starts a new tmux session named `verify-elevenlabs-<run-id>-<name>`, with a 220-column pane, `HOME` set to the run's disposable home, `ELEVENLABS_API_KEY` and `ELEVENLABS_BASE_URL` unset, and the working directory set to the run's work directory. It then runs the built binary with the arguments after `--`.

```bash
.cursor/skills/verify-elevenlabs/bin/verify-elevenlabs run --name <name> -- <args>
```

`<name>` must match `[A-Za-z0-9._-]+`. It is the evidence file stem and part of the tmux session name. Doctor runs first; a failing doctor aborts the drive.

The CLI is a short command, not a TUI. Stable handles are the subcommand words and flags in `cli/elevenlabs/workflow/`, not cursor positions:

| User action | Arguments after `--` |
|---|---|
| Scaffold a project in the work directory | `agents init .` |
| List built-in agent templates | `agents templates list` |
| Show one template | `agents templates show <template>` |
| Show local agent status | `agents status` |
| Show say defaults | `say config` |
| Set a say default | `say config <voice\|model\|output-format\|player> <value>` |
| Show residency | `residency` |
| Set residency | `residency <region>` |

Recipes and expected output are in `features/`. Start from the baseline in `features/README.md`. One `run` is one command. A second view is a second `run` against the same work directory.

Generated API subcommands switch to JSON when stdout is not a terminal. These workflow commands print the same text either way, because they use `println!`. The harness still runs them in tmux so the transcript is the pane the user would see.

`agents init . --override` without `--yes` reads stdin. Inside this tmux session stdin stays open, so the prompt blocks until the 45 second timeout. The decline path is documented in `features/agents-init.md` and is not a `run` invocation.

Commands that upload or delete remote resources (`agents add`, `agents push`, `agents pull`, `tools add`, `tests add`, and the live script `scripts/verify-agents-as-code.sh`) are outside this skill. They need an API key and they mutate a real workspace.

## Evidence

Proof artifacts go to the `evidence=` directory doctor prints. Default: `/tmp/verify-elevenlabs-evidence/<run-id>/`. Set `VERIFY_EVIDENCE_DIR` before `launch` to put them somewhere else. Cleanup never deletes this directory.

Each `run --name <name>` writes:

- `<name>.transcript.txt` — the tmux pane, including the typed command and the program output
- `<name>.exit` — the process exit code
- `<name>.meta` — run id, argv, binary, version, home, work, session name

Standards:

- Drive the user command. Do not flip a Rust flag, call a test helper, or hit a mock server.
- Capture the command and the resulting state. A transcript of `agents init` is not enough; follow it with `agents status` and a listing of the files init wrote.
- Check side effects on disk. `agents init` must leave `agents.json`, `tools.json`, `tests.json`, `.gitignore`, `.env.example`, and the three config directories. `say config` and `residency` must change `$HOME/.elevenlabs/config.json` under the disposable home, which a later `say config` or `residency` command shows again.
- Do not trust a name. `--dry-run` on `agents push` is a live-account flag and is not a proof path here. `agents init --override` exits 0 even when the user declines; the proof is the stdout line `Initialization cancelled` plus the files that are still present.
- These offline commands do not open a browser and do not call the API. Confirm that by using a work directory outside the repo (so `dotenvy` does not walk up into a repo `.env`) and by leaving `ELEVENLABS_API_KEY` unset. The transcript must not contain an HTTP status or an auth error.
- Record the feature file name and the `--name` used next to the artifacts (the `.meta` file does this).

Copy any file that has to outlive the work directory into the evidence directory before cleanup. The work directory is deleted.

## Cleanup

```bash
.cursor/skills/verify-elevenlabs/bin/verify-elevenlabs cleanup
```

Cleanup kills only tmux sessions named `verify-elevenlabs-<run-id>` or `verify-elevenlabs-<run-id>-*`, then deletes `/tmp/verify-elevenlabs-runs/<id>/`. It does not kill by process name, so another `elevenlabs` or `cargo` process is left alone. It does not delete the cargo target directory. It does not delete the evidence directory. If that directory is missing, cleanup exits non-zero.

After a failed drive, run cleanup for that run id before starting another attempt, so a timed-out tmux session is not left holding the work directory.

A second run id is a second scratch tree. Cleaning one does not clean the other.

## Helpers

The only helper is the executable `verify-elevenlabs` script above. It is invoked as:

```bash
.cursor/skills/verify-elevenlabs/bin/verify-elevenlabs <launch|doctor|run|cleanup> [--run-id ID]
```

`run` also requires `--name NAME -- <elevenlabs-args>`. There is no other script to discover.
