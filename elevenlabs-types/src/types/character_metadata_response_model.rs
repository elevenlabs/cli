pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CharacterMetadataResponseModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_creation_prompt_suggestion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<CharacterGender>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<CharacterAge>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<CharacterRole>,
}

impl CharacterMetadataResponseModel {
    pub fn builder() -> CharacterMetadataResponseModelBuilder {
        <CharacterMetadataResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CharacterMetadataResponseModelBuilder {
    description: Option<String>,
    sample_message: Option<String>,
    voice_creation_prompt_suggestion: Option<String>,
    gender: Option<CharacterGender>,
    age: Option<CharacterAge>,
    accent: Option<String>,
    role: Option<CharacterRole>,
}

impl CharacterMetadataResponseModelBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn sample_message(mut self, value: impl Into<String>) -> Self {
        self.sample_message = Some(value.into());
        self
    }

    pub fn voice_creation_prompt_suggestion(mut self, value: impl Into<String>) -> Self {
        self.voice_creation_prompt_suggestion = Some(value.into());
        self
    }

    pub fn gender(mut self, value: CharacterGender) -> Self {
        self.gender = Some(value);
        self
    }

    pub fn age(mut self, value: CharacterAge) -> Self {
        self.age = Some(value);
        self
    }

    pub fn accent(mut self, value: impl Into<String>) -> Self {
        self.accent = Some(value.into());
        self
    }

    pub fn role(mut self, value: CharacterRole) -> Self {
        self.role = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CharacterMetadataResponseModel`].
    pub fn build(self) -> Result<CharacterMetadataResponseModel, BuildError> {
        Ok(CharacterMetadataResponseModel {
            description: self.description,
            sample_message: self.sample_message,
            voice_creation_prompt_suggestion: self.voice_creation_prompt_suggestion,
            gender: self.gender,
            age: self.age,
            accent: self.accent,
            role: self.role,
        })
    }
}
