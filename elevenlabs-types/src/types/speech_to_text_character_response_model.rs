pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SpeechToTextCharacterResponseModel {
    /// The character that was transcribed.
    #[serde(default)]
    pub text: String,
    /// The start time of the character in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub start: Option<f64>,
    /// The end time of the character in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub end: Option<f64>,
}

impl SpeechToTextCharacterResponseModel {
    pub fn builder() -> SpeechToTextCharacterResponseModelBuilder {
        <SpeechToTextCharacterResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpeechToTextCharacterResponseModelBuilder {
    text: Option<String>,
    start: Option<f64>,
    end: Option<f64>,
}

impl SpeechToTextCharacterResponseModelBuilder {
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

    /// Consumes the builder and constructs a [`SpeechToTextCharacterResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](SpeechToTextCharacterResponseModelBuilder::text)
    pub fn build(self) -> Result<SpeechToTextCharacterResponseModel, BuildError> {
        Ok(SpeechToTextCharacterResponseModel {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            start: self.start,
            end: self.end,
        })
    }
}
