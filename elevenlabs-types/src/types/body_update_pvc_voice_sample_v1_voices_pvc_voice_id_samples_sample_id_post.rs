pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyUpdatePvcVoiceSampleV1VoicesPvcVoiceIdSamplesSampleIdPost {
    /// If set will remove background noise for voice samples using our audio isolation model. If the samples do not include background noise, it can make the quality worse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_background_noise: Option<bool>,
    /// Speaker IDs to be used for PVC training. Make sure you send all the speaker IDs you want to use for PVC training in one request because the last request will override the previous ones.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_speaker_ids: Option<Vec<String>>,
    /// The start time of the audio to be used for PVC training. Time should be in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trim_start_time: Option<i64>,
    /// The end time of the audio to be used for PVC training. Time should be in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trim_end_time: Option<i64>,
    /// The name of the audio file to be used for PVC training.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

impl BodyUpdatePvcVoiceSampleV1VoicesPvcVoiceIdSamplesSampleIdPost {
    pub fn builder() -> BodyUpdatePvcVoiceSampleV1VoicesPvcVoiceIdSamplesSampleIdPostBuilder {
        <BodyUpdatePvcVoiceSampleV1VoicesPvcVoiceIdSamplesSampleIdPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyUpdatePvcVoiceSampleV1VoicesPvcVoiceIdSamplesSampleIdPostBuilder {
    remove_background_noise: Option<bool>,
    selected_speaker_ids: Option<Vec<String>>,
    trim_start_time: Option<i64>,
    trim_end_time: Option<i64>,
    file_name: Option<String>,
}

impl BodyUpdatePvcVoiceSampleV1VoicesPvcVoiceIdSamplesSampleIdPostBuilder {
    pub fn remove_background_noise(mut self, value: bool) -> Self {
        self.remove_background_noise = Some(value);
        self
    }

    pub fn selected_speaker_ids(mut self, value: Vec<String>) -> Self {
        self.selected_speaker_ids = Some(value);
        self
    }

    pub fn trim_start_time(mut self, value: i64) -> Self {
        self.trim_start_time = Some(value);
        self
    }

    pub fn trim_end_time(mut self, value: i64) -> Self {
        self.trim_end_time = Some(value);
        self
    }

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyUpdatePvcVoiceSampleV1VoicesPvcVoiceIdSamplesSampleIdPost`].
    pub fn build(self) -> Result<BodyUpdatePvcVoiceSampleV1VoicesPvcVoiceIdSamplesSampleIdPost, BuildError> {
        Ok(BodyUpdatePvcVoiceSampleV1VoicesPvcVoiceIdSamplesSampleIdPost {
            remove_background_noise: self.remove_background_noise,
            selected_speaker_ids: self.selected_speaker_ids,
            trim_start_time: self.trim_start_time,
            trim_end_time: self.trim_end_time,
            file_name: self.file_name,
        })
    }
}

