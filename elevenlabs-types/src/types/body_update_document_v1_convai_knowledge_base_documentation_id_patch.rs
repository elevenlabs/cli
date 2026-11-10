pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyUpdateDocumentV1ConvaiKnowledgeBaseDocumentationIdPatch {
    /// A custom, human-readable name for the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated content for the document. Only supported for text documents, URL documents with auto-sync disabled, and file documents that are not synced from an external source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Whether the document or folder is refreshed automatically. Only applicable to URL documents, folders, and externally-synced files. Leave unset to keep the current value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_sync: Option<bool>,
    /// Whether to automatically remove the document once its source is no longer available. Only applicable when auto-sync is enabled. Leave unset to keep the current value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_remove: Option<bool>,
    /// Minimum frequency (in days) at which the document is refreshed. The actual interval may be shorter, never longer. Only applicable when auto-sync is enabled. Leave unset to keep the current value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_frequency_days: Option<i64>,
    /// Folders only. When true, items already in the folder are only tightened to stay at least as strict as the folder, and keep any setting that is stricter than the folder's. When false, they are set to exactly the folder's auto-sync settings. Ignored for externally synced folders, whose contents always follow the folder's schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tighten_only: Option<bool>,
}

impl BodyUpdateDocumentV1ConvaiKnowledgeBaseDocumentationIdPatch {
    pub fn builder() -> BodyUpdateDocumentV1ConvaiKnowledgeBaseDocumentationIdPatchBuilder {
        <BodyUpdateDocumentV1ConvaiKnowledgeBaseDocumentationIdPatchBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyUpdateDocumentV1ConvaiKnowledgeBaseDocumentationIdPatchBuilder {
    name: Option<String>,
    content: Option<String>,
    enable_auto_sync: Option<bool>,
    auto_remove: Option<bool>,
    minimum_frequency_days: Option<i64>,
    tighten_only: Option<bool>,
}

impl BodyUpdateDocumentV1ConvaiKnowledgeBaseDocumentationIdPatchBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn enable_auto_sync(mut self, value: bool) -> Self {
        self.enable_auto_sync = Some(value);
        self
    }

    pub fn auto_remove(mut self, value: bool) -> Self {
        self.auto_remove = Some(value);
        self
    }

    pub fn minimum_frequency_days(mut self, value: i64) -> Self {
        self.minimum_frequency_days = Some(value);
        self
    }

    pub fn tighten_only(mut self, value: bool) -> Self {
        self.tighten_only = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyUpdateDocumentV1ConvaiKnowledgeBaseDocumentationIdPatch`].
    pub fn build(self) -> Result<BodyUpdateDocumentV1ConvaiKnowledgeBaseDocumentationIdPatch, BuildError> {
        Ok(BodyUpdateDocumentV1ConvaiKnowledgeBaseDocumentationIdPatch {
            name: self.name,
            content: self.content,
            enable_auto_sync: self.enable_auto_sync,
            auto_remove: self.auto_remove,
            minimum_frequency_days: self.minimum_frequency_days,
            tighten_only: self.tighten_only,
        })
    }
}

