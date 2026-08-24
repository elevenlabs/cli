pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LoadableMemoryEntry {
    #[serde(default)]
    pub entry_id: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub text: String,
}

impl LoadableMemoryEntry {
    pub fn builder() -> LoadableMemoryEntryBuilder {
        <LoadableMemoryEntryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LoadableMemoryEntryBuilder {
    entry_id: Option<String>,
    summary: Option<String>,
    text: Option<String>,
}

impl LoadableMemoryEntryBuilder {
    pub fn entry_id(mut self, value: impl Into<String>) -> Self {
        self.entry_id = Some(value.into());
        self
    }

    pub fn summary(mut self, value: impl Into<String>) -> Self {
        self.summary = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LoadableMemoryEntry`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entry_id`](LoadableMemoryEntryBuilder::entry_id)
    /// - [`summary`](LoadableMemoryEntryBuilder::summary)
    /// - [`text`](LoadableMemoryEntryBuilder::text)
    pub fn build(self) -> Result<LoadableMemoryEntry, BuildError> {
        Ok(LoadableMemoryEntry {
            entry_id: self.entry_id.ok_or_else(|| BuildError::missing_field("entry_id"))?,
            summary: self.summary.ok_or_else(|| BuildError::missing_field("summary"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
