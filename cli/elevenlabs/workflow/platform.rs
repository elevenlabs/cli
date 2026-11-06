//! Roughly where the CLI runs: the OS, whether it is over SSH, in a
//! container or in WSL, how it was installed, and which coding agent ran it.
//! The API is told in the User-Agent (see [`super::telemetry`]), so sign-in
//! and request problems can be counted per platform and agent, and npx runs
//! told apart from installs. Nothing else about the machine is sent, and
//! `--telemetry off` sends none of it.

use std::path::Path;

/// The prefixes of the tags below, and of the `signin/` tag `onboard` adds,
/// so an inherited User-Agent suffix can be cleared of them (see
/// [`super::telemetry`]).
pub(super) const TAG_PREFIXES: &[&str] = &["os/", "env/", "install/", "agent/", "signin/"];

/// `os/<os>`, `env/<env>` when one applies, `install/<source>`, then
/// `agent/<agent>` when a coding agent ran this: the User-Agent tags.
pub(super) fn tags() -> String {
    let tags = tags_for(detect(), install_source());
    match agent() {
        Some(agent) => format!("{tags} agent/{agent}"),
        None => tags,
    }
}

/// The coding agent running this command, from a variable it sets for the
/// commands it runs. Only the label leaves the machine, never a value.
pub(super) fn agent() -> Option<&'static str> {
    agent_for(&|name| std::env::var(name).ok())
}

/// Checked in this order: Cursor and VS Code forks also set `TERM_PROGRAM=vscode`.
pub(super) fn agent_for(var: &dyn Fn(&str) -> Option<String>) -> Option<&'static str> {
    let set = |name: &str| var(name).is_some_and(|v| !v.trim().is_empty());
    if set("CLAUDECODE") {
        // Claude Code in the desktop app is told apart: its built-in browser
        // can't write the clipboard the key travels on.
        let desktop = var("CLAUDE_CODE_ENTRYPOINT").as_deref() == Some("claude-desktop");
        Some(if desktop {
            "claude-desktop"
        } else {
            "claude-code"
        })
    } else if set("GEMINI_CLI") {
        Some("gemini-cli")
    } else if set("CODEX_SANDBOX") || set("CODEX_SANDBOX_NETWORK_DISABLED") {
        Some("codex")
    } else if set("CURSOR_AGENT") || set("CURSOR_TRACE_ID") {
        Some("cursor")
    } else if var("TERM_PROGRAM").as_deref() == Some("vscode") {
        Some("vscode")
    } else {
        None
    }
}

pub(super) fn tags_for((os, env): (&str, Option<&str>), install: &str) -> String {
    match env {
        Some(env) => format!("os/{os} env/{env} install/{install}"),
        None => format!("os/{os} install/{install}"),
    }
}

/// The OS (`windows`, `macos`, `linux` or `other`), and how the CLI is
/// reached there when not directly (`ssh`, `container` or `wsl`).
pub(super) fn detect() -> (&'static str, Option<&'static str>) {
    // Podman and Kubernetes count here, but not for whether a browser can
    // reach the CLI: toolbox and distrobox are Podman containers that share
    // the host's network and display, and telepresence copies a pod's
    // environment onto a laptop.
    let in_container = in_container()
        || Path::new("/run/.containerenv").exists()
        || std::env::var_os("KUBERNETES_SERVICE_HOST").is_some();
    detect_for(
        std::env::consts::OS,
        &|name| std::env::var(name).ok(),
        in_container,
    )
}

pub(super) fn detect_for(
    os: &str,
    var: &dyn Fn(&str) -> Option<String>,
    in_container: bool,
) -> (&'static str, Option<&'static str>) {
    let set = |name: &str| var(name).is_some_and(|v| !v.trim().is_empty());
    let os = match os {
        "windows" => "windows",
        "macos" => "macos",
        "linux" => "linux",
        _ => "other",
    };
    let env = if set("SSH_CONNECTION") || set("SSH_TTY") || set("SSH_CLIENT") {
        Some("ssh")
    } else if in_container || set("CODESPACES") || set("REMOTE_CONTAINERS") {
        Some("container")
    } else if os == "linux" && set("WSL_DISTRO_NAME") {
        Some("wsl")
    } else {
        None
    };
    (os, env)
}

/// `/.dockerenv`, or a container runtime in PID 1's cgroup.
pub(super) fn in_container() -> bool {
    Path::new("/.dockerenv").exists()
        || std::fs::read_to_string("/proc/1/cgroup").is_ok_and(|c| {
            ["docker", "containerd", "kubepods"]
                .iter()
                .any(|m| c.contains(m))
        })
}

/// How this binary got onto the machine, read from where it sits: each
/// package manager unpacks it somewhere recognisable. Only the label leaves
/// the machine, never the path.
pub(super) fn install_source() -> &'static str {
    match std::env::current_exe().and_then(std::fs::canonicalize) {
        Ok(exe) => install_source_for(&exe.to_string_lossy().replace('\\', "/")),
        Err(_) => "unknown",
    }
}

/// `path` uses `/` separators. The one-off runners come before `npm`
/// because they unpack into a `node_modules` folder too.
pub(super) fn install_source_for(path: &str) -> &'static str {
    let has = |part: &str| path.contains(part);
    if has("/_npx/") {
        "npx"
    } else if has("/pnpm/dlx/") {
        "pnpm-dlx"
    } else if has("/bunx-") {
        "bunx"
    } else if has("/dlx-") {
        "yarn-dlx"
    } else if has("/node_modules/@elevenlabs/cli-") {
        "npm"
    } else if has("/Cellar/elevenlabs/") {
        "homebrew"
    } else if has("/scoop/apps/elevenlabs/") {
        "scoop"
    } else if has("/.cargo/bin/") {
        // The shell and PowerShell installers put it in CARGO_HOME, and so
        // does `cargo install`; the path cannot tell them apart.
        "installer"
    } else {
        "other"
    }
}

#[cfg(test)]
mod tests {
    use super::{agent_for, detect_for, install_source_for, tags_for};

    fn vars(set: &'static [&'static str]) -> impl Fn(&str) -> Option<String> {
        move |name| set.contains(&name).then(|| "1".to_string())
    }

    #[test]
    fn the_os_and_how_the_cli_is_reached() {
        assert_eq!(detect_for("windows", &vars(&[]), false), ("windows", None));
        assert_eq!(
            detect_for("macos", &vars(&["SSH_TTY"]), false),
            ("macos", Some("ssh"))
        );
        assert_eq!(
            detect_for("linux", &vars(&["WSL_DISTRO_NAME"]), false),
            ("linux", Some("wsl"))
        );
        // Only Linux runs under WSL.
        assert_eq!(
            detect_for("windows", &vars(&["WSL_DISTRO_NAME"]), false),
            ("windows", None)
        );
        assert_eq!(
            detect_for("linux", &vars(&[]), true),
            ("linux", Some("container"))
        );
        assert_eq!(
            detect_for("linux", &vars(&["REMOTE_CONTAINERS"]), false),
            ("linux", Some("container"))
        );
        // SSH into WSL is still a remote shell.
        assert_eq!(
            detect_for("linux", &vars(&["SSH_CLIENT", "WSL_DISTRO_NAME"]), false),
            ("linux", Some("ssh"))
        );
        assert_eq!(detect_for("freebsd", &vars(&[]), false), ("other", None));
    }

    #[test]
    fn a_blank_variable_is_not_set() {
        let blank = |name: &str| (name == "SSH_TTY").then(|| "  ".to_string());
        assert_eq!(detect_for("macos", &blank, false), ("macos", None));
    }

    #[test]
    fn the_install_source_from_where_the_binary_sits() {
        let cases = [
            (
                "/Users/a/.npm/_npx/6c1d/node_modules/@elevenlabs/cli-darwin-arm64/elevenlabs",
                "npx",
            ),
            (
                "C:/Users/a/AppData/Local/npm-cache/_npx/6c1d/node_modules/@elevenlabs/cli-win32-x64/elevenlabs.exe",
                "npx",
            ),
            (
                "/Users/a/Library/Caches/pnpm/dlx/abc/node_modules/.pnpm/@elevenlabs+cli-darwin-arm64@1.7.1/node_modules/@elevenlabs/cli-darwin-arm64/elevenlabs",
                "pnpm-dlx",
            ),
            (
                "/tmp/bunx-501-@elevenlabs/cli@latest/node_modules/@elevenlabs/cli-linux-x64/elevenlabs",
                "bunx",
            ),
            (
                "/tmp/xfs-1a2b/dlx-4242/node_modules/@elevenlabs/cli-linux-x64/elevenlabs",
                "yarn-dlx",
            ),
            (
                "/opt/homebrew/lib/node_modules/@elevenlabs/cli/node_modules/@elevenlabs/cli-darwin-arm64/elevenlabs",
                "npm",
            ),
            (
                "C:/Users/a/AppData/Roaming/npm/node_modules/@elevenlabs/cli/node_modules/@elevenlabs/cli-win32-x64/elevenlabs.exe",
                "npm",
            ),
            ("/opt/homebrew/Cellar/elevenlabs/1.7.1/bin/elevenlabs", "homebrew"),
            (
                "/home/linuxbrew/.linuxbrew/Cellar/elevenlabs/1.7.1/bin/elevenlabs",
                "homebrew",
            ),
            ("C:/Users/a/scoop/apps/elevenlabs/1.7.1/elevenlabs.exe", "scoop"),
            ("/home/a/.cargo/bin/elevenlabs", "installer"),
            ("/home/a/code/cli/target/debug/elevenlabs", "other"),
        ];
        for (path, want) in cases {
            assert_eq!(install_source_for(path), want, "{path}");
        }
    }

    #[test]
    fn the_install_tag_follows_the_platform_tags() {
        assert_eq!(tags_for(("macos", None), "npx"), "os/macos install/npx");
        assert_eq!(
            tags_for(("linux", Some("ssh")), "homebrew"),
            "os/linux env/ssh install/homebrew"
        );
    }

    #[test]
    fn the_coding_agent_from_the_variables_it_sets() {
        type Vars = &'static [(&'static str, &'static str)];
        let with = |set: Vars| {
            move |name: &str| {
                set.iter()
                    .find(|(k, _)| *k == name)
                    .map(|(_, v)| v.to_string())
            }
        };
        let cases: [(Vars, Option<&str>); 9] = [
            (&[("CLAUDECODE", "1")], Some("claude-code")),
            (
                &[("CLAUDECODE", "1"), ("CLAUDE_CODE_ENTRYPOINT", "cli")],
                Some("claude-code"),
            ),
            (
                &[
                    ("CLAUDECODE", "1"),
                    ("CLAUDE_CODE_ENTRYPOINT", "claude-desktop"),
                ],
                Some("claude-desktop"),
            ),
            (&[("GEMINI_CLI", "1")], Some("gemini-cli")),
            (&[("CODEX_SANDBOX", "seatbelt")], Some("codex")),
            (
                &[("CURSOR_TRACE_ID", "abc"), ("TERM_PROGRAM", "vscode")],
                Some("cursor"),
            ),
            (&[("TERM_PROGRAM", "vscode")], Some("vscode")),
            (&[("TERM_PROGRAM", "iTerm.app")], None),
            (&[("CLAUDECODE", " ")], None),
        ];
        for (set, want) in cases {
            assert_eq!(agent_for(&with(set)), want, "{set:?}");
        }
    }
}
