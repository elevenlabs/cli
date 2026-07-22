pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct HistoryAlignmentResponseModel {
    /// The characters in the alignment.
    #[serde(default)]
    pub characters: Vec<String>,
    /// The start times of the characters in seconds.
    #[serde(default)]
    pub character_start_times_seconds: Vec<f64>,
    /// The end times of the characters in seconds.
    #[serde(default)]
    pub character_end_times_seconds: Vec<f64>,
}

impl HistoryAlignmentResponseModel {
    pub fn builder() -> HistoryAlignmentResponseModelBuilder {
        <HistoryAlignmentResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HistoryAlignmentResponseModelBuilder {
    characters: Option<Vec<String>>,
    character_start_times_seconds: Option<Vec<f64>>,
    character_end_times_seconds: Option<Vec<f64>>,
}

impl HistoryAlignmentResponseModelBuilder {
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

    /// Consumes the builder and constructs a [`HistoryAlignmentResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`characters`](HistoryAlignmentResponseModelBuilder::characters)
    /// - [`character_start_times_seconds`](HistoryAlignmentResponseModelBuilder::character_start_times_seconds)
    /// - [`character_end_times_seconds`](HistoryAlignmentResponseModelBuilder::character_end_times_seconds)
    pub fn build(self) -> Result<HistoryAlignmentResponseModel, BuildError> {
        Ok(HistoryAlignmentResponseModel {
            characters: self.characters.ok_or_else(|| BuildError::missing_field("characters"))?,
            character_start_times_seconds: self.character_start_times_seconds.ok_or_else(|| BuildError::missing_field("character_start_times_seconds"))?,
            character_end_times_seconds: self.character_end_times_seconds.ok_or_else(|| BuildError::missing_field("character_end_times_seconds"))?,
        })
    }
}
