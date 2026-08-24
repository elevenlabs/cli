pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetLibraryVoicesResponse {
    /// The list of shared voices
    #[serde(default)]
    pub voices: Vec<LibraryVoiceResponse>,
    /// Whether there are more shared voices in subsequent pages.
    #[serde(default)]
    pub has_more: bool,
    /// The total number of shared voices matching the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sort_id: Option<String>,
}

impl GetLibraryVoicesResponse {
    pub fn builder() -> GetLibraryVoicesResponseBuilder {
        <GetLibraryVoicesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetLibraryVoicesResponseBuilder {
    voices: Option<Vec<LibraryVoiceResponse>>,
    has_more: Option<bool>,
    total_count: Option<i64>,
    last_sort_id: Option<String>,
}

impl GetLibraryVoicesResponseBuilder {
    pub fn voices(mut self, value: Vec<LibraryVoiceResponse>) -> Self {
        self.voices = Some(value);
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn total_count(mut self, value: i64) -> Self {
        self.total_count = Some(value);
        self
    }

    pub fn last_sort_id(mut self, value: impl Into<String>) -> Self {
        self.last_sort_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetLibraryVoicesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`voices`](GetLibraryVoicesResponseBuilder::voices)
    /// - [`has_more`](GetLibraryVoicesResponseBuilder::has_more)
    pub fn build(self) -> Result<GetLibraryVoicesResponse, BuildError> {
        Ok(GetLibraryVoicesResponse {
            voices: self.voices.ok_or_else(|| BuildError::missing_field("voices"))?,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
            total_count: self.total_count,
            last_sort_id: self.last_sort_id,
        })
    }
}
