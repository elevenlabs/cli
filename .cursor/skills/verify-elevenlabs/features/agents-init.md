# Initialize a project

Initialize a project scaffolds a local agents-as-code directory: index files, empty config directories, a `.gitignore` that ignores `.env`, and a `.env.example` placeholder.

## Sub-features

- `init-scaffold` creates the project files in the directory the user names.
- `init-status` shows an empty project after scaffolding.
- `init-decline-override` leaves existing files in place when override confirmation is declined.

## How to get to it (user POV)

- Run `elevenlabs agents init` in the directory to scaffold. The default path is `.`.
- Run `elevenlabs agents init <path>` to scaffold a different directory.
- Run `elevenlabs agents init . --override` to recreate config directories. Without `--yes`, the CLI asks `Continue? (y/N):` on stderr before deleting anything.

## Driving it with verify-elevenlabs

Preconditions:

- `verify-elevenlabs doctor` prints `ok` for this run.
- The work directory is empty.
- `ELEVENLABS_API_KEY` is unset.

- **Scaffold.** Initialize the work directory. Run `verify-elevenlabs run --name init-scaffold -- agents init .`. Exit code `0`. The transcript contains `Initializing project in` followed by the work directory path, `Created agents.json`, `Created tools.json`, `Created tests.json`, `Created directory: agent_configs`, `Created directory: tool_configs`, `Created directory: test_configs`, `Created .gitignore`, `Created .env.example`, and `Project initialized successfully!`.
- **Confirm the files.** In the work directory, `agents.json`, `tools.json`, and `tests.json` exist, `agent_configs`, `tool_configs`, and `test_configs` are directories, `.gitignore` contains the line `.env` and does not ignore `agent_configs/`, and `.env.example` contains `ELEVENLABS_API_KEY=your_api_key_here`. `agents.json` is JSON whose `agents` array is empty. Copy the work tree into the evidence directory before cleanup:

```bash
find "$WORK" -print | sort > "$EVIDENCE/agents-init.tree.txt"
cp "$WORK/agents.json" "$EVIDENCE/agents-init.agents.json"
```

`WORK` and `EVIDENCE` are the `work=` and `evidence=` values from doctor.

- **Read it back.** Ask for status. Run `verify-elevenlabs run --name init-status -- agents status`. Exit code `0`. The transcript is `No agents configured`.
- **Decline override.** Put a file at `agent_configs/precious.json`, then run the binary with stdin closed (not `verify-elevenlabs run`, because that tmux session keeps stdin open and the prompt blocks):

```bash
HOME="$HOME" env -u ELEVENLABS_API_KEY -u ELEVENLABS_BASE_URL \
  "$BINARY" agents init . --override </dev/null
```

Use the disposable `home=` and `binary=` from doctor, with the working directory set to `work=`. Exit code `0`. Stdout contains `Initialization cancelled`. `agent_configs/precious.json` is still there. Save that stdout in `$EVIDENCE/init-decline-override.transcript.txt` and do not treat exit code `0` alone as success.

## Gotchas

- `agents init . --override` exits 0 when the user declines. The proof is `Initialization cancelled` and the surviving file.
- Inside `verify-elevenlabs run`, stdin is a terminal, so `--override` without `--yes` waits at `Continue? (y/N):` until the 45 second timeout.
- `--yes` with `--override` deletes `agent_configs/`, `tool_configs/`, and `test_configs/` without asking. Do not pass `--yes` against a directory you do not intend to wipe.
- Running init from inside this repo lets `dotenvy` load a parent `.env`. Keep the work directory under `/tmp/verify-elevenlabs-runs/<id>/work`.
- A second init without `--override` skips files that already exist and prints `already exists (skipped)`. That is not a fresh scaffold.
- `agents init .` prints `Initializing project in <cwd>/.`. The cwd is the work directory, so the line contains that path plus `/.`.
