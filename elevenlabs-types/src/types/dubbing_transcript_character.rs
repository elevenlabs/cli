pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DubbingTranscriptCharacter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub start_s: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub end_s: Option<f64>,
}

impl DubbingTranscriptCharacter {
    pub fn builder() -> DubbingTranscriptCharacterBuilder {
        <DubbingTranscriptCharacterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingTranscriptCharacterBuilder {
    text: Option<String>,
    start_s: Option<f64>,
    end_s: Option<f64>,
}

impl DubbingTranscriptCharacterBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn start_s(mut self, value: f64) -> Self {
        self.start_s = Some(value);
        self
    }

    pub fn end_s(mut self, value: f64) -> Self {
        self.end_s = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbingTranscriptCharacter`].
    pub fn build(self) -> Result<DubbingTranscriptCharacter, BuildError> {
        Ok(DubbingTranscriptCharacter {
            text: self.text,
            start_s: self.start_s,
            end_s: self.end_s,
        })
    }
}
