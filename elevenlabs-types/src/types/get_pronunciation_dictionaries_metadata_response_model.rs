pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetPronunciationDictionariesMetadataResponseModel {
    /// A list of pronunciation dictionaries and their metadata.
    #[serde(default)]
    pub pronunciation_dictionaries: Vec<GetPronunciationDictionaryMetadataResponse>,
    /// The next cursor to use for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Whether there are more pronunciation dictionaries to fetch.
    #[serde(default)]
    pub has_more: bool,
}

impl GetPronunciationDictionariesMetadataResponseModel {
    pub fn builder() -> GetPronunciationDictionariesMetadataResponseModelBuilder {
        <GetPronunciationDictionariesMetadataResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPronunciationDictionariesMetadataResponseModelBuilder {
    pronunciation_dictionaries: Option<Vec<GetPronunciationDictionaryMetadataResponse>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl GetPronunciationDictionariesMetadataResponseModelBuilder {
    pub fn pronunciation_dictionaries(mut self, value: Vec<GetPronunciationDictionaryMetadataResponse>) -> Self {
        self.pronunciation_dictionaries = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetPronunciationDictionariesMetadataResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`pronunciation_dictionaries`](GetPronunciationDictionariesMetadataResponseModelBuilder::pronunciation_dictionaries)
    /// - [`has_more`](GetPronunciationDictionariesMetadataResponseModelBuilder::has_more)
    pub fn build(self) -> Result<GetPronunciationDictionariesMetadataResponseModel, BuildError> {
        Ok(GetPronunciationDictionariesMetadataResponseModel {
            pronunciation_dictionaries: self.pronunciation_dictionaries.ok_or_else(|| BuildError::missing_field("pronunciation_dictionaries"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
        })
    }
}
