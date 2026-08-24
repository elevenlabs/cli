pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpeakerSeparationResponseModel {
    /// The ID of the voice.
    #[serde(default)]
    pub voice_id: String,
    /// The ID of the sample.
    #[serde(default)]
    pub sample_id: String,
    /// The status of the speaker separation.
    pub status: SpeakerSeparationResponseModelStatus,
    /// The speakers of the sample.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speakers: Option<HashMap<String, Option<SpeakerResponseModel>>>,
    /// The IDs of the selected speakers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_speaker_ids: Option<Vec<String>>,
}

impl SpeakerSeparationResponseModel {
    pub fn builder() -> SpeakerSeparationResponseModelBuilder {
        <SpeakerSeparationResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpeakerSeparationResponseModelBuilder {
    voice_id: Option<String>,
    sample_id: Option<String>,
    status: Option<SpeakerSeparationResponseModelStatus>,
    speakers: Option<HashMap<String, Option<SpeakerResponseModel>>>,
    selected_speaker_ids: Option<Vec<String>>,
}

impl SpeakerSeparationResponseModelBuilder {
    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    pub fn sample_id(mut self, value: impl Into<String>) -> Self {
        self.sample_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: SpeakerSeparationResponseModelStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn speakers(mut self, value: HashMap<String, Option<SpeakerResponseModel>>) -> Self {
        self.speakers = Some(value);
        self
    }

    pub fn selected_speaker_ids(mut self, value: Vec<String>) -> Self {
        self.selected_speaker_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SpeakerSeparationResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`voice_id`](SpeakerSeparationResponseModelBuilder::voice_id)
    /// - [`sample_id`](SpeakerSeparationResponseModelBuilder::sample_id)
    /// - [`status`](SpeakerSeparationResponseModelBuilder::status)
    pub fn build(self) -> Result<SpeakerSeparationResponseModel, BuildError> {
        Ok(SpeakerSeparationResponseModel {
            voice_id: self.voice_id.ok_or_else(|| BuildError::missing_field("voice_id"))?,
            sample_id: self.sample_id.ok_or_else(|| BuildError::missing_field("sample_id"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            speakers: self.speakers,
            selected_speaker_ids: self.selected_speaker_ids,
        })
    }
}
