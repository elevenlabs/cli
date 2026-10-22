//! Roughly where the CLI runs: the OS, and whether it is over SSH, in a
//! container or in WSL. The API is told in the User-Agent (see
//! [`super::api::command_scope`]), so sign-in and request problems can be
//! counted per platform. Nothing else about the machine is sent.

use std::path::Path;

/// `os/<os>`, then `env/<env>` when one applies: the User-Agent tags.
pub(super) fn tags() -> String {
    match detect() {
        (os, Some(env)) => format!("os/{os} env/{env}"),
        (os, None) => format!("os/{os}"),
    }
}

/// The OS (`windows`, `macos`, `linux` or `other`), and how the CLI is
/// reached there when not directly (`ssh`, `container` or `wsl`).
pub(super) fn detect() -> (&'static str, Option<&'static str>) {
    detect_for(
        std::env::consts::OS,
        &|name| std::env::var(name).ok(),
        in_container(),
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
    } else if in_container
        || set("CODESPACES")
        || set("REMOTE_CONTAINERS")
        || set("KUBERNETES_SERVICE_HOST")
    {
        Some("container")
    } else if os == "linux" && set("WSL_DISTRO_NAME") {
        Some("wsl")
    } else {
        None
    };
    (os, env)
}

/// `/.dockerenv`, Podman's `/run/.containerenv`, or a container runtime in
/// PID 1's cgroup.
pub(super) fn in_container() -> bool {
    Path::new("/.dockerenv").exists()
        || Path::new("/run/.containerenv").exists()
        || std::fs::read_to_string("/proc/1/cgroup").is_ok_and(|c| {
            ["docker", "containerd", "kubepods"]
                .iter()
                .any(|m| c.contains(m))
        })
}

#[cfg(test)]
mod tests {
    use super::detect_for;

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
        assert_eq!(
            detect_for("linux", &vars(&["KUBERNETES_SERVICE_HOST"]), false),
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
}
