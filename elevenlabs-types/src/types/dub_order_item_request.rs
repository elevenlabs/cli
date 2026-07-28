pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DubOrderItemRequest {
    /// The ID of the uploaded media file to dub.
    #[serde(default)]
    pub media_id: MediaId,
    /// The language code of the source media (e.g. 'en', 'es').
    #[serde(default)]
    pub source_language: String,
    /// List of target language codes to dub the media into.
    #[serde(default)]
    pub destination_languages: Vec<String>,
    /// Whether to generate captions for the dubbed outputs.
    #[serde(default)]
    pub include_captions: bool,
    /// Whether to generate captions for the source language.
    #[serde(default)]
    pub include_source_captions: bool,
    /// Optional free-text instructions for the dubbing team.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Whether captions should use SDH format, which includes descriptions for deaf and hard-of-hearing viewers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captions_sdh: Option<bool>,
}

impl DubOrderItemRequest {
    pub fn builder() -> DubOrderItemRequestBuilder {
        <DubOrderItemRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubOrderItemRequestBuilder {
    media_id: Option<MediaId>,
    source_language: Option<String>,
    destination_languages: Option<Vec<String>>,
    include_captions: Option<bool>,
    include_source_captions: Option<bool>,
    instructions: Option<String>,
    captions_sdh: Option<bool>,
}

impl DubOrderItemRequestBuilder {
    pub fn media_id(mut self, value: MediaId) -> Self {
        self.media_id = Some(value);
        self
    }

    pub fn source_language(mut self, value: impl Into<String>) -> Self {
        self.source_language = Some(value.into());
        self
    }

    pub fn destination_languages(mut self, value: Vec<String>) -> Self {
        self.destination_languages = Some(value);
        self
    }

    pub fn include_captions(mut self, value: bool) -> Self {
        self.include_captions = Some(value);
        self
    }

    pub fn include_source_captions(mut self, value: bool) -> Self {
        self.include_source_captions = Some(value);
        self
    }

    pub fn instructions(mut self, value: impl Into<String>) -> Self {
        self.instructions = Some(value.into());
        self
    }

    pub fn captions_sdh(mut self, value: bool) -> Self {
        self.captions_sdh = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubOrderItemRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`media_id`](DubOrderItemRequestBuilder::media_id)
    /// - [`source_language`](DubOrderItemRequestBuilder::source_language)
    /// - [`destination_languages`](DubOrderItemRequestBuilder::destination_languages)
    /// - [`include_captions`](DubOrderItemRequestBuilder::include_captions)
    /// - [`include_source_captions`](DubOrderItemRequestBuilder::include_source_captions)
    pub fn build(self) -> Result<DubOrderItemRequest, BuildError> {
        Ok(DubOrderItemRequest {
            media_id: self.media_id.ok_or_else(|| BuildError::missing_field("media_id"))?,
            source_language: self.source_language.ok_or_else(|| BuildError::missing_field("source_language"))?,
            destination_languages: self.destination_languages.ok_or_else(|| BuildError::missing_field("destination_languages"))?,
            include_captions: self.include_captions.ok_or_else(|| BuildError::missing_field("include_captions"))?,
            include_source_captions: self.include_source_captions.ok_or_else(|| BuildError::missing_field("include_source_captions"))?,
            instructions: self.instructions,
            captions_sdh: self.captions_sdh,
        })
    }
}
