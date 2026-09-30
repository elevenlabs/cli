# Read the signed-in user

`elevenlabs user get` is the README's concrete authenticated call. It reads the account behind `ELEVENLABS_API_KEY` and does not create or delete anything.

## Sub-features

- `user-get` prints the user JSON for the key already in the environment.

## How to get to it (user POV)

- Export `ELEVENLABS_API_KEY`, then run `elevenlabs user get`.
- For a one-off call the README also shows `elevenlabs user get --xi-api-key xi-...`. The harness does not use that form, because the flag value would be copied into the transcript.

## Driving it with verify-elevenlabs

Preconditions:

- `verify-elevenlabs doctor` prints `ok` for this run.
- The parent environment already has `ELEVENLABS_API_KEY`. If it does not, stop and report that variable. Do not invent a key.
- Stored residency is `global` or unset.

- **Get the user.** Run `verify-elevenlabs run --with-api-key --name user-get -- user get`. Exit code `0`. The transcript contains `"user_id"` and `"subscription"`. It does not contain the API key string and it does not contain an HTTP error status.
- **Proof.** `.meta` says `api_key=forwarded`. When copying the transcript out of the evidence directory, remove `first_name`, `xi_api_key_preview`, `referral_link_code`, and `partnerstack_partner_default_link`. Those fields are account data, not part of the proof.

## Gotchas

- Without a key the command fails authentication. That failure is not a successful read.
- `--dry-run` prints the request, including the `xi-api-key` header. Do not use it here.
- The JSON includes a key preview. Treat the whole body as sensitive even after the helper redacts the full key.
- `user get` is a read. It is not a reason to run `agents add`, `agents push`, or `scripts/verify-agents-as-code.sh`.
