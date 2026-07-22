pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Model representing a single word with its timing information from the aligner.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ForcedAlignmentWordResponseModel {
    /// The word that was transcribed.
    #[serde(default)]
    pub text: String,
    /// The start time of the word in seconds.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub start: f64,
    /// The end time of the word in seconds.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub end: f64,
    /// The average alignment loss/confidence score for this word, calculated from its constituent characters.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub loss: f64,
}

impl ForcedAlignmentWordResponseModel {
    pub fn builder() -> ForcedAlignmentWordResponseModelBuilder {
        <ForcedAlignmentWordResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ForcedAlignmentWordResponseModelBuilder {
    text: Option<String>,
    start: Option<f64>,
    end: Option<f64>,
    loss: Option<f64>,
}

impl ForcedAlignmentWordResponseModelBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn start(mut self, value: f64) -> Self {
        self.start = Some(value);
        self
    }

    pub fn end(mut self, value: f64) -> Self {
        self.end = Some(value);
        self
    }

    pub fn loss(mut self, value: f64) -> Self {
        self.loss = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ForcedAlignmentWordResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](ForcedAlignmentWordResponseModelBuilder::text)
    /// - [`start`](ForcedAlignmentWordResponseModelBuilder::start)
    /// - [`end`](ForcedAlignmentWordResponseModelBuilder::end)
    /// - [`loss`](ForcedAlignmentWordResponseModelBuilder::loss)
    pub fn build(self) -> Result<ForcedAlignmentWordResponseModel, BuildError> {
        Ok(ForcedAlignmentWordResponseModel {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            start: self.start.ok_or_else(|| BuildError::missing_field("start"))?,
            end: self.end.ok_or_else(|| BuildError::missing_field("end"))?,
            loss: self.loss.ok_or_else(|| BuildError::missing_field("loss"))?,
        })
    }
}
