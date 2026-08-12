pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FileInputConfig {
    /// When enabled, users may attach images or PDFs in chat when the LLM supports multimodal input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Number of most-recent files kept in memory during a conversation. Older files are summarized and their bytes freed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_files_in_memory: Option<i64>,
    /// Total files a user can upload in one conversation. Uploads are billed per file. Use -1 for no limit, or a value >= max_files_in_memory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_files_per_conversation: Option<i64>,
}

impl FileInputConfig {
    pub fn builder() -> FileInputConfigBuilder {
        <FileInputConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FileInputConfigBuilder {
    enabled: Option<bool>,
    max_files_in_memory: Option<i64>,
    max_files_per_conversation: Option<i64>,
}

impl FileInputConfigBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn max_files_in_memory(mut self, value: i64) -> Self {
        self.max_files_in_memory = Some(value);
        self
    }

    pub fn max_files_per_conversation(mut self, value: i64) -> Self {
        self.max_files_per_conversation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FileInputConfig`].
    pub fn build(self) -> Result<FileInputConfig, BuildError> {
        Ok(FileInputConfig {
            enabled: self.enabled,
            max_files_in_memory: self.max_files_in_memory,
            max_files_per_conversation: self.max_files_per_conversation,
        })
    }
}
