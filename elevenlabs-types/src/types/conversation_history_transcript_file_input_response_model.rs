pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationHistoryTranscriptFileInputResponseModel {
    #[serde(default)]
    pub file_id: String,
    #[serde(default)]
    pub original_filename: String,
    #[serde(default)]
    pub mime_type: String,
    #[serde(default)]
    pub file_url: String,
}

impl ConversationHistoryTranscriptFileInputResponseModel {
    pub fn builder() -> ConversationHistoryTranscriptFileInputResponseModelBuilder {
        <ConversationHistoryTranscriptFileInputResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationHistoryTranscriptFileInputResponseModelBuilder {
    file_id: Option<String>,
    original_filename: Option<String>,
    mime_type: Option<String>,
    file_url: Option<String>,
}

impl ConversationHistoryTranscriptFileInputResponseModelBuilder {
    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn original_filename(mut self, value: impl Into<String>) -> Self {
        self.original_filename = Some(value.into());
        self
    }

    pub fn mime_type(mut self, value: impl Into<String>) -> Self {
        self.mime_type = Some(value.into());
        self
    }

    pub fn file_url(mut self, value: impl Into<String>) -> Self {
        self.file_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationHistoryTranscriptFileInputResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_id`](ConversationHistoryTranscriptFileInputResponseModelBuilder::file_id)
    /// - [`original_filename`](ConversationHistoryTranscriptFileInputResponseModelBuilder::original_filename)
    /// - [`mime_type`](ConversationHistoryTranscriptFileInputResponseModelBuilder::mime_type)
    /// - [`file_url`](ConversationHistoryTranscriptFileInputResponseModelBuilder::file_url)
    pub fn build(self) -> Result<ConversationHistoryTranscriptFileInputResponseModel, BuildError> {
        Ok(ConversationHistoryTranscriptFileInputResponseModel {
            file_id: self.file_id.ok_or_else(|| BuildError::missing_field("file_id"))?,
            original_filename: self.original_filename.ok_or_else(|| BuildError::missing_field("original_filename"))?,
            mime_type: self.mime_type.ok_or_else(|| BuildError::missing_field("mime_type"))?,
            file_url: self.file_url.ok_or_else(|| BuildError::missing_field("file_url"))?,
        })
    }
}
