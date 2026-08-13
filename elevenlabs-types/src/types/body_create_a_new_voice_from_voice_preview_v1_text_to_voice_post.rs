pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BodyCreateANewVoiceFromVoicePreviewV1TextToVoicePost {
    /// Name to use for the created voice.
    #[serde(default)]
    pub voice_name: String,
    /// Description to use for the created voice.
    #[serde(default)]
    pub voice_description: String,
    /// The generated_voice_id to create; obtain it from POST /v1/text-to-voice/design, POST /v1/text-to-voice/:voice_id/remix, or the response headers when generating previews.
    #[serde(default)]
    pub generated_voice_id: String,
    /// Optional, metadata to add to the created voice. Defaults to None.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, Option<String>>>,
    /// List of voice ids that the user has played but not selected. Used for RLHF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub played_not_selected_voice_ids: Option<Vec<String>>,
}

impl BodyCreateANewVoiceFromVoicePreviewV1TextToVoicePost {
    pub fn builder() -> BodyCreateANewVoiceFromVoicePreviewV1TextToVoicePostBuilder {
        <BodyCreateANewVoiceFromVoicePreviewV1TextToVoicePostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyCreateANewVoiceFromVoicePreviewV1TextToVoicePostBuilder {
    voice_name: Option<String>,
    voice_description: Option<String>,
    generated_voice_id: Option<String>,
    labels: Option<HashMap<String, Option<String>>>,
    played_not_selected_voice_ids: Option<Vec<String>>,
}

impl BodyCreateANewVoiceFromVoicePreviewV1TextToVoicePostBuilder {
    pub fn voice_name(mut self, value: impl Into<String>) -> Self {
        self.voice_name = Some(value.into());
        self
    }

    pub fn voice_description(mut self, value: impl Into<String>) -> Self {
        self.voice_description = Some(value.into());
        self
    }

    pub fn generated_voice_id(mut self, value: impl Into<String>) -> Self {
        self.generated_voice_id = Some(value.into());
        self
    }

    pub fn labels(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.labels = Some(value);
        self
    }

    pub fn played_not_selected_voice_ids(mut self, value: Vec<String>) -> Self {
        self.played_not_selected_voice_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyCreateANewVoiceFromVoicePreviewV1TextToVoicePost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`voice_name`](BodyCreateANewVoiceFromVoicePreviewV1TextToVoicePostBuilder::voice_name)
    /// - [`voice_description`](BodyCreateANewVoiceFromVoicePreviewV1TextToVoicePostBuilder::voice_description)
    /// - [`generated_voice_id`](BodyCreateANewVoiceFromVoicePreviewV1TextToVoicePostBuilder::generated_voice_id)
    pub fn build(self) -> Result<BodyCreateANewVoiceFromVoicePreviewV1TextToVoicePost, BuildError> {
        Ok(BodyCreateANewVoiceFromVoicePreviewV1TextToVoicePost {
            voice_name: self.voice_name.ok_or_else(|| BuildError::missing_field("voice_name"))?,
            voice_description: self.voice_description.ok_or_else(|| BuildError::missing_field("voice_description"))?,
            generated_voice_id: self.generated_voice_id.ok_or_else(|| BuildError::missing_field("generated_voice_id"))?,
            labels: self.labels,
            played_not_selected_voice_ids: self.played_not_selected_voice_ids,
        })
    }
}

