# Release

Create a new release of the ElevenLabs CLI.

## Arguments

The user may provide a version number (e.g. `0.6.0`). If not provided, determine the next version by:
1. Running `git describe --tags --abbrev=0` to get the latest tag
2. Incrementing the patch version (e.g. `@elevenlabs/cli@0.5.1` -> `0.5.2`)
3. Confirming the version with the user before proceeding

## Version locations

The version string must be updated in exactly these files:

| File | Field |
|------|-------|
| `package.json` | `"version": "X.Y.Z"` |

After updating `package.json`, run `pnpm install --lockfile-only` to sync `pnpm-lock.yaml`.

## Steps

1. **Verify clean state**: Run `git status` on `main` branch. Abort if there are uncommitted changes or if not on `main`.

2. **Update version**: Edit `package.json` with the new version string and sync the lockfile.

3. **Build**: Run `pnpm run build` to verify the package compiles.

4. **Test**: Run `pnpm test` to verify all tests pass.

5. **Commit**: Stage `package.json` and `pnpm-lock.yaml` and commit:
   ```
   X.Y.Z
   ```

6. **Grep for old version**: Search the repo for any remaining references to the previous version string to make sure nothing was missed.

7. **Confirm with user**: Before pushing, show the user a summary of what will happen (push to main, create tag, create GitHub release) and ask for confirmation.

8. **Push and tag**:
   ```bash
   git push origin main
   git tag @elevenlabs/cli@X.Y.Z
   git push origin @elevenlabs/cli@X.Y.Z
   ```

9. **Create GitHub release**:
   ```bash
   gh release create @elevenlabs/cli@X.Y.Z --generate-notes
   ```

10. **Report**: Share the release URL with the user.
