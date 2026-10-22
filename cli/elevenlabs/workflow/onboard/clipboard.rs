//! What the approval page left on the clipboard.

use std::time::{Duration, Instant};

/// The page's answer when it did not copy a key, written as `elevenlabs:<status>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Marker {
    /// This account can't create keys this way (the experiment is off for it).
    NotAvailable,
    /// The member's workspace role can't create keys.
    NotPermitted,
    /// No key was handed over: the page couldn't create one (a refused
    /// request, a limit, an outage), or the developer continued without it.
    Failed,
}

impl Marker {
    const ALL: [Marker; 3] = [Marker::NotAvailable, Marker::NotPermitted, Marker::Failed];

    pub(super) fn status(self) -> &'static str {
        match self {
            Marker::NotAvailable => "not_available",
            Marker::NotPermitted => "not_permitted",
            Marker::Failed => "failed",
        }
    }

    /// The exact clipboard text the page writes.
    pub(super) fn text(self) -> String {
        format!("elevenlabs:{}", self.status())
    }
}

/// A key, one of the page's markers, noise, or no answer at all.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum Clip {
    Key(String),
    Marker(Marker),
    Nothing,
    /// The clipboard couldn't be read, so a key may still be on it.
    Unreadable,
}

pub(super) fn classify(text: Option<String>) -> Clip {
    let text = text.map(|t| t.trim().to_string()).unwrap_or_default();
    if is_key(&text) {
        return Clip::Key(text);
    }
    Marker::ALL
        .into_iter()
        .find(|m| m.text() == text)
        .map_or(Clip::Nothing, Clip::Marker)
}

/// `sk_`, at least 20 hex digits, then only lowercase letters, digits and
/// underscores (residency keys carry a suffix). Other vendors' `sk_` keys
/// have uppercase or non-hex segments and are left alone.
fn is_key(text: &str) -> bool {
    let Some(rest) = text.strip_prefix("sk_") else {
        return false;
    };
    let hex = rest
        .chars()
        .take_while(|c| c.is_ascii_digit() || ('a'..='f').contains(c))
        .count();
    hex >= 20
        && rest
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

pub(super) fn clip_of(text: Result<String, arboard::Error>) -> Clip {
    match text {
        Ok(text) => classify(Some(text)),
        Err(e) if unreadable(&e) => Clip::Unreadable,
        Err(_) => Clip::Nothing,
    }
}

/// The clipboard didn't open, as opposed to holding no text (or text that
/// isn't UTF-8, which no key is).
fn unreadable(error: &arboard::Error) -> bool {
    !matches!(
        error,
        arboard::Error::ContentNotAvailable | arboard::Error::ConversionFailure
    )
}

/// How long the hand-off read keeps trying, and how often.
const HANDOFF_WAIT: Duration = Duration::from_secs(2);
const HANDOFF_POLL: Duration = Duration::from_millis(100);

/// `read` until it finds a key or a marker, or `budget` runs out.
/// `Unreadable` only when no read got through.
pub(super) fn retry_read(budget: Duration, poll: Duration, mut read: impl FnMut() -> Clip) -> Clip {
    let deadline = Instant::now() + budget;
    let mut readable = false;
    loop {
        match read() {
            Clip::Unreadable => {}
            Clip::Nothing => readable = true,
            found => return found,
        }
        if Instant::now() + poll > deadline {
            return if readable {
                Clip::Nothing
            } else {
                Clip::Unreadable
            };
        }
        std::thread::sleep(poll);
    }
}

/// Set to keep the system clipboard out of the CLI's own end-to-end tests.
pub(super) const NO_CLIPBOARD_ENV: &str = "ELEVENLABS_ONBOARD_NO_CLIPBOARD";

/// The system clipboard; `None` where there is none to read.
pub(super) struct Clipboard(Option<arboard::Clipboard>);

impl Clipboard {
    pub(super) fn open() -> Self {
        if std::env::var_os(NO_CLIPBOARD_ENV).is_some() {
            return Self(None);
        }
        Self(arboard::Clipboard::new().ok())
    }

    pub(super) fn read(&mut self) -> Clip {
        match self.0.as_mut() {
            Some(c) => clip_of(c.get_text()),
            None => Clip::Nothing,
        }
    }

    /// What the approval page copied. On Windows another program (a
    /// clipboard manager, a remote desktop) can hold the clipboard for a
    /// moment, and a read gives up after a few milliseconds; the browser's
    /// copy can also land late. So the read is tried again for a while.
    pub(super) fn read_handoff(&mut self) -> Clip {
        if self.0.is_none() {
            return Clip::Nothing;
        }
        retry_read(HANDOFF_WAIT, HANDOFF_POLL, || self.read())
    }

    /// Clear the clipboard if it still holds `value`; the user may have copied
    /// something else since. Tried again while the clipboard won't open, as
    /// the hand-off read is.
    pub(super) fn clear_if(&mut self, value: &str) {
        let Some(c) = self.0.as_mut().filter(|_| !value.is_empty()) else {
            return;
        };
        let deadline = Instant::now() + HANDOFF_WAIT;
        loop {
            let done = match c.get_text() {
                Ok(text) => text.trim() != value || c.clear().is_ok(),
                Err(e) => !unreadable(&e),
            };
            if done || Instant::now() + HANDOFF_POLL > deadline {
                return;
            }
            std::thread::sleep(HANDOFF_POLL);
        }
    }

    /// Clear a key left behind by a sign-in that did not finish. Nothing
    /// vouches for that key (no token to check it against), so it is never
    /// stored; leaving it on the clipboard would only let it leak.
    pub(super) fn discard_key(&mut self) {
        if let Clip::Key(key) = self.read() {
            self.clear_if(&key);
        }
    }
}
