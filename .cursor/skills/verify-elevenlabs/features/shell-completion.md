# Shell completion

Shell completion prints a script the user sources. Generating it does not call the API and does not change the shell.

## Sub-features

- `completion-bash` prints a bash completion script for `elevenlabs`.

## How to get to it (user POV)

- Run `elevenlabs completion bash`.
- The same command accepts `zsh`, `fish`, `powershell`, and `elvish`.

## Driving it with verify-elevenlabs

Preconditions:

- `verify-elevenlabs doctor` prints `ok` for this run.
- No project files and no API key are required.

- **Bash script.** Run `verify-elevenlabs run --name completion-bash -- completion bash`. Exit code `0`. The transcript contains the typed command `elevenlabs completion bash` and the line `complete -F _elevenlabs -o nosort -o bashdefault -o default elevenlabs`.
- **Proof.** The transcript is the script. It does not contain an HTTP status. The work directory is unchanged.

## Gotchas

- The script is large because it covers every subcommand (on the order of 75,000 lines). Assert the `complete -F _elevenlabs` line, not a particular resource. A tmux history of 10000 lines keeps only the tail, including when `history-limit` is raised after the window exists. The helper raises the server default before creating the window and restores it immediately after.
- `completion` with no shell name prints help and exits non-zero. This recipe always passes `bash`.
- Sourcing the script into the user's real shell is not part of the drive. The harness only runs the CLI.
