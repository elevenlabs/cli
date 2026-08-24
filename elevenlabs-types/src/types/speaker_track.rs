pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SpeakerTrack {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub media_ref: DubbingMediaReference,
    #[serde(default)]
    pub speaker_name: String,
    #[serde(default)]
    pub voices: HashMap<String, String>,
    #[serde(default)]
    pub segments: Vec<String>,
}

impl SpeakerTrack {
    pub fn builder() -> SpeakerTrackBuilder {
        <SpeakerTrackBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpeakerTrackBuilder {
    id: Option<String>,
    media_ref: Option<DubbingMediaReference>,
    speaker_name: Option<String>,
    voices: Option<HashMap<String, String>>,
    segments: Option<Vec<String>>,
}

impl SpeakerTrackBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn media_ref(mut self, value: DubbingMediaReference) -> Self {
        self.media_ref = Some(value);
        self
    }

    pub fn speaker_name(mut self, value: impl Into<String>) -> Self {
        self.speaker_name = Some(value.into());
        self
    }

    pub fn voices(mut self, value: HashMap<String, String>) -> Self {
        self.voices = Some(value);
        self
    }

    pub fn segments(mut self, value: Vec<String>) -> Self {
        self.segments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SpeakerTrack`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SpeakerTrackBuilder::id)
    /// - [`media_ref`](SpeakerTrackBuilder::media_ref)
    /// - [`speaker_name`](SpeakerTrackBuilder::speaker_name)
    /// - [`voices`](SpeakerTrackBuilder::voices)
    /// - [`segments`](SpeakerTrackBuilder::segments)
    pub fn build(self) -> Result<SpeakerTrack, BuildError> {
        Ok(SpeakerTrack {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            media_ref: self.media_ref.ok_or_else(|| BuildError::missing_field("media_ref"))?,
            speaker_name: self.speaker_name.ok_or_else(|| BuildError::missing_field("speaker_name"))?,
            voices: self.voices.ok_or_else(|| BuildError::missing_field("voices"))?,
            segments: self.segments.ok_or_else(|| BuildError::missing_field("segments"))?,
        })
    }
}
