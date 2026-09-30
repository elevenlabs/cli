# Say defaults

Say defaults shows and stores the voice, model, format, and player used by `elevenlabs say`. Values live in `$HOME/.elevenlabs/config.json`. Showing them does not synthesize speech.

## Sub-features

- `say-show` prints the four defaults.
- `say-set` stores a voice id and shows it again.
- `say-unset` clears that voice id and returns to the built-in default.

## How to get to it (user POV)

- Run `elevenlabs say config` to show every default.
- Run `elevenlabs say config voice` to show one setting.
- Run `elevenlabs say config voice <voice-id>` to store a voice.
- Run `elevenlabs say config voice --unset` to clear it.
- The same shape works for `model`, `output-format`, and `player`.

## Driving it with verify-elevenlabs

Preconditions:

- `verify-elevenlabs doctor` prints `ok` for this run.
- Doctor's `home=` is the disposable home, not the user's real home.
- No `~/.elevenlabs` directory exists under that disposable home yet.

- **Show defaults.** Run `verify-elevenlabs run --name say-show -- say config`. Exit code `0`. The transcript contains a `Voice:` line with `JBFqnCBsd6RMkjVDRZzb (default)`, a `Model:` line with `eleven_v3 (default)`, a `Format:` line with `mp3_44100_128 (default, follows the player)`, a `Player:` line, and `Set one with 'elevenlabs say config <voice|model|output-format|player> <value>'.`
- **Set a voice.** Run `verify-elevenlabs run --name say-set -- say config voice 21m00Tcm4TlvDq8ikWAM`. Exit code `0`. The transcript contains `Voice set to: 21m00Tcm4TlvDq8ikWAM`.
- **Read it back.** Run `verify-elevenlabs run --name say-read -- say config voice`. Exit code `0`. The transcript contains a `Voice:` line with `21m00Tcm4TlvDq8ikWAM` and that line does not contain `(default)`. The disposable home now has `.elevenlabs/config.json` whose `say.voice_id` is `21m00Tcm4TlvDq8ikWAM`. Copy that file to `$EVIDENCE/say-config.config.json` before cleanup.
- **Clear it.** Run `verify-elevenlabs run --name say-unset -- say config voice --unset`. Exit code `0`. The transcript contains `Voice cleared.` and `JBFqnCBsd6RMkjVDRZzb (default)`.
- **Proof.** The read-back transcript and the copied `config.json` agree on the voice id. The real user home has no new `.elevenlabs/config.json` from this run.

## Gotchas

- `say config` writes the real `~/.elevenlabs/config.json` when `HOME` is the user's home. Doctor must show a disposable home before any set or unset.
- `elevenlabs say "hello"` calls the API and plays audio. It is not this feature.
- The `Player:` line depends on what is installed (`ffplay`, `mpv`, `paplay`, `aplay`, or `none found on PATH`). Assert that the label is present, not a particular player.
- `config` is a subcommand. `elevenlabs say config` changes settings. Speaking the word config aloud would be `elevenlabs say -- config`, which needs an API key.
- A voice id is stored as a string. This recipe does not check that ElevenLabs hosts that voice.
