pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BodyCreateDubbingLanguageTargetV1DubbingProjectProjectIdLanguagePost {
    /// BCP-47 language tag to dub the project into (e.g. 'fr', 'es-MX'); must be a language the dubbing model supports. A region-qualified tag must be one of the supported dialects.
    #[serde(default)]
    pub target_language: String,
    /// Voice settings applied to the whole language (e.g. cloning strength).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<VoiceSettings>,
    /// Enterprise only. Optional translations to use instead of machine translation. A map from each source segment's external_id (or its id, if you supplied none) to the translated text; every source segment must be covered exactly once. At most 20000 entries, totalling at most 4 MiB of text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations: Option<HashMap<String, Option<String>>>,
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
    voice_settings: Option<VoiceSettings>,
    translations: Option<HashMap<String, Option<String>>>,
}

impl BodyCreateDubbingLanguageTargetV1DubbingProjectProjectIdLanguagePostBuilder {
    pub fn target_language(mut self, value: impl Into<String>) -> Self {
        self.target_language = Some(value.into());
        self
    }

    pub fn voice_settings(mut self, value: VoiceSettings) -> Self {
        self.voice_settings = Some(value);
        self
    }

    pub fn translations(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.translations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyCreateDubbingLanguageTargetV1DubbingProjectProjectIdLanguagePost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`target_language`](BodyCreateDubbingLanguageTargetV1DubbingProjectProjectIdLanguagePostBuilder::target_language)
    pub fn build(self) -> Result<BodyCreateDubbingLanguageTargetV1DubbingProjectProjectIdLanguagePost, BuildError> {
        Ok(BodyCreateDubbingLanguageTargetV1DubbingProjectProjectIdLanguagePost {
            target_language: self.target_language.ok_or_else(|| BuildError::missing_field("target_language"))?,
            voice_settings: self.voice_settings,
            translations: self.translations,
        })
    }
}

