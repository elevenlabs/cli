pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DubbingTranscript {
    #[serde(default)]
    pub language: String,
    #[serde(default)]
    pub utterances: Vec<DubbingTranscriptUtterance>,
}

impl DubbingTranscript {
    pub fn builder() -> DubbingTranscriptBuilder {
        <DubbingTranscriptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingTranscriptBuilder {
    language: Option<String>,
    utterances: Option<Vec<DubbingTranscriptUtterance>>,
}

impl DubbingTranscriptBuilder {
    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn utterances(mut self, value: Vec<DubbingTranscriptUtterance>) -> Self {
        self.utterances = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbingTranscript`].
    /// This method will fail if any of the following fields are not set:
    /// - [`language`](DubbingTranscriptBuilder::language)
    /// - [`utterances`](DubbingTranscriptBuilder::utterances)
    pub fn build(self) -> Result<DubbingTranscript, BuildError> {
        Ok(DubbingTranscript {
            language: self.language.ok_or_else(|| BuildError::missing_field("language"))?,
            utterances: self.utterances.ok_or_else(|| BuildError::missing_field("utterances"))?,
        })
    }
}
