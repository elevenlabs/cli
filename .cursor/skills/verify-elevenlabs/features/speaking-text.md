# Speaking text

Speaking text turns a sentence into speech. The README's playback command needs a working audio device. Writing a file with `--output` is the same feature without a player, and it is the path to drive on a machine that has no sound device.

## Sub-features

- `say-output` sends the README sentence to text-to-speech and writes an MP3.
- `say-playback` is the README command that plays audio. Drive it only when a player can open a sound device.

## How to get to it (user POV)

- Run `elevenlabs say "this came from the terminal"` to play speech.
- Run `elevenlabs say "save it" --output out.mp3` to write a file and skip playback.
- A missing player fails before the API call. A missing sound device does not: the CLI still requests audio and hands it to the player.

## Driving it with verify-elevenlabs

Preconditions:

- `verify-elevenlabs doctor` prints `ok` for this run.
- The parent environment already has `ELEVENLABS_API_KEY`. If it does not, stop and report that variable. Do not invent a key.
- Stored residency is `global` or unset. If `residency` was switched earlier in this home, run `residency global` first so the request does not go to another region.
- `say config` has not left a custom voice or model, or those defaults are acceptable.
- For `say-playback`, `/dev/snd` exists and `ffplay` or `mpv` is on `PATH`. If `/dev/snd` is missing, skip playback and drive `say-output` instead. Report that skip with the unmet device, not as a failed synthesis.

- **Write a file.** Run `verify-elevenlabs run --timeout 180 --with-api-key --name say-output -- say "this came from the terminal" --output from-terminal.mp3`. Exit code `0`. The transcript contains `Saved to from-terminal.mp3` and does not contain the API key. The work directory has `from-terminal.mp3`, the file is non-empty, and its first bytes are an MP3 frame or an ID3 tag. Copy it to `$EVIDENCE/from-terminal.mp3` before cleanup.
- **Proof.** The exit file is `0`, the copied MP3 is non-empty, and `.meta` says `api_key=forwarded`. The real user home is unchanged.

## Gotchas

- `say config` is a different feature. `elevenlabs say config` does not synthesize speech.
- Playback without `/dev/snd` still calls the API, then ffplay fails to open ALSA. That spends a request and does not prove playback. Use `--output`.
- The default model is `eleven_v3`. A short sentence can take longer than the 45 second harness timeout. Pass `--timeout 180`.
- `--output` skips the player check, so a missing `ffplay` does not block this recipe.
- Do not pass `--xi-api-key` or `--dry-run`. Both can put the secret in the transcript. The helper rejects `--xi-api-key`.
- `say` does not create or delete agents. It does spend text-to-speech quota on the account that owns the key.
