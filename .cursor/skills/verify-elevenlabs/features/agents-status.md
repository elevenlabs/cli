# Agent status

Agent status prints the local registry in `agents.json`. With no project it tells the user to run init. With a freshly initialized project it reports that no agents are configured.

## Sub-features

- `status-missing` fails when `agents.json` is absent.
- `status-empty` reports an initialized project that has no agents.

## How to get to it (user POV)

- Run `elevenlabs agents status` in a project directory.
- Run it outside a project to see the init hint.

## Driving it with verify-elevenlabs

Preconditions:

- `verify-elevenlabs doctor` prints `ok` for this run.
- For `status-missing`, the work directory has no `agents.json`.
- For `status-empty`, `agents init .` has already succeeded in that work directory and nothing has been added.

- **Missing project.** Run `verify-elevenlabs run --name status-missing -- agents status`. Exit code `3`. The transcript contains `agents.json not found. Run 'elevenlabs agents init' first.`
- **Empty project.** After a successful init in the same work directory, run `verify-elevenlabs run --name status-empty -- agents status`. Exit code `0`. The transcript contains `No agents configured` and does not contain `Agent Status:`.
- **Proof.** Keep both transcripts. They are different states of the same command, and one does not stand in for the other.

## Gotchas

- `agents list` is the API command for remote agents, not this local view. The local command is `agents status`.
- Status reads `agents.json` from the current directory. The harness current directory is the run's work directory.
- A populated status (`Not pushed yet` or `Created`) needs an `agents.json` entry that points at a real config file. `agents add` creates that entry by calling the API. Do not invent a passing populated status by editing the index unless the recipe says to, and do not call `agents add` from this map.
- Exit code `3` is the validation failure. Exit code `0` with `No agents configured` is the empty success. Do not collapse them.
