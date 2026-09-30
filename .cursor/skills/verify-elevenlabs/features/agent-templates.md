# Agent templates

Agent templates lists the built-in starting configurations and prints one template as JSON, without creating a project or calling the API.

## Sub-features

- `templates-list` prints every built-in template name and description.
- `templates-show` prints one template's JSON.
- `templates-unknown` rejects a template name that does not exist.

## How to get to it (user POV)

- Run `elevenlabs agents templates list`.
- Run `elevenlabs agents templates show <template>`.
- The names are `default`, `minimal`, `voice-only`, `text-only`, `customer-service`, and `assistant`.

## Driving it with verify-elevenlabs

Preconditions:

- `verify-elevenlabs doctor` prints `ok` for this run.
- No project files are required. Templates do not read `agents.json`.

- **List.** Run `verify-elevenlabs run --name templates-list -- agents templates list`. Exit code `0`. The transcript contains `Available agent templates:` and each name on its own line: `default`, `minimal`, `voice-only`, `text-only`, `customer-service`, `assistant`. It also contains `Use 'elevenlabs agents add <name> --template <template_name>' to create an agent with a specific template`.
- **Show.** Run `verify-elevenlabs run --name templates-show -- agents templates show minimal`. Exit code `0`. The transcript contains `Template: minimal` and a JSON object whose `name` is `Example`.
- **Unknown name.** Run `verify-elevenlabs run --name templates-unknown -- agents templates show volcano`. Exit code `3`. The transcript contains `Unknown template type 'volcano'. Available:`.
- **Proof.** The list transcript names all six templates, and the show transcript includes `"name": "Example"`. The work directory is still empty.

## Gotchas

- `agents templates show` fills the sample name as `Example`. Assert that string, not the template type, inside the JSON `name` field.
- Listing templates does not write `agents.json`. A later `agents status` still reports that init has not been run, unless init was driven first.
- `agents add <name> --template <template>` uploads an agent. It is not the way to prove this feature.
