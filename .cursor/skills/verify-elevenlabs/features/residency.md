# Residency

Residency shows and stores the region that selects the API base URL. The value lives in `$HOME/.elevenlabs/config.json`. Showing or setting it does not send a request.

## Sub-features

- `residency-show` prints the default region and its base URL.
- `residency-set` stores `eu-residency` and shows that region again.
- `residency-invalid` rejects a region name that does not exist.

## How to get to it (user POV)

- Run `elevenlabs residency` to show the current region.
- Run `elevenlabs residency <region>` to switch. The regions are `us`, `global`, `eu-residency`, `in-residency`, and `sg-residency`.

## Driving it with verify-elevenlabs

Preconditions:

- `verify-elevenlabs doctor` prints `ok` for this run.
- Doctor's `home=` is the disposable home, not the user's real home.
- Drive this feature before `say-config`, or use a fresh run id. Both features write `$HOME/.elevenlabs/config.json`.

- **Show the default.** Run `verify-elevenlabs run --name residency-show -- residency`. Exit code `0`. The transcript contains `Residency: global`, `Base URL:  https://api.elevenlabs.io`, `Available: us, global, eu-residency, in-residency, sg-residency`, and `Set one with 'elevenlabs residency <region>'.`
- **Switch region.** Run `verify-elevenlabs run --name residency-set -- residency eu-residency`. Exit code `0`. The transcript contains `Residency set to: eu-residency` and `Base URL:  https://api.eu.residency.elevenlabs.io`.
- **Read it back.** Run `verify-elevenlabs run --name residency-read -- residency`. Exit code `0`. The transcript contains `Residency: eu-residency` and `Base URL:  https://api.eu.residency.elevenlabs.io`. The disposable home's `.elevenlabs/config.json` contains `"residency": "eu-residency"`. Copy that file to `$EVIDENCE/residency.config.json` before cleanup.
- **Reject an unknown region.** Run `verify-elevenlabs run --name residency-invalid -- residency volcano`. Exit code `3`. The transcript contains `Invalid residency 'volcano'. Available:`. A following `residency` command still prints `Residency: eu-residency`.
- **Proof.** The read-back transcript and the copied config file both show `eu-residency`. The real user home is unchanged.

## Gotchas

- Setting residency in the real home changes the base URL for every later `elevenlabs` command that user runs. Use the disposable home only.
- The show line is `Base URL:` followed by two spaces, then the URL.
- `--base-url` and `ELEVENLABS_BASE_URL` override the stored region for one command. The harness unsets `ELEVENLABS_BASE_URL` before the command so the stored value is what the process would export.
- `global` and `us` are different regions. `global` uses `https://api.elevenlabs.io`. `us` uses `https://api.us.elevenlabs.io`.
- An invalid region does not roll back a previous successful set.
