# Quick start

Quick start is the first command the README tells a user to run after install: `elevenlabs --help` lists the top-level commands.

## Sub-features

- `help-root` prints the usage line and the commands a new user needs next.

## How to get to it (user POV)

- Run `elevenlabs --help`.
- Run `elevenlabs <resource> --help` to see methods for one resource. That is a second command, not this recipe.

## Driving it with verify-elevenlabs

Preconditions:

- `verify-elevenlabs doctor` prints `ok` and `profile=release` for this run.
- No project files and no API key are required.

- **Help.** Run `verify-elevenlabs run --name help-root -- --help`. Exit code `0`. The transcript contains `Usage: elevenlabs`, `agents`, `say`, and `residency`. The typed command is `elevenlabs --help`, not a path under `target/debug`.
- **Proof.** The transcript is the help text. It does not contain an HTTP status.

## Gotchas

- `--help` is an argument to `elevenlabs`, so it belongs after `--` in the harness: `run --name help-root -- --help`.
- Help text is long. Assert the usage line and the command names, not the whole page.
- `elevenlabs --help --format json` prints a machine-readable catalog. It is a different invocation from the README quick start.
