pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UtteranceResponseModel {
    /// The start time of the utterance in seconds.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub start: f64,
    /// The end time of the utterance in seconds.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub end: f64,
}

impl UtteranceResponseModel {
    pub fn builder() -> UtteranceResponseModelBuilder {
        <UtteranceResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UtteranceResponseModelBuilder {
    start: Option<f64>,
    end: Option<f64>,
}

impl UtteranceResponseModelBuilder {
    pub fn start(mut self, value: f64) -> Self {
        self.start = Some(value);
        self
    }

    pub fn end(mut self, value: f64) -> Self {
        self.end = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UtteranceResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start`](UtteranceResponseModelBuilder::start)
    /// - [`end`](UtteranceResponseModelBuilder::end)
    pub fn build(self) -> Result<UtteranceResponseModel, BuildError> {
        Ok(UtteranceResponseModel {
            start: self.start.ok_or_else(|| BuildError::missing_field("start"))?,
            end: self.end.ok_or_else(|| BuildError::missing_field("end"))?,
        })
    }
}
