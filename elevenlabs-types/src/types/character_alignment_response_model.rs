pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CharacterAlignmentResponseModel {
    #[serde(default)]
    pub characters: Vec<String>,
    #[serde(default)]
    pub character_start_times_seconds: Vec<f64>,
    #[serde(default)]
    pub character_end_times_seconds: Vec<f64>,
}

impl CharacterAlignmentResponseModel {
    pub fn builder() -> CharacterAlignmentResponseModelBuilder {
        <CharacterAlignmentResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CharacterAlignmentResponseModelBuilder {
    characters: Option<Vec<String>>,
    character_start_times_seconds: Option<Vec<f64>>,
    character_end_times_seconds: Option<Vec<f64>>,
}

impl CharacterAlignmentResponseModelBuilder {
    pub fn characters(mut self, value: Vec<String>) -> Self {
        self.characters = Some(value);
        self
    }

    pub fn character_start_times_seconds(mut self, value: Vec<f64>) -> Self {
        self.character_start_times_seconds = Some(value);
        self
    }

    pub fn character_end_times_seconds(mut self, value: Vec<f64>) -> Self {
        self.character_end_times_seconds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CharacterAlignmentResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`characters`](CharacterAlignmentResponseModelBuilder::characters)
    /// - [`character_start_times_seconds`](CharacterAlignmentResponseModelBuilder::character_start_times_seconds)
    /// - [`character_end_times_seconds`](CharacterAlignmentResponseModelBuilder::character_end_times_seconds)
    pub fn build(self) -> Result<CharacterAlignmentResponseModel, BuildError> {
        Ok(CharacterAlignmentResponseModel {
            characters: self.characters.ok_or_else(|| BuildError::missing_field("characters"))?,
            character_start_times_seconds: self.character_start_times_seconds.ok_or_else(|| BuildError::missing_field("character_start_times_seconds"))?,
            character_end_times_seconds: self.character_end_times_seconds.ok_or_else(|| BuildError::missing_field("character_end_times_seconds"))?,
        })
    }
}
