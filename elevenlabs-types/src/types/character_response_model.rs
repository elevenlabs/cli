pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CharacterResponseModel {
    #[serde(default)]
    pub character_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<CharacterMetadataResponseModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_voice_ids: Option<Vec<String>>,
}

impl CharacterResponseModel {
    pub fn builder() -> CharacterResponseModelBuilder {
        <CharacterResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CharacterResponseModelBuilder {
    character_id: Option<String>,
    name: Option<String>,
    metadata: Option<CharacterMetadataResponseModel>,
    recommended_voice_ids: Option<Vec<String>>,
}

impl CharacterResponseModelBuilder {
    pub fn character_id(mut self, value: impl Into<String>) -> Self {
        self.character_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: CharacterMetadataResponseModel) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn recommended_voice_ids(mut self, value: Vec<String>) -> Self {
        self.recommended_voice_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CharacterResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`character_id`](CharacterResponseModelBuilder::character_id)
    /// - [`name`](CharacterResponseModelBuilder::name)
    pub fn build(self) -> Result<CharacterResponseModel, BuildError> {
        Ok(CharacterResponseModel {
            character_id: self.character_id.ok_or_else(|| BuildError::missing_field("character_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            metadata: self.metadata,
            recommended_voice_ids: self.recommended_voice_ids,
        })
    }
}
