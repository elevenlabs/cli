//! A sign-in that outlives the command that started it.
//!
//! Approving in the browser can take minutes (a new account, an emailed
//! code), and coding agents stop or background shell commands long before
//! that. So `connect` hands the sign-in to a background worker and waits for
//! it only briefly; `connect --wait` waits again. The worker writes its
//! result to a file, so whichever command is waiting when it lands prints it.
//!
//! The worker holds `worker.lock` for its whole life, and the OS releases the
//! lock when it exits, however it exits. That lock is the only test of whether
//! a sign-in is running: no pids, clocks or timeouts to get wrong.
//!
//! The worker is started through a short-lived launcher, so it is not a
//! descendant of the agent's command and survives the agent killing that
//! command's process tree; it also runs in its own process group.
//!
//! One session per project folder, under `~/.elevenlabs/onboard/`, and one
//! sign-in waiting for the browser on the machine at a time: two would share
//! the callback port.

use std::ffi::OsString;
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use fern_cli_sdk::error::CliError;
use serde_json::Value;
use sha2::{Digest, Sha256};

use super::clipboard::Clipboard;
use super::env_file;
use super::reply::{Next, Reply};

/// How long the sign-in waits for the approval page: the framework's callback timeout.
pub(super) const SIGN_IN_MINUTES: u64 = 5;
/// How long a new worker may take to take its lock before starting counts as failed.
const STARTUP: Duration = Duration::from_secs(10);
const POLL: Duration = Duration::from_millis(250);

pub(super) const WORKER_FLAG: &str = "worker";
pub(super) const LAUNCH_FLAG: &str = "launch-worker";

pub(super) struct Session {
    dir: PathBuf,
}

/// The worker's hold on its session; released when the worker exits.
pub(super) struct WorkerLock(#[allow(dead_code)] File);

impl Session {
    pub fn for_project(project_dir: &Path) -> Result<Self, CliError> {
        let home = std::env::var_os("HOME")
            .or_else(|| std::env::var_os("USERPROFILE"))
            .ok_or_else(|| {
                CliError::Other(anyhow::anyhow!("no home folder to keep the sign-in in"))
            })?;
        let canonical = project_dir
            .canonicalize()
            .unwrap_or_else(|_| project_dir.to_path_buf());
        let id = Sha256::digest(canonical.to_string_lossy().as_bytes());
        let id: String = id.iter().take(8).map(|b| format!("{b:02x}")).collect();
        Ok(Self::in_dir(
            PathBuf::from(home)
                .join(".elevenlabs")
                .join("onboard")
                .join(id),
        ))
    }

    pub fn in_dir(dir: PathBuf) -> Self {
        Self { dir }
    }

    fn path(&self, name: &str) -> PathBuf {
        self.dir.join(name)
    }

    /// The last finished sign-in's reply, until a new one starts.
    pub fn result(&self) -> Option<Value> {
        let text = std::fs::read_to_string(self.path("result.json")).ok()?;
        serde_json::from_str(&text).ok()
    }

    /// A worker is waiting for the approval, or finishing: it holds its lock.
    pub fn running(&self) -> bool {
        let Ok(file) = File::open(self.path("worker.lock")) else {
            return false;
        };
        match file.try_lock() {
            Ok(()) => {
                let _ = file.unlock();
                false
            }
            Err(_) => true,
        }
    }

    /// Start a worker for this sign-in. `request` names what was asked for
    /// (the env file, a replacement): a worker already running for the same
    /// request is waited on instead, and so is one past the browser, which is
    /// storing a key the developer just approved. One for another request is
    /// replaced. Returns once the worker holds its lock, so every later command
    /// sees it running.
    pub fn start(&self, request: &str) -> Result<(), CliError> {
        let root = self.dir.parent().unwrap_or(&self.dir).to_path_buf();
        std::fs::create_dir_all(&self.dir).map_err(io(&self.dir))?;
        #[cfg(unix)]
        for dir in [&root, &self.dir] {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(dir, std::fs::Permissions::from_mode(0o700));
        }
        // One starter at a time on the machine, so two folders never both listen.
        let claim_path = root.join("claim.lock");
        let claim = open_lock(&claim_path)?;
        claim.lock().map_err(io(&claim_path))?;
        if self.running() {
            let same = std::fs::read_to_string(self.path("request")).is_ok_and(|r| r == request);
            if same || self.path("signed-in").exists() {
                return Ok(());
            }
            self.stop();
        }
        for stale in ["result.json", "worker.log", "worker.pid", "signed-in"] {
            let _ = std::fs::remove_file(self.path(stale));
        }
        std::fs::write(self.path("request"), request).map_err(io(&self.path("request")))?;
        self.stop_others();
        let status = this_program()?
            .args(worker_args(LAUNCH_FLAG))
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map_err(|e| CliError::Other(anyhow::anyhow!("start the sign-in: {e}")))?;
        let deadline = Instant::now() + STARTUP;
        while status.success() && !self.running() && self.result().is_none() {
            if Instant::now() >= deadline {
                break;
            }
            std::thread::sleep(Duration::from_millis(50));
        }
        if self.running() || self.result().is_some() {
            Ok(())
        } else {
            Err(CliError::Other(anyhow::anyhow!(
                "the sign-in didn't start (launcher: {status}); see {}",
                self.path("worker.log").display()
            )))
        }
    }

    /// Stop any other project's sign-in still waiting for the browser. Only
    /// one may listen for the browser's callback: two can hold the same port
    /// (one on IPv4, one on IPv6), and the callback then reaches the wrong one.
    /// One past the browser no longer listens, and is left to store its key.
    /// The stopped one's agent is told why, so it asks before signing in again.
    pub(super) fn stop_others(&self) {
        let Some(root) = self.dir.parent() else {
            return;
        };
        for entry in std::fs::read_dir(root).into_iter().flatten().flatten() {
            let other = Session::in_dir(entry.path());
            if other.dir == self.dir || !other.running() || other.path("signed-in").exists() {
                continue;
            }
            other.stop();
            let replaced = Reply::new(
                Next::AskUser,
                "A sign-in started for another project replaced this one. Tell me if you want to sign in for this project again.",
            )
            .detail("key_status", "sign_in_replaced");
            let _ = env_file::write(&other.path("result.json"), &replaced.to_value().to_string());
        }
    }

    /// End this session's worker. Its lock is held, so the pid it recorded is
    /// still that worker's. Stopped between the browser's callback and storing
    /// the key, it may leave the key the page copied, so that goes too.
    fn stop(&self) {
        let pid = std::fs::read_to_string(self.path("worker.pid"))
            .ok()
            .and_then(|t| t.trim().parse::<u32>().ok());
        if let Some(pid) = pid {
            stop(pid);
            Clipboard::open().discard_key();
        }
    }

    /// The launcher's job: start the worker detached, then exit.
    pub fn launch_worker(&self) -> Result<(), CliError> {
        let log = File::create(self.path("worker.log")).map_err(io(&self.path("worker.log")))?;
        let mut command = this_program()?;
        command
            .args(worker_args(WORKER_FLAG))
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(log);
        detach(&mut command);
        command
            .spawn()
            .map(drop)
            .map_err(|e| CliError::Other(anyhow::anyhow!("start the sign-in worker: {e}")))
    }

    /// The worker's first act: take the session's lock, and record its pid
    /// (in its own file: a locked file can't be read on Windows). `None` when
    /// another worker holds the lock. A command checking `running()` holds it
    /// for an instant, so taking it is retried briefly.
    pub fn take(&self) -> Result<Option<WorkerLock>, CliError> {
        let file = open_lock(&self.path("worker.lock"))?;
        let deadline = Instant::now() + Duration::from_millis(500);
        while file.try_lock().is_err() {
            if Instant::now() >= deadline {
                return Ok(None);
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        let pid = self.path("worker.pid");
        std::fs::write(&pid, std::process::id().to_string()).map_err(io(&pid))?;
        Ok(Some(WorkerLock(file)))
    }

    /// The browser step is over: another project's sign-in must not stop this one now.
    pub fn signed_in(&self, _lock: &WorkerLock) {
        let _ = std::fs::write(self.path("signed-in"), "");
    }

    /// The worker's last act: its reply, for whoever is waiting.
    pub fn finish(&self, _lock: &WorkerLock, reply: &Value) -> Result<(), CliError> {
        env_file::write(&self.path("result.json"), &reply.to_string())
    }

    /// Wait up to `budget` for the result, returning early when the worker is
    /// gone or `enough` says so.
    pub fn wait(&self, budget: Duration, enough: impl Fn(&Self) -> bool) -> Option<Value> {
        let deadline = Instant::now() + budget;
        loop {
            if let Some(result) = self.result() {
                return Some(result);
            }
            if !self.running() || enough(self) || Instant::now() >= deadline {
                return self.result();
            }
            std::thread::sleep(POLL);
        }
    }

    /// The approval link the worker printed (the sign-in flow logs `  URL: <url>` before it waits).
    pub fn url(&self) -> Option<String> {
        let log = std::fs::read_to_string(self.path("worker.log")).ok()?;
        log.lines()
            .find_map(|l| l.trim_start().strip_prefix("URL: "))
            .map(|url| url.trim().to_string())
            .filter(|url| url.starts_with("http"))
    }
}

fn open_lock(path: &Path) -> Result<File, CliError> {
    OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(path)
        .map_err(io(path))
}

/// This invocation's own arguments with `flag` added, so the worker signs in
/// exactly as this command was asked to.
fn worker_args(flag: &str) -> Vec<OsString> {
    worker_args_from(std::env::args_os().skip(1), flag)
}

/// `args` without the session's own flags, then `--<flag>`.
pub(super) fn worker_args_from(args: impl Iterator<Item = OsString>, flag: &str) -> Vec<OsString> {
    let ours = [format!("--{WORKER_FLAG}"), format!("--{LAUNCH_FLAG}")];
    let mut kept: Vec<OsString> = args
        .filter(|a| !ours.iter().any(|o| a == o.as_str()))
        .collect();
    kept.push(format!("--{flag}").into());
    kept
}

/// This program again, started with the shell's environment: by now this
/// process has the project's `.env` loaded, which git must never see.
fn this_program() -> Result<Command, CliError> {
    let exe = std::env::current_exe()
        .map_err(|e| CliError::Other(anyhow::anyhow!("find this program: {e}")))?;
    let mut command = Command::new(exe);
    if let Some(shell) = super::shell_env() {
        command.env_clear().envs(shell);
    }
    Ok(command)
}

#[cfg(unix)]
fn detach(command: &mut Command) {
    use std::os::unix::process::CommandExt;
    command.process_group(0);
}

#[cfg(windows)]
fn detach(command: &mut Command) {
    use std::os::windows::process::CommandExt;
    const DETACHED_PROCESS: u32 = 0x0000_0008;
    const CREATE_NEW_PROCESS_GROUP: u32 = 0x0000_0200;
    command.creation_flags(DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP);
}

/// End a worker and wait briefly for it to let go of its port. Only ever one
/// real process: never 0 or 1, and never a value that would read as a process
/// group.
fn stop(pid: u32) {
    if pid <= 1 || i32::try_from(pid).is_err() {
        return;
    }
    #[cfg(unix)]
    unsafe {
        libc::kill(pid as i32, libc::SIGTERM);
    }
    #[cfg(windows)]
    let _ = Command::new("taskkill")
        .args(["/PID", &pid.to_string(), "/F"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    let deadline = Instant::now() + Duration::from_secs(2);
    while Instant::now() < deadline && process_exists(pid) {
        std::thread::sleep(Duration::from_millis(50));
    }
}

#[cfg(unix)]
fn process_exists(pid: u32) -> bool {
    unsafe { libc::kill(pid as i32, 0) == 0 }
}

#[cfg(windows)]
fn process_exists(_pid: u32) -> bool {
    false
}

fn io(path: &Path) -> impl Fn(std::io::Error) -> CliError + '_ {
    move |e| CliError::Other(anyhow::anyhow!("{}: {e}", path.display()))
}
