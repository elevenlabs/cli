pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyRunPvcTrainingV1VoicesPvcVoiceIdTrainPost {
    /// The model ID to use for the conversion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
}

impl BodyRunPvcTrainingV1VoicesPvcVoiceIdTrainPost {
    pub fn builder() -> BodyRunPvcTrainingV1VoicesPvcVoiceIdTrainPostBuilder {
        <BodyRunPvcTrainingV1VoicesPvcVoiceIdTrainPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyRunPvcTrainingV1VoicesPvcVoiceIdTrainPostBuilder {
    model_id: Option<String>,
}

impl BodyRunPvcTrainingV1VoicesPvcVoiceIdTrainPostBuilder {
    pub fn model_id(mut self, value: impl Into<String>) -> Self {
        self.model_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyRunPvcTrainingV1VoicesPvcVoiceIdTrainPost`].
    pub fn build(self) -> Result<BodyRunPvcTrainingV1VoicesPvcVoiceIdTrainPost, BuildError> {
        Ok(BodyRunPvcTrainingV1VoicesPvcVoiceIdTrainPost {
            model_id: self.model_id,
        })
    }
}

