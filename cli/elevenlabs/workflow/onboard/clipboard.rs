//! What the approval page left on the clipboard, and what the skill does
//! about each outcome.

/// A key, one of the page's `elevenlabs:<status>` markers, or noise.
pub(super) enum Clip {
    Key(String),
    Marker(&'static str),
    Nothing,
}

pub(super) fn classify(text: Option<String>) -> Clip {
    let text = text.map(|t| t.trim().to_string()).unwrap_or_default();
    if is_key(&text) {
        return Clip::Key(text);
    }
    match text.as_str() {
        "elevenlabs:not_available" => Clip::Marker("not_available"),
        "elevenlabs:not_permitted" => Clip::Marker("not_permitted"),
        // The page could not create a key (a refused request, a limit, an
        // outage): the same legacy path as a flag that is off.
        "elevenlabs:failed" => Clip::Marker("failed"),
        _ => Clip::Nothing,
    }
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

/// The skill's next step for a `key_status`.
pub(super) fn next_for(key_status: &str) -> &'static str {
    match key_status {
        "created" => "none",
        "not_available" | "not_permitted" | "failed" => "legacy",
        _ => "paste_key",
    }
}

/// The raw clipboard text that stands for `marker`.
pub(super) fn marker_text(marker: &str) -> String {
    format!("elevenlabs:{marker}")
}

/// The system clipboard; `None` where there is none to read.
pub(super) struct Clipboard(Option<arboard::Clipboard>);

impl Clipboard {
    pub(super) fn open() -> Self {
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
    pub(super) fn discard_key(&mut self) -> bool {
        match self.read() {
            Clip::Key(key) => {
                self.clear_if(&key);
                true
            }
            _ => false,
        }
    }
}
