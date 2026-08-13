pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Character-level alignment data (field names use snake_case in JSON).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DialogueTextAlignment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chars: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub char_start_times_ms: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub char_durations_ms: Option<Vec<i64>>,
}

impl DialogueTextAlignment {
    pub fn builder() -> DialogueTextAlignmentBuilder {
        <DialogueTextAlignmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DialogueTextAlignmentBuilder {
    chars: Option<Vec<String>>,
    char_start_times_ms: Option<Vec<i64>>,
    char_durations_ms: Option<Vec<i64>>,
}

impl DialogueTextAlignmentBuilder {
    pub fn chars(mut self, value: Vec<String>) -> Self {
        self.chars = Some(value);
        self
    }

    pub fn char_start_times_ms(mut self, value: Vec<i64>) -> Self {
        self.char_start_times_ms = Some(value);
        self
    }

    pub fn char_durations_ms(mut self, value: Vec<i64>) -> Self {
        self.char_durations_ms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DialogueTextAlignment`].
    pub fn build(self) -> Result<DialogueTextAlignment, BuildError> {
        Ok(DialogueTextAlignment {
            chars: self.chars,
            char_start_times_ms: self.char_start_times_ms,
            char_durations_ms: self.char_durations_ms,
        })
    }
}
