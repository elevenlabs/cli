//! Whether a browser this process opens lands on this machine, so the
//! approval page's redirect and clipboard can reach the CLI.

use super::super::platform::{detect_for, in_container};

/// False over SSH, in a container, or on Linux with no display.
pub(super) fn reachable_in(
    os: &str,
    var: &dyn Fn(&str) -> Option<String>,
    in_container: bool,
) -> bool {
    let set = |name: &str| var(name).is_some_and(|v| !v.trim().is_empty());
    let (os, env) = detect_for(os, var, in_container);
    match env {
        Some("ssh" | "container") => false,
        // WSL: the browser opens on Windows.
        Some(_) => true,
        None => os != "linux" || set("DISPLAY") || set("WAYLAND_DISPLAY"),
    }
}

pub(super) fn reachable() -> bool {
    reachable_in(
        std::env::consts::OS,
        &|n| std::env::var(n).ok(),
        in_container(),
    )
}
