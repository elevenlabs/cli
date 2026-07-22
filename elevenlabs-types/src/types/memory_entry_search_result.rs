pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MemoryEntrySearchResult {
    #[serde(default)]
    pub entry_id: String,
    #[serde(default)]
    pub version: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<MemoryEntrySearchResultSource>,
}

impl MemoryEntrySearchResult {
    pub fn builder() -> MemoryEntrySearchResultBuilder {
        <MemoryEntrySearchResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MemoryEntrySearchResultBuilder {
    entry_id: Option<String>,
    version: Option<i64>,
    summary: Option<String>,
    text: Option<String>,
    source: Option<MemoryEntrySearchResultSource>,
}

impl MemoryEntrySearchResultBuilder {
    pub fn entry_id(mut self, value: impl Into<String>) -> Self {
        self.entry_id = Some(value.into());
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
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

    pub fn source(mut self, value: MemoryEntrySearchResultSource) -> Self {
        self.source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MemoryEntrySearchResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entry_id`](MemoryEntrySearchResultBuilder::entry_id)
    /// - [`version`](MemoryEntrySearchResultBuilder::version)
    pub fn build(self) -> Result<MemoryEntrySearchResult, BuildError> {
        Ok(MemoryEntrySearchResult {
            entry_id: self.entry_id.ok_or_else(|| BuildError::missing_field("entry_id"))?,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
            summary: self.summary,
            text: self.text,
            source: self.source,
        })
    }
}
