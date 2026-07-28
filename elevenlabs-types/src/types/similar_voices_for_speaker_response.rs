pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SimilarVoicesForSpeakerResponse {
    #[serde(default)]
    pub voices: Vec<SimilarVoice>,
}

impl SimilarVoicesForSpeakerResponse {
    pub fn builder() -> SimilarVoicesForSpeakerResponseBuilder {
        <SimilarVoicesForSpeakerResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SimilarVoicesForSpeakerResponseBuilder {
    voices: Option<Vec<SimilarVoice>>,
}

impl SimilarVoicesForSpeakerResponseBuilder {
    pub fn voices(mut self, value: Vec<SimilarVoice>) -> Self {
        self.voices = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SimilarVoicesForSpeakerResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`voices`](SimilarVoicesForSpeakerResponseBuilder::voices)
    pub fn build(self) -> Result<SimilarVoicesForSpeakerResponse, BuildError> {
        Ok(SimilarVoicesForSpeakerResponse {
            voices: self.voices.ok_or_else(|| BuildError::missing_field("voices"))?,
        })
    }
}
