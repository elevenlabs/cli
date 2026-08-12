pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WordTimestamp {
    #[serde(default)]
    pub word: String,
    #[serde(default)]
    pub start_ms: i64,
    #[serde(default)]
    pub end_ms: i64,
}

impl WordTimestamp {
    pub fn builder() -> WordTimestampBuilder {
        <WordTimestampBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WordTimestampBuilder {
    word: Option<String>,
    start_ms: Option<i64>,
    end_ms: Option<i64>,
}

impl WordTimestampBuilder {
    pub fn word(mut self, value: impl Into<String>) -> Self {
        self.word = Some(value.into());
        self
    }

    pub fn start_ms(mut self, value: i64) -> Self {
        self.start_ms = Some(value);
        self
    }

    pub fn end_ms(mut self, value: i64) -> Self {
        self.end_ms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WordTimestamp`].
    /// This method will fail if any of the following fields are not set:
    /// - [`word`](WordTimestampBuilder::word)
    /// - [`start_ms`](WordTimestampBuilder::start_ms)
    /// - [`end_ms`](WordTimestampBuilder::end_ms)
    pub fn build(self) -> Result<WordTimestamp, BuildError> {
        Ok(WordTimestamp {
            word: self.word.ok_or_else(|| BuildError::missing_field("word"))?,
            start_ms: self.start_ms.ok_or_else(|| BuildError::missing_field("start_ms"))?,
            end_ms: self.end_ms.ok_or_else(|| BuildError::missing_field("end_ms"))?,
        })
    }
}
