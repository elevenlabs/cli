pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DubbingTranscriptWord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub start_s: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub end_s: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub characters: Option<Vec<DubbingTranscriptCharacter>>,
}

impl DubbingTranscriptWord {
    pub fn builder() -> DubbingTranscriptWordBuilder {
        <DubbingTranscriptWordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingTranscriptWordBuilder {
    text: Option<String>,
    word_type: Option<String>,
    start_s: Option<f64>,
    end_s: Option<f64>,
    characters: Option<Vec<DubbingTranscriptCharacter>>,
}

impl DubbingTranscriptWordBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn word_type(mut self, value: impl Into<String>) -> Self {
        self.word_type = Some(value.into());
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

    pub fn characters(mut self, value: Vec<DubbingTranscriptCharacter>) -> Self {
        self.characters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbingTranscriptWord`].
    pub fn build(self) -> Result<DubbingTranscriptWord, BuildError> {
        Ok(DubbingTranscriptWord {
            text: self.text,
            word_type: self.word_type,
            start_s: self.start_s,
            end_s: self.end_s,
            characters: self.characters,
        })
    }
}
