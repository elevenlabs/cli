pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SimilarVoice {
    #[serde(default)]
    pub voice_id: String,
    #[serde(default)]
    pub name: String,
    pub category: VoiceCategory,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_url: Option<String>,
}

impl SimilarVoice {
    pub fn builder() -> SimilarVoiceBuilder {
        <SimilarVoiceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SimilarVoiceBuilder {
    voice_id: Option<String>,
    name: Option<String>,
    category: Option<VoiceCategory>,
    description: Option<String>,
    preview_url: Option<String>,
}

impl SimilarVoiceBuilder {
    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn category(mut self, value: VoiceCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn preview_url(mut self, value: impl Into<String>) -> Self {
        self.preview_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SimilarVoice`].
    /// This method will fail if any of the following fields are not set:
    /// - [`voice_id`](SimilarVoiceBuilder::voice_id)
    /// - [`name`](SimilarVoiceBuilder::name)
    /// - [`category`](SimilarVoiceBuilder::category)
    pub fn build(self) -> Result<SimilarVoice, BuildError> {
        Ok(SimilarVoice {
            voice_id: self.voice_id.ok_or_else(|| BuildError::missing_field("voice_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            category: self.category.ok_or_else(|| BuildError::missing_field("category"))?,
            description: self.description,
            preview_url: self.preview_url,
        })
    }
}
