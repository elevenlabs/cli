pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FlattenedConfigOverrides {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
}

impl FlattenedConfigOverrides {
    pub fn builder() -> FlattenedConfigOverridesBuilder {
        <FlattenedConfigOverridesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FlattenedConfigOverridesBuilder {
    language: Option<String>,
    first_message: Option<String>,
    prompt: Option<String>,
    voice_id: Option<String>,
}

impl FlattenedConfigOverridesBuilder {
    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn first_message(mut self, value: impl Into<String>) -> Self {
        self.first_message = Some(value.into());
        self
    }

    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FlattenedConfigOverrides`].
    pub fn build(self) -> Result<FlattenedConfigOverrides, BuildError> {
        Ok(FlattenedConfigOverrides {
            language: self.language,
            first_message: self.first_message,
            prompt: self.prompt,
            voice_id: self.voice_id,
        })
    }
}
