//! The manual key path: where the approval page can't hand a key over (a
//! remote machine, a refused copy, an account that can't create keys this
//! way), the developer creates one and pastes it into the env file. The file
//! is prepared first, so pasting is the only thing left to do.

use std::path::Path;

use fern_cli_sdk::error::CliError;

use super::env_file::{self, API_KEY_ENV};
use super::project::{shell_arg, Project};
use super::reply::{Next, Reply};

pub(super) const API_KEYS_PAGE: &str = "https://elevenlabs.io/app/settings/api-keys";

pub(super) const NO_BROWSER: &str = "A browser can't reach this machine (an SSH session, a container or a cloud environment), so the approval page can't hand a key to it.";

/// The manual steps for putting a key in `file` (the project's env file, or
/// the file that holds a rejected key). `why` opens the message.
pub(super) fn reply(project: &Project, file: &Path, why: &str) -> Result<Reply, CliError> {
    prepare(file)?;
    let name = project.show(file);
    Ok(Reply::new(
        Next::AskUser,
        format!(
            "{why} Please add a key by hand:\n\
             1. Create a key at {API_KEYS_PAGE}.\n\
             2. Paste it after {API_KEY_ENV}= in {name}, or set {API_KEY_ENV} in this environment's secrets.\n\
             3. Never paste it into this chat. Tell me when it's done."
        ),
    )
    .run(&project.next(&format!("check --env-file {}", shell_arg(&name))))
    .detail("env_file", name))
}

/// An empty key line with a note above it, and the file gitignored if it can
/// be; when it can't, `check` stops on that before the key is used.
pub(super) fn prepare(file: &Path) -> Result<(), CliError> {
    let existing = env_file::read(file)?;
    if env_file::file_key(&existing).is_none() {
        env_file::write(file, &env_file::with_key(&existing, ""))?;
    }
    let _ = env_file::ensure_ignored(file);
    Ok(())
}
