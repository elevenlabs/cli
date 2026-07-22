pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BodyCreateDubbingLanguageTargetV1DubbingProjectProjectIdLanguagePost {
    /// BCP-47 language tag to dub the project into (e.g. 'fr', 'es-419').
    #[serde(default)]
    pub target_language: String,
    /// Dubbing model id for this target; omit to use the project default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    /// Voice settings applied to the whole language (e.g. cloning strength).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<VoiceSettings>,
}

impl BodyCreateDubbingLanguageTargetV1DubbingProjectProjectIdLanguagePost {
    pub fn builder() -> BodyCreateDubbingLanguageTargetV1DubbingProjectProjectIdLanguagePostBuilder {
        <BodyCreateDubbingLanguageTargetV1DubbingProjectProjectIdLanguagePostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyCreateDubbingLanguageTargetV1DubbingProjectProjectIdLanguagePostBuilder {
    target_language: Option<String>,
    model_id: Option<String>,
    voice_settings: Option<VoiceSettings>,
}

impl BodyCreateDubbingLanguageTargetV1DubbingProjectProjectIdLanguagePostBuilder {
    pub fn target_language(mut self, value: impl Into<String>) -> Self {
        self.target_language = Some(value.into());
        self
    }

    pub fn model_id(mut self, value: impl Into<String>) -> Self {
        self.model_id = Some(value.into());
        self
    }

    pub fn voice_settings(mut self, value: VoiceSettings) -> Self {
        self.voice_settings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyCreateDubbingLanguageTargetV1DubbingProjectProjectIdLanguagePost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`target_language`](BodyCreateDubbingLanguageTargetV1DubbingProjectProjectIdLanguagePostBuilder::target_language)
    pub fn build(self) -> Result<BodyCreateDubbingLanguageTargetV1DubbingProjectProjectIdLanguagePost, BuildError> {
        Ok(BodyCreateDubbingLanguageTargetV1DubbingProjectProjectIdLanguagePost {
            target_language: self.target_language.ok_or_else(|| BuildError::missing_field("target_language"))?,
            model_id: self.model_id,
            voice_settings: self.voice_settings,
        })
    }
}

