---
name: elevenlabs-say
description: Speak text aloud from the terminal with `elevenlabs say`. Use when asked to say/speak/read something out loud, announce or narrate a result, play text as audio, generate a quick voiceover or MP3 from text, make spoken output sound excited/whispered/emotional via v3 audio tags, or set a default voice/model/player for the CLI.
---

# `elevenlabs say`

Turns text into speech and plays it, without picking a voice ID, a file path
or a player.

```bash
elevenlabs say "this came from the terminal"
```

Needs credentials: either `elevenlabs auth login` or `ELEVENLABS_API_KEY` in
the environment.

## Reading the text

Three equivalent sources — pick whichever suits the caller:

```bash
elevenlabs say "hello there"          # positional (quote it)
elevenlabs say hello there            # bare words are joined with spaces
echo "build finished" | elevenlabs say  # piped stdin
elevenlabs say -                      # explicit stdin
```

With no argument and no piped input, `say` errors instead of hanging.

> `config` is a subcommand, so speaking that exact word needs
> `elevenlabs say -- config`. Put any flags **before** the `--`.

## Defaults

Stored per user in `~/.elevenlabs/config.json` under a `say` key, alongside
the `residency` setting. Nothing sensitive goes in that file — credentials
stay in the OS keyring.

```bash
elevenlabs say config                                 # show everything
elevenlabs say config voice 21m00Tcm4TlvDq8ikWAM
elevenlabs say config model eleven_multilingual_v2
elevenlabs say config output-format wav_44100
elevenlabs say config player mpv
elevenlabs say config voice                           # show just this one
elevenlabs say config voice --unset                   # back to the built-in
```

| Key | Config field | Built-in default |
|---|---|---|
| `voice` | `say.voice_id` | `JBFqnCBsd6RMkjVDRZzb` (George) |
| `model` | `say.model_id` | `eleven_v3` |
| `output-format` | `say.output_format` | follows the player |
| `player` | `say.player` | first one found on `PATH` |

`eleven_v3` is the most expressive model and the only family that understands
the audio tags below. `eleven_flash_v2_5` is the swap when latency matters
more than delivery — roughly a second quicker to first audio.

Find voice IDs with `elevenlabs voices search`, and model IDs with
`elevenlabs models list`.

## Per-run overrides

Every default has a flag that wins for one invocation:

```bash
elevenlabs say "one off" \
  --voice JBFqnCBsd6RMkjVDRZzb \
  --model eleven_multilingual_v2 \
  --output-format mp3_44100_192 \
  --player mpv
```

Precedence is always **flag > config file > built-in default**.

## Audio tags (v3 only)

`eleven_v3` reads inline square-bracket directions as stage directions rather
than as words, which is what makes a line land as speech instead of narration:

```bash
elevenlabs say "[excited] The build passed!"
elevenlabs say "[whispers] don't tell anyone [laughs]"
elevenlabs say "[sarcastic] Oh, brilliant. [sighs]"
```

Three rough families:

- **Delivery and emotion** — `[excited]`, `[curious]`, `[sarcastic]`,
  `[whispers]`, `[mischievously]`, `[crying]`
- **Non-verbal sounds** — `[laughs]`, `[laughs harder]`, `[starts laughing]`,
  `[sighs]`, `[exhales]`, `[snorts]`
- **Experimental** — `[strong French accent]`, `[sings]`, and effects such as
  `[applause]` or `[gunshot]`

Tags belong to the v3 family. On `eleven_flash_v2_5`, `eleven_turbo_v2_5` or
`eleven_multilingual_v2` they are not directions — they get read out as
literal text, so `[excited] hello` is spoken as the word "excited" followed by
"hello".

How well a tag lands depends on the voice: one whose training fights the
direction (`[shout]` on a soft narrator) will ignore it, tags can be combined,
and the experimental ones are worth trying before you depend on them.

More information: <https://elevenlabs.io/docs/overview/capabilities/text-to-speech/best-practices#audio-tags>

## Saving instead of playing

`--output` writes the audio and skips playback entirely, so it works on a
machine with no audio player and no sound device:

```bash
elevenlabs say "saved to a file" --output out.mp3
elevenlabs say "as wav" -o out.wav        # extension picks the format
```

## Audio formats

`output_format` is `codec_samplerate[_bitrate]`, e.g. `mp3_44100_128`,
`wav_44100`, `pcm_24000`, `opus_48000_128`, `ulaw_8000`. Which one gets used:

1. `--output-format`, if given.
2. The `--output` filename's extension (`.mp3`, `.wav`, `.opus`/`.ogg`,
   `.pcm`/`.raw`, `.ulaw`, `.alaw`) — a filename beats the stored default so
   `out.wav` never ends up holding MP3 bytes.
3. The configured `say.output_format`.
4. The player's requirement: WAV for the WAV-only players below, else
   `mp3_44100_128`.

## Players

Playback shells out; nothing is linked into the binary. The first of these
found on `PATH` wins, unless `--player` or `say config player` names one:

| Player | Reads stdin | Accepts | Notes |
|---|---|---|---|
| `ffplay` | yes | any | from ffmpeg |
| `mpv` | yes | any | |
| `afplay` | no | any | macOS, always present |
| `paplay` | no | WAV only | Linux/PulseAudio |
| `aplay` | no | WAV only | Linux/ALSA |
| PowerShell `Media.SoundPlayer` | no | WAV only | Windows |

stdin-capable players are preferred because audio starts on the first chunk
instead of after the whole download. The others get a temp file, played once
the stream finishes.

`--player` also accepts a command that is not in that table (a wrapper
script, say). Unknown players are driven conservatively: the audio is written
to a temp file and the path is passed as the single argument.

Arguments are always passed individually, never through a shell.

## Troubleshooting

**`No audio player found.`** — install ffmpeg or mpv, pass `--player`, or use
`--output <path>` to skip playback.

**`Audio player 'x' was not found on PATH.`** — the configured or requested
player is missing. `elevenlabs say config player --unset` returns to
auto-detection.

**`The 'aplay' player only handles WAV, but the format is 'mp3_44100_128'.`**
— either `--output-format wav_44100` or switch to a player that decodes MP3.

**A 401/422 prints a JSON error envelope and exits non-zero.** The request is
sent and its status checked *before* any player is spawned or file created,
so a failed call never plays an error body or leaves a partial file behind.
Exit codes follow the CLI's convention: `1` for an API error, `3` for a
validation error.

**Requests go to the wrong region.** `say` honors `elevenlabs residency`,
`ELEVENLABS_BASE_URL` and `--base-url` like every other command.
