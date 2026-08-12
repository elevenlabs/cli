pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DubbingMetadataResponse {
    /// The ID of the dubbing project.
    #[serde(default)]
    pub dubbing_id: String,
    /// The name of the dubbing project.
    #[serde(default)]
    pub name: String,
    /// The state this dub is in.
    #[serde(default)]
    pub status: String,
    /// Once dubbing has completed, the ISO-639-1 code of the original media's source language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_language: Option<String>,
    /// The ISO-639-1 code of the languages this media has been dubbed into.
    #[serde(default)]
    pub target_languages: Vec<String>,
    /// Whether this dubbing project is editable in Dubbing Studio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    /// Timestamp this dub was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Metadata, such as the length in seconds and content type, of the dubbed content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_metadata: Option<DubbingMediaMetadata>,
    /// Error message indicate, if this dub has failed, what happened.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl DubbingMetadataResponse {
    pub fn builder() -> DubbingMetadataResponseBuilder {
        <DubbingMetadataResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingMetadataResponseBuilder {
    dubbing_id: Option<String>,
    name: Option<String>,
    status: Option<String>,
    source_language: Option<String>,
    target_languages: Option<Vec<String>>,
    editable: Option<bool>,
    created_at: Option<DateTime<FixedOffset>>,
    media_metadata: Option<DubbingMediaMetadata>,
    error: Option<String>,
}

impl DubbingMetadataResponseBuilder {
    pub fn dubbing_id(mut self, value: impl Into<String>) -> Self {
        self.dubbing_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn source_language(mut self, value: impl Into<String>) -> Self {
        self.source_language = Some(value.into());
        self
    }

    pub fn target_languages(mut self, value: Vec<String>) -> Self {
        self.target_languages = Some(value);
        self
    }

    pub fn editable(mut self, value: bool) -> Self {
        self.editable = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn media_metadata(mut self, value: DubbingMediaMetadata) -> Self {
        self.media_metadata = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DubbingMetadataResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dubbing_id`](DubbingMetadataResponseBuilder::dubbing_id)
    /// - [`name`](DubbingMetadataResponseBuilder::name)
    /// - [`status`](DubbingMetadataResponseBuilder::status)
    /// - [`target_languages`](DubbingMetadataResponseBuilder::target_languages)
    /// - [`created_at`](DubbingMetadataResponseBuilder::created_at)
    pub fn build(self) -> Result<DubbingMetadataResponse, BuildError> {
        Ok(DubbingMetadataResponse {
            dubbing_id: self.dubbing_id.ok_or_else(|| BuildError::missing_field("dubbing_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            source_language: self.source_language,
            target_languages: self.target_languages.ok_or_else(|| BuildError::missing_field("target_languages"))?,
            editable: self.editable,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            media_metadata: self.media_metadata,
            error: self.error,
        })
    }
}
