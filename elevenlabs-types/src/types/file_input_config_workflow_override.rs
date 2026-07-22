pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FileInputConfigWorkflowOverride {
    /// When enabled, users may attach images or PDFs in chat when the LLM supports multimodal input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Maximum number of files that can be uploaded per conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_files_per_conversation: Option<i64>,
}

impl FileInputConfigWorkflowOverride {
    pub fn builder() -> FileInputConfigWorkflowOverrideBuilder {
        <FileInputConfigWorkflowOverrideBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FileInputConfigWorkflowOverrideBuilder {
    enabled: Option<bool>,
    max_files_per_conversation: Option<i64>,
}

impl FileInputConfigWorkflowOverrideBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn max_files_per_conversation(mut self, value: i64) -> Self {
        self.max_files_per_conversation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FileInputConfigWorkflowOverride`].
    pub fn build(self) -> Result<FileInputConfigWorkflowOverride, BuildError> {
        Ok(FileInputConfigWorkflowOverride {
            enabled: self.enabled,
            max_files_per_conversation: self.max_files_per_conversation,
        })
    }
}
