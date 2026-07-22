pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecordingResponse {
    /// The ID of the recording.
    #[serde(default)]
    pub recording_id: String,
    /// The MIME type of the recording.
    #[serde(default)]
    pub mime_type: String,
    /// The size of the recording in bytes.
    #[serde(default)]
    pub size_bytes: i64,
    /// The date of the recording in Unix time.
    #[serde(default)]
    pub upload_date_unix: i64,
    /// The transcription of the recording.
    #[serde(default)]
    pub transcription: String,
}

impl RecordingResponse {
    pub fn builder() -> RecordingResponseBuilder {
        <RecordingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecordingResponseBuilder {
    recording_id: Option<String>,
    mime_type: Option<String>,
    size_bytes: Option<i64>,
    upload_date_unix: Option<i64>,
    transcription: Option<String>,
}

impl RecordingResponseBuilder {
    pub fn recording_id(mut self, value: impl Into<String>) -> Self {
        self.recording_id = Some(value.into());
        self
    }

    pub fn mime_type(mut self, value: impl Into<String>) -> Self {
        self.mime_type = Some(value.into());
        self
    }

    pub fn size_bytes(mut self, value: i64) -> Self {
        self.size_bytes = Some(value);
        self
    }

    pub fn upload_date_unix(mut self, value: i64) -> Self {
        self.upload_date_unix = Some(value);
        self
    }

    pub fn transcription(mut self, value: impl Into<String>) -> Self {
        self.transcription = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RecordingResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`recording_id`](RecordingResponseBuilder::recording_id)
    /// - [`mime_type`](RecordingResponseBuilder::mime_type)
    /// - [`size_bytes`](RecordingResponseBuilder::size_bytes)
    /// - [`upload_date_unix`](RecordingResponseBuilder::upload_date_unix)
    /// - [`transcription`](RecordingResponseBuilder::transcription)
    pub fn build(self) -> Result<RecordingResponse, BuildError> {
        Ok(RecordingResponse {
            recording_id: self.recording_id.ok_or_else(|| BuildError::missing_field("recording_id"))?,
            mime_type: self.mime_type.ok_or_else(|| BuildError::missing_field("mime_type"))?,
            size_bytes: self.size_bytes.ok_or_else(|| BuildError::missing_field("size_bytes"))?,
            upload_date_unix: self.upload_date_unix.ok_or_else(|| BuildError::missing_field("upload_date_unix"))?,
            transcription: self.transcription.ok_or_else(|| BuildError::missing_field("transcription"))?,
        })
    }
}
