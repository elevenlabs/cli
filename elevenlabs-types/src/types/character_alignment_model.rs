pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CharacterAlignmentModel {
    #[serde(default)]
    pub characters: Vec<String>,
    #[serde(default)]
    pub character_start_times_seconds: Vec<f64>,
    #[serde(default)]
    pub character_end_times_seconds: Vec<f64>,
}

impl CharacterAlignmentModel {
    pub fn builder() -> CharacterAlignmentModelBuilder {
        <CharacterAlignmentModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CharacterAlignmentModelBuilder {
    characters: Option<Vec<String>>,
    character_start_times_seconds: Option<Vec<f64>>,
    character_end_times_seconds: Option<Vec<f64>>,
}

impl CharacterAlignmentModelBuilder {
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

    /// Consumes the builder and constructs a [`CharacterAlignmentModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`characters`](CharacterAlignmentModelBuilder::characters)
    /// - [`character_start_times_seconds`](CharacterAlignmentModelBuilder::character_start_times_seconds)
    /// - [`character_end_times_seconds`](CharacterAlignmentModelBuilder::character_end_times_seconds)
    pub fn build(self) -> Result<CharacterAlignmentModel, BuildError> {
        Ok(CharacterAlignmentModel {
            characters: self.characters.ok_or_else(|| BuildError::missing_field("characters"))?,
            character_start_times_seconds: self.character_start_times_seconds.ok_or_else(|| BuildError::missing_field("character_start_times_seconds"))?,
            character_end_times_seconds: self.character_end_times_seconds.ok_or_else(|| BuildError::missing_field("character_end_times_seconds"))?,
        })
    }
}
