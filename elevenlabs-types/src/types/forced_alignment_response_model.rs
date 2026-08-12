pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Model representing the response from the aligner service.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ForcedAlignmentResponseModel {
    /// List of characters with their timing information.
    #[serde(default)]
    pub characters: Vec<ForcedAlignmentCharacterResponseModel>,
    /// List of words with their timing information.
    #[serde(default)]
    pub words: Vec<ForcedAlignmentWordResponseModel>,
    /// The average alignment loss/confidence score for the entire transcript, calculated from all characters.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub loss: f64,
}

impl ForcedAlignmentResponseModel {
    pub fn builder() -> ForcedAlignmentResponseModelBuilder {
        <ForcedAlignmentResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ForcedAlignmentResponseModelBuilder {
    characters: Option<Vec<ForcedAlignmentCharacterResponseModel>>,
    words: Option<Vec<ForcedAlignmentWordResponseModel>>,
    loss: Option<f64>,
}

impl ForcedAlignmentResponseModelBuilder {
    pub fn characters(mut self, value: Vec<ForcedAlignmentCharacterResponseModel>) -> Self {
        self.characters = Some(value);
        self
    }

    pub fn words(mut self, value: Vec<ForcedAlignmentWordResponseModel>) -> Self {
        self.words = Some(value);
        self
    }

    pub fn loss(mut self, value: f64) -> Self {
        self.loss = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ForcedAlignmentResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`characters`](ForcedAlignmentResponseModelBuilder::characters)
    /// - [`words`](ForcedAlignmentResponseModelBuilder::words)
    /// - [`loss`](ForcedAlignmentResponseModelBuilder::loss)
    pub fn build(self) -> Result<ForcedAlignmentResponseModel, BuildError> {
        Ok(ForcedAlignmentResponseModel {
            characters: self.characters.ok_or_else(|| BuildError::missing_field("characters"))?,
            words: self.words.ok_or_else(|| BuildError::missing_field("words"))?,
            loss: self.loss.ok_or_else(|| BuildError::missing_field("loss"))?,
        })
    }
}
