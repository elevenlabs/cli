//! What the approval page left on the clipboard.

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

/// A key, one of the page's markers, or noise.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum Clip {
    Key(String),
    Marker(Marker),
    Nothing,
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
        classify(self.0.as_mut().and_then(|c| c.get_text().ok()))
    }

    /// Clear the clipboard if it still holds `value`; the user may have copied
    /// something else since.
    pub(super) fn clear_if(&mut self, value: &str) {
        if let Some(c) = self.0.as_mut() {
            if c.get_text().is_ok_and(|t| t.trim() == value) {
                let _ = c.clear();
            }
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
