pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DubbingTranscriptResponseModel {
    #[serde(default)]
    pub language: String,
    #[serde(default)]
    pub utterances: Vec<DubbingTranscriptUtterance>,
}

impl DubbingTranscriptResponseModel {
    pub fn builder() -> DubbingTranscriptResponseModelBuilder {
        <DubbingTranscriptResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingTranscriptResponseModelBuilder {
    language: Option<String>,
    utterances: Option<Vec<DubbingTranscriptUtterance>>,
}

impl DubbingTranscriptResponseModelBuilder {
    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn utterances(mut self, value: Vec<DubbingTranscriptUtterance>) -> Self {
        self.utterances = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbingTranscriptResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`language`](DubbingTranscriptResponseModelBuilder::language)
    /// - [`utterances`](DubbingTranscriptResponseModelBuilder::utterances)
    pub fn build(self) -> Result<DubbingTranscriptResponseModel, BuildError> {
        Ok(DubbingTranscriptResponseModel {
            language: self.language.ok_or_else(|| BuildError::missing_field("language"))?,
            utterances: self.utterances.ok_or_else(|| BuildError::missing_field("utterances"))?,
        })
    }
}
