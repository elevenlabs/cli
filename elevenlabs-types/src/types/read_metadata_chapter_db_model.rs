pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReadMetadataChapterDbModel {
    #[serde(default)]
    pub chapter_name: String,
    #[serde(default)]
    pub word_count: i64,
    #[serde(default)]
    pub char_count: i64,
    #[serde(default)]
    pub starting_char_offset: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_parsed_html: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_summary: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub duration_seconds: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_fallback_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter_id: Option<String>,
}

impl ReadMetadataChapterDbModel {
    pub fn builder() -> ReadMetadataChapterDbModelBuilder {
        <ReadMetadataChapterDbModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReadMetadataChapterDbModelBuilder {
    chapter_name: Option<String>,
    word_count: Option<i64>,
    char_count: Option<i64>,
    starting_char_offset: Option<i64>,
    has_parsed_html: Option<bool>,
    has_summary: Option<bool>,
    duration_seconds: Option<f64>,
    file_number: Option<String>,
    is_fallback_name: Option<bool>,
    chapter_id: Option<String>,
}

impl ReadMetadataChapterDbModelBuilder {
    pub fn chapter_name(mut self, value: impl Into<String>) -> Self {
        self.chapter_name = Some(value.into());
        self
    }

    pub fn word_count(mut self, value: i64) -> Self {
        self.word_count = Some(value);
        self
    }

    pub fn char_count(mut self, value: i64) -> Self {
        self.char_count = Some(value);
        self
    }

    pub fn starting_char_offset(mut self, value: i64) -> Self {
        self.starting_char_offset = Some(value);
        self
    }

    pub fn has_parsed_html(mut self, value: bool) -> Self {
        self.has_parsed_html = Some(value);
        self
    }

    pub fn has_summary(mut self, value: bool) -> Self {
        self.has_summary = Some(value);
        self
    }

    pub fn duration_seconds(mut self, value: f64) -> Self {
        self.duration_seconds = Some(value);
        self
    }

    pub fn file_number(mut self, value: impl Into<String>) -> Self {
        self.file_number = Some(value.into());
        self
    }

    pub fn is_fallback_name(mut self, value: bool) -> Self {
        self.is_fallback_name = Some(value);
        self
    }

    pub fn chapter_id(mut self, value: impl Into<String>) -> Self {
        self.chapter_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReadMetadataChapterDbModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chapter_name`](ReadMetadataChapterDbModelBuilder::chapter_name)
    /// - [`word_count`](ReadMetadataChapterDbModelBuilder::word_count)
    /// - [`char_count`](ReadMetadataChapterDbModelBuilder::char_count)
    /// - [`starting_char_offset`](ReadMetadataChapterDbModelBuilder::starting_char_offset)
    pub fn build(self) -> Result<ReadMetadataChapterDbModel, BuildError> {
        Ok(ReadMetadataChapterDbModel {
            chapter_name: self.chapter_name.ok_or_else(|| BuildError::missing_field("chapter_name"))?,
            word_count: self.word_count.ok_or_else(|| BuildError::missing_field("word_count"))?,
            char_count: self.char_count.ok_or_else(|| BuildError::missing_field("char_count"))?,
            starting_char_offset: self.starting_char_offset.ok_or_else(|| BuildError::missing_field("starting_char_offset"))?,
            has_parsed_html: self.has_parsed_html,
            has_summary: self.has_summary,
            duration_seconds: self.duration_seconds,
            file_number: self.file_number,
            is_fallback_name: self.is_fallback_name,
            chapter_id: self.chapter_id,
        })
    }
}
