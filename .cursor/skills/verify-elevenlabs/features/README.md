# ElevenLabs CLI verification map

This directory is the maintained source for verifying the user-facing behavior of the `elevenlabs` CLI. Read the index before driving the binary, then use the matching feature file as the recipe.

The surface is the terminal binary built from this repo. The xi web app, the Fern docs site, and the skills catalog are not part of this map.

## Baseline preconditions

- Run `.cursor/skills/verify-elevenlabs/bin/verify-elevenlabs launch` and require a `ready: elevenlabs <version> run=<id>` line.
- Run `.cursor/skills/verify-elevenlabs/bin/verify-elevenlabs doctor` and require `ok`, `api_key=unset`, and `port=none`.
- Use the `home=` and `work=` paths doctor prints. Both are under `/tmp/verify-elevenlabs-runs/<id>/`. The home directory is not the user's real home.
- The work directory starts empty. `dotenvy` must not see a repo `.env`, so the work directory stays outside this checkout.
- `ELEVENLABS_API_KEY` is unset inside the drive. Do not export a key to make a recipe pass.
- Never drive an instance whose doctor failed, the real `~/.elevenlabs`, or the repo path `.verify-project`.

## Driving conventions

- Start every recipe from the baseline state unless its preconditions say otherwise.
- Treat every command as literal. Keep quoted arguments and flags unchanged.
- Run CLI actions through `verify-elevenlabs run --name <name> -- <args>`.
- One `run` is one command. A follow-up view is another `run` with a different `--name`.
- Pass `--run-id` on every command when more than one verification run is alive.
- Restore nothing in the user's home. The disposable home is deleted by cleanup.
- Do not remove proof artifacts during cleanup.

## Proof and skip reporting

- Capture the user action and the resulting state, not only the last line.
- CLI proof includes the tmux transcript, the exit code file, and the `.meta` file written beside them.
- Mutation proof includes a second user-facing command that reads the new state back, plus the files that command wrote.
- Record the feature file and the `--name` with every artifact. The `.meta` `feature_command` field is that name.
- Report an unreachable path with the attempted command and the unmet precondition.
- Do not report a skipped entry point as verified through a different path.
- `agents add`, `push`, `pull`, tool creation, and test creation call the API. They are not recipes in this map.

## Feature entry contract

Each feature file starts with an H1 title and one paragraph describing the user-visible behavior. It then uses exactly four H2 sections in this order.

1. `Sub-features` lists short IDs with one line for each behavior.
2. `How to get to it (user POV)` lists every user entry point.
3. `Driving it with verify-elevenlabs` starts with `Preconditions:` and uses labeled bullets that pair each user action with an exact command and observable result.
4. `Gotchas` lists traps that can waste or invalidate a verification run.

## Features

- [Initialize a project](./agents-init.md) covers scaffolding an agents-as-code project and declining a destructive override.
- [Agent templates](./agent-templates.md) covers listing built-in templates and showing one template's JSON.
- [Agent status](./agents-status.md) covers the missing-project error and the empty project status.
- [Say defaults](./say-config.md) covers reading, setting, and clearing local speech defaults.
- [Residency](./residency.md) covers reading and switching the local data-residency region.
