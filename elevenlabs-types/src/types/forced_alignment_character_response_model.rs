pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Model representing a single character with its timing information from the aligner.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ForcedAlignmentCharacterResponseModel {
    /// The character that was transcribed.
    #[serde(default)]
    pub text: String,
    /// The start time of the character in seconds.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub start: f64,
    /// The end time of the character in seconds.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub end: f64,
}

impl ForcedAlignmentCharacterResponseModel {
    pub fn builder() -> ForcedAlignmentCharacterResponseModelBuilder {
        <ForcedAlignmentCharacterResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ForcedAlignmentCharacterResponseModelBuilder {
    text: Option<String>,
    start: Option<f64>,
    end: Option<f64>,
}

impl ForcedAlignmentCharacterResponseModelBuilder {
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

    /// Consumes the builder and constructs a [`ForcedAlignmentCharacterResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](ForcedAlignmentCharacterResponseModelBuilder::text)
    /// - [`start`](ForcedAlignmentCharacterResponseModelBuilder::start)
    /// - [`end`](ForcedAlignmentCharacterResponseModelBuilder::end)
    pub fn build(self) -> Result<ForcedAlignmentCharacterResponseModel, BuildError> {
        Ok(ForcedAlignmentCharacterResponseModel {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            start: self.start.ok_or_else(|| BuildError::missing_field("start"))?,
            end: self.end.ok_or_else(|| BuildError::missing_field("end"))?,
        })
    }
}
