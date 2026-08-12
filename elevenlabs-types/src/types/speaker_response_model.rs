pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SpeakerResponseModel {
    /// The ID of the speaker.
    #[serde(default)]
    pub speaker_id: String,
    /// The duration of the speaker segment in seconds.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub duration_secs: f64,
    /// The utterances of the speaker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utterances: Option<Vec<UtteranceResponseModel>>,
}

impl SpeakerResponseModel {
    pub fn builder() -> SpeakerResponseModelBuilder {
        <SpeakerResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpeakerResponseModelBuilder {
    speaker_id: Option<String>,
    duration_secs: Option<f64>,
    utterances: Option<Vec<UtteranceResponseModel>>,
}

impl SpeakerResponseModelBuilder {
    pub fn speaker_id(mut self, value: impl Into<String>) -> Self {
        self.speaker_id = Some(value.into());
        self
    }

    pub fn duration_secs(mut self, value: f64) -> Self {
        self.duration_secs = Some(value);
        self
    }

    pub fn utterances(mut self, value: Vec<UtteranceResponseModel>) -> Self {
        self.utterances = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SpeakerResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`speaker_id`](SpeakerResponseModelBuilder::speaker_id)
    /// - [`duration_secs`](SpeakerResponseModelBuilder::duration_secs)
    pub fn build(self) -> Result<SpeakerResponseModel, BuildError> {
        Ok(SpeakerResponseModel {
            speaker_id: self.speaker_id.ok_or_else(|| BuildError::missing_field("speaker_id"))?,
            duration_secs: self.duration_secs.ok_or_else(|| BuildError::missing_field("duration_secs"))?,
            utterances: self.utterances,
        })
    }
}
