pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubtitleOrderItemRequest {
    /// The IDs of the uploaded media files to generate subtitles for.
    #[serde(default)]
    pub media_ids: Vec<MediaId>,
    /// The language code of the source media (e.g. 'en', 'es').
    #[serde(default)]
    pub source_language: String,
    /// List of target language codes. Subtitles will be generated for each media file in each destination language.
    #[serde(default)]
    pub destination_languages: Vec<String>,
    /// Formatting options for subtitle cues such as duration, line count, and character limits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cue_options: Option<CueOptionsRequest>,
    /// Whether subtitles should use SDH format, which includes descriptions for deaf and hard-of-hearing viewers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdh: Option<bool>,
    /// Optional free-text instructions for the subtitling team.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
}

impl SubtitleOrderItemRequest {
    pub fn builder() -> SubtitleOrderItemRequestBuilder {
        <SubtitleOrderItemRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubtitleOrderItemRequestBuilder {
    media_ids: Option<Vec<MediaId>>,
    source_language: Option<String>,
    destination_languages: Option<Vec<String>>,
    cue_options: Option<CueOptionsRequest>,
    sdh: Option<bool>,
    instructions: Option<String>,
}

impl SubtitleOrderItemRequestBuilder {
    pub fn media_ids(mut self, value: Vec<MediaId>) -> Self {
        self.media_ids = Some(value);
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

    pub fn cue_options(mut self, value: CueOptionsRequest) -> Self {
        self.cue_options = Some(value);
        self
    }

    pub fn sdh(mut self, value: bool) -> Self {
        self.sdh = Some(value);
        self
    }

    pub fn instructions(mut self, value: impl Into<String>) -> Self {
        self.instructions = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubtitleOrderItemRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`media_ids`](SubtitleOrderItemRequestBuilder::media_ids)
    /// - [`source_language`](SubtitleOrderItemRequestBuilder::source_language)
    /// - [`destination_languages`](SubtitleOrderItemRequestBuilder::destination_languages)
    pub fn build(self) -> Result<SubtitleOrderItemRequest, BuildError> {
        Ok(SubtitleOrderItemRequest {
            media_ids: self.media_ids.ok_or_else(|| BuildError::missing_field("media_ids"))?,
            source_language: self.source_language.ok_or_else(|| BuildError::missing_field("source_language"))?,
            destination_languages: self.destination_languages.ok_or_else(|| BuildError::missing_field("destination_languages"))?,
            cue_options: self.cue_options,
            sdh: self.sdh,
            instructions: self.instructions,
        })
    }
}
