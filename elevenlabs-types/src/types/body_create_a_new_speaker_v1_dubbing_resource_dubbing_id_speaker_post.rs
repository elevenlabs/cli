pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BodyCreateANewSpeakerV1DubbingResourceDubbingIdSpeakerPost {
    /// Name to attribute to this speaker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker_name: Option<String>,
    /// Either the identifier of a voice from the ElevenLabs voice library, or one of ['track-clone', 'clip-clone'].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
    /// For models that support it, the voice similarity value to use. This will default to 0.65, with a valid range of [0.0, 1.0].
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub voice_stability: Option<f64>,
    /// For models that support it, the voice similarity value to use. This will default to 1.0, with a valid range of [0.0, 1.0].
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub voice_similarity: Option<f64>,
    /// For models that support it, the voice style value to use. This will default to 1.0, with a valid range of [0.0, 1.0].
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub voice_style: Option<f64>,
}

impl BodyCreateANewSpeakerV1DubbingResourceDubbingIdSpeakerPost {
    pub fn builder() -> BodyCreateANewSpeakerV1DubbingResourceDubbingIdSpeakerPostBuilder {
        <BodyCreateANewSpeakerV1DubbingResourceDubbingIdSpeakerPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyCreateANewSpeakerV1DubbingResourceDubbingIdSpeakerPostBuilder {
    speaker_name: Option<String>,
    voice_id: Option<String>,
    voice_stability: Option<f64>,
    voice_similarity: Option<f64>,
    voice_style: Option<f64>,
}

impl BodyCreateANewSpeakerV1DubbingResourceDubbingIdSpeakerPostBuilder {
    pub fn speaker_name(mut self, value: impl Into<String>) -> Self {
        self.speaker_name = Some(value.into());
        self
    }

    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    pub fn voice_stability(mut self, value: f64) -> Self {
        self.voice_stability = Some(value);
        self
    }

    pub fn voice_similarity(mut self, value: f64) -> Self {
        self.voice_similarity = Some(value);
        self
    }

    pub fn voice_style(mut self, value: f64) -> Self {
        self.voice_style = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyCreateANewSpeakerV1DubbingResourceDubbingIdSpeakerPost`].
    pub fn build(self) -> Result<BodyCreateANewSpeakerV1DubbingResourceDubbingIdSpeakerPost, BuildError> {
        Ok(BodyCreateANewSpeakerV1DubbingResourceDubbingIdSpeakerPost {
            speaker_name: self.speaker_name,
            voice_id: self.voice_id,
            voice_stability: self.voice_stability,
            voice_similarity: self.voice_similarity,
            voice_style: self.voice_style,
        })
    }
}

