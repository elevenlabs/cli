//! Whether a browser this process opens lands on this machine, so the
//! approval page's redirect and clipboard can reach the CLI.

use std::path::Path;

/// False over SSH, in a container, or on Linux with no display.
pub(super) fn reachable_in(
    os: &str,
    var: &dyn Fn(&str) -> Option<String>,
    in_container: bool,
) -> bool {
    let set = |name: &str| var(name).is_some_and(|v| !v.trim().is_empty());
    if set("SSH_CONNECTION") || set("SSH_TTY") || set("SSH_CLIENT") {
        return false;
    }
    if in_container || set("CODESPACES") || set("REMOTE_CONTAINERS") {
        return false;
    }
    os != "linux" || set("WSL_DISTRO_NAME") || set("DISPLAY") || set("WAYLAND_DISPLAY")
}

pub(super) fn reachable() -> bool {
    let in_container = Path::new("/.dockerenv").exists()
        || std::fs::read_to_string("/proc/1/cgroup").is_ok_and(|c| {
            ["docker", "containerd", "kubepods"]
                .iter()
                .any(|m| c.contains(m))
        });
    reachable_in(
        std::env::consts::OS,
        &|n| std::env::var(n).ok(),
        in_container,
    )
}
