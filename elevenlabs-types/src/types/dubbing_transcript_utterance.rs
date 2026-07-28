pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DubbingTranscriptUtterance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub start_s: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub end_s: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub words: Option<Vec<DubbingTranscriptWord>>,
}

impl DubbingTranscriptUtterance {
    pub fn builder() -> DubbingTranscriptUtteranceBuilder {
        <DubbingTranscriptUtteranceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingTranscriptUtteranceBuilder {
    text: Option<String>,
    speaker_id: Option<String>,
    start_s: Option<f64>,
    end_s: Option<f64>,
    words: Option<Vec<DubbingTranscriptWord>>,
}

impl DubbingTranscriptUtteranceBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn speaker_id(mut self, value: impl Into<String>) -> Self {
        self.speaker_id = Some(value.into());
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

    pub fn words(mut self, value: Vec<DubbingTranscriptWord>) -> Self {
        self.words = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbingTranscriptUtterance`].
    pub fn build(self) -> Result<DubbingTranscriptUtterance, BuildError> {
        Ok(DubbingTranscriptUtterance {
            text: self.text,
            speaker_id: self.speaker_id,
            start_s: self.start_s,
            end_s: self.end_s,
            words: self.words,
        })
    }
}
