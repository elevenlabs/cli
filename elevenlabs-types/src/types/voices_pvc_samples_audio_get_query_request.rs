pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VoicesPvcSamplesAudioGetQueryRequest {
    /// If set will remove background noise for voice samples using our audio isolation model. If the samples do not include background noise, it can make the quality worse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_background_noise: Option<bool>,
}

impl VoicesPvcSamplesAudioGetQueryRequest {
    pub fn builder() -> VoicesPvcSamplesAudioGetQueryRequestBuilder {
        <VoicesPvcSamplesAudioGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoicesPvcSamplesAudioGetQueryRequestBuilder {
    remove_background_noise: Option<bool>,
}

impl VoicesPvcSamplesAudioGetQueryRequestBuilder {
    pub fn remove_background_noise(mut self, value: bool) -> Self {
        self.remove_background_noise = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VoicesPvcSamplesAudioGetQueryRequest`].
    pub fn build(self) -> Result<VoicesPvcSamplesAudioGetQueryRequest, BuildError> {
        Ok(VoicesPvcSamplesAudioGetQueryRequest {
            remove_background_noise: self.remove_background_noise,
        })
    }
}

