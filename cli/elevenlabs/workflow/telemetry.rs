//! Anonymous usage telemetry: where the CLI runs and how it was installed
//! (see [`super::platform`]), carried as User-Agent tags on every request so
//! the API's `server_cli_request` analytics can count, say, npx runs of
//! `onboard` against installed ones.
//!
//! On by default. `--telemetry off` on any command, `ELEVENLABS_TELEMETRY=off`,
//! or `DO_NOT_TRACK=1` turns it off, and then neither the User-Agent nor the
//! sign-in's approval link says anything about the machine. The flag wins
//! over the env vars, so `--telemetry on` re-enables it for one command.
//!
//! What it does not cover: the `cmd/<command>` tag (see
//! [`super::api::command_scope`]) and `--intent`, which an agent supplies on
//! purpose. Both name what was asked of the API, not the machine asking.
//!
//! Like `--intent`, the flag is read from argv before clap parses anything:
//! the generated `<resource> <method>` commands never call into this crate,
//! so setting the User-Agent suffix env var up front is the only way their
//! requests get the tags too.

use std::sync::OnceLock;

use fern_cli_sdk::app::CliApp;
use fern_cli_sdk::openapi::discovery::{
    GlobalParameter, GlobalParameterApplyMode, GlobalParameterLocation,
};

const FLAG: &str = "--telemetry";
const ENV: &str = "ELEVENLABS_TELEMETRY";

static ENABLED: OnceLock<bool> = OnceLock::new();

/// Whether telemetry is on for this process. Resolved by [`register`];
/// `false` before then, so nothing is ever sent unresolved.
pub fn enabled() -> bool {
    ENABLED.get().copied().unwrap_or(false)
}

/// `Some(true)` / `Some(false)` for a value that says on or off. Anything
/// else is `None`, which callers treat as off: whoever typed
/// `--telemetry nope` did not mean "yes".
fn parse(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "on" | "true" | "1" | "yes" => Some(true),
        "off" | "false" | "0" | "no" => Some(false),
        _ => None,
    }
}

/// The last `--telemetry` value in argv (clap's last-wins), stopping at `--`.
fn scan_argv<I: IntoIterator<Item = String>>(args: I) -> Option<String> {
    let mut found = None;
    let mut iter = args.into_iter().skip(1);
    while let Some(arg) = iter.next() {
        if arg == "--" {
            break;
        }
        if arg == FLAG {
            if let Some(value) = iter.next() {
                found = Some(value);
            }
        } else if let Some(value) = arg.strip_prefix("--telemetry=") {
            found = Some(value.to_string());
        }
    }
    found
}

/// The flag, then `ELEVENLABS_TELEMETRY`, then `DO_NOT_TRACK`
/// (<https://consoledonottrack.com>), where any value but `0` opts out.
fn decide(flag: Option<&str>, var: &dyn Fn(&str) -> Option<String>) -> bool {
    let set = |name: &str| var(name).filter(|v| !v.trim().is_empty());
    let chosen = flag
        .map(str::to_string)
        .map(|v| (FLAG, v))
        .or_else(|| set(ENV).map(|v| (ENV, v)));
    if let Some((source, value)) = chosen {
        return parse(&value).unwrap_or_else(|| {
            eprintln!("warning: {source} takes on or off; telemetry is off for this run.");
            false
        });
    }
    set("DO_NOT_TRACK").is_none_or(|v| v.trim() == "0")
}

/// `existing` minus any platform tags, with `tags` appended when given.
///
/// The inherited value is cleaned because it may come from a parent
/// `elevenlabs` (the suffix env var is inherited by every child), which
/// would otherwise double the tags, or carry them past a child's
/// `--telemetry off`.
fn compose(existing: Option<&str>, tags: Option<&str>) -> Option<String> {
    let kept = existing
        .unwrap_or_default()
        .split_whitespace()
        .filter(|token| {
            !super::platform::TAG_PREFIXES
                .iter()
                .any(|p| token.starts_with(p))
        });
    let joined = kept.chain(tags).collect::<Vec<_>>().join(" ");
    (!joined.is_empty()).then_some(joined)
}

/// Resolve the opt-out and, when telemetry is on, add the platform tags to
/// the User-Agent suffix every request reads.
fn resolve() {
    let on = decide(scan_argv(std::env::args()).as_deref(), &|n| {
        std::env::var(n).ok()
    });
    let _ = ENABLED.set(on);

    let key = super::api::suffix_env_key();
    let tags = on.then(super::platform::tags);
    match compose(std::env::var(&key).ok().as_deref(), tags.as_deref()) {
        Some(value) => std::env::set_var(&key, value),
        None => std::env::remove_var(&key),
    }
}

const TELEMETRY_HELP: &str = "on or off. Whether requests say, anonymously, which OS the CLI \
     runs on and how it was installed (npx, npm, Homebrew, ...). Overrides ELEVENLABS_TELEMETRY; \
     DO_NOT_TRACK=1 also turns it off.";

/// Resolve telemetry and register `--telemetry` so clap accepts it on every
/// command. Its value is never sent: `Explicit` with no opt-ins means the
/// framework skips it, as with `--intent`.
pub fn register(app: CliApp) -> CliApp {
    resolve();
    app.global_parameter(GlobalParameter {
        name: "telemetry".into(),
        location: GlobalParameterLocation::Header,
        target: "X-Telemetry-Unsent".into(),
        env: None,
        default: None,
        optional: true,
        apply: GlobalParameterApplyMode::Explicit,
        parameter_name: None,
        docs: Some(TELEMETRY_HELP.into()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn argv(args: &[&str]) -> Vec<String> {
        std::iter::once("elevenlabs")
            .chain(args.iter().copied())
            .map(String::from)
            .collect()
    }

    fn vars(set: &'static [(&'static str, &'static str)]) -> impl Fn(&str) -> Option<String> {
        move |name| {
            set.iter()
                .find(|(k, _)| *k == name)
                .map(|(_, v)| v.to_string())
        }
    }

    #[test]
    fn the_flag_is_read_in_both_spellings_and_last_wins() {
        assert_eq!(
            scan_argv(argv(&["onboard", "--telemetry", "off"])).as_deref(),
            Some("off")
        );
        assert_eq!(
            scan_argv(argv(&["--telemetry=off", "say", "hi"])).as_deref(),
            Some("off")
        );
        assert_eq!(
            scan_argv(argv(&["--telemetry", "off", "--telemetry=on"])).as_deref(),
            Some("on")
        );
        assert_eq!(scan_argv(argv(&["say", "--", "--telemetry", "off"])), None);
        assert_eq!(scan_argv(argv(&["say", "hi"])), None);
    }

    #[test]
    fn on_by_default() {
        assert!(decide(None, &vars(&[])));
    }

    #[test]
    fn the_flag_beats_the_env_vars() {
        assert!(!decide(Some("off"), &vars(&[])));
        assert!(decide(
            Some("on"),
            &vars(&[(ENV, "off"), ("DO_NOT_TRACK", "1")])
        ));
        assert!(!decide(Some("off"), &vars(&[(ENV, "on")])));
    }

    #[test]
    fn the_env_vars_opt_out() {
        assert!(!decide(None, &vars(&[(ENV, "off")])));
        assert!(!decide(None, &vars(&[(ENV, "FALSE")])));
        assert!(decide(None, &vars(&[(ENV, "on"), ("DO_NOT_TRACK", "1")])));
        assert!(!decide(None, &vars(&[("DO_NOT_TRACK", "1")])));
        assert!(!decide(None, &vars(&[("DO_NOT_TRACK", "true")])));
        assert!(decide(None, &vars(&[("DO_NOT_TRACK", "0")])));
        assert!(decide(None, &vars(&[(ENV, "  "), ("DO_NOT_TRACK", "")])));
    }

    #[test]
    fn a_value_that_is_neither_on_nor_off_turns_it_off() {
        assert!(!decide(Some("nope"), &vars(&[])));
        assert!(!decide(None, &vars(&[(ENV, "disabled")])));
    }

    #[test]
    fn the_tags_are_appended_to_a_consumer_token() {
        assert_eq!(
            compose(Some("partner-app/3.1"), Some("os/macos install/npx")).as_deref(),
            Some("partner-app/3.1 os/macos install/npx")
        );
        assert_eq!(
            compose(None, Some("os/linux install/npm")).as_deref(),
            Some("os/linux install/npm")
        );
    }

    #[test]
    fn inherited_tags_are_replaced_not_doubled() {
        assert_eq!(
            compose(
                Some("partner-app/3.1 os/linux env/ssh install/npm"),
                Some("os/linux env/ssh install/npm")
            )
            .as_deref(),
            Some("partner-app/3.1 os/linux env/ssh install/npm")
        );
    }

    #[test]
    fn off_strips_inherited_tags_and_keeps_the_rest() {
        assert_eq!(
            compose(Some("partner-app/3.1 os/macos install/npx"), None).as_deref(),
            Some("partner-app/3.1")
        );
        assert_eq!(compose(Some("os/macos install/npx"), None), None);
        assert_eq!(compose(None, None), None);
    }
}
