pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get_transcript_for_dub
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTranscriptForDubQueryRequest {
    /// Format to return transcript in. For subtitles use either 'srt' or 'webvtt', and for a full transcript use 'json'. The 'json' format is not yet supported for Dubbing Studio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_type: Option<TranscriptGetTranscriptForDubRequestFormatType>,
}

impl GetTranscriptForDubQueryRequest {
    pub fn builder() -> GetTranscriptForDubQueryRequestBuilder {
        <GetTranscriptForDubQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTranscriptForDubQueryRequestBuilder {
    format_type: Option<TranscriptGetTranscriptForDubRequestFormatType>,
}

impl GetTranscriptForDubQueryRequestBuilder {
    pub fn format_type(mut self, value: TranscriptGetTranscriptForDubRequestFormatType) -> Self {
        self.format_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetTranscriptForDubQueryRequest`].
    pub fn build(self) -> Result<GetTranscriptForDubQueryRequest, BuildError> {
        Ok(GetTranscriptForDubQueryRequest {
            format_type: self.format_type,
        })
    }
}

