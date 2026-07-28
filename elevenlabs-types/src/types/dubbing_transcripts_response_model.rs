pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DubbingTranscriptsResponseModel {
    pub transcript_format: DubbingTranscriptsResponseModelTranscriptFormat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webvtt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json: Option<DubbingTranscript>,
}

impl DubbingTranscriptsResponseModel {
    pub fn builder() -> DubbingTranscriptsResponseModelBuilder {
        <DubbingTranscriptsResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingTranscriptsResponseModelBuilder {
    transcript_format: Option<DubbingTranscriptsResponseModelTranscriptFormat>,
    srt: Option<String>,
    webvtt: Option<String>,
    json: Option<DubbingTranscript>,
}

impl DubbingTranscriptsResponseModelBuilder {
    pub fn transcript_format(mut self, value: DubbingTranscriptsResponseModelTranscriptFormat) -> Self {
        self.transcript_format = Some(value);
        self
    }

    pub fn srt(mut self, value: impl Into<String>) -> Self {
        self.srt = Some(value.into());
        self
    }

    pub fn webvtt(mut self, value: impl Into<String>) -> Self {
        self.webvtt = Some(value.into());
        self
    }

    pub fn json(mut self, value: DubbingTranscript) -> Self {
        self.json = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbingTranscriptsResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transcript_format`](DubbingTranscriptsResponseModelBuilder::transcript_format)
    pub fn build(self) -> Result<DubbingTranscriptsResponseModel, BuildError> {
        Ok(DubbingTranscriptsResponseModel {
            transcript_format: self.transcript_format.ok_or_else(|| BuildError::missing_field("transcript_format"))?,
            srt: self.srt,
            webvtt: self.webvtt,
            json: self.json,
        })
    }
}
