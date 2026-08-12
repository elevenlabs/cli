pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyCreateUrlDocumentV1ConvaiKnowledgeBaseUrlPost {
    /// URL to a page of documentation that the agent will have access to in order to interact with users.
    #[serde(default)]
    pub url: String,
    /// A custom, human-readable name for the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// If set, the created document or folder will be placed inside the given folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// Whether to enable auto-sync for this URL document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_sync: Option<bool>,
    /// Whether to automatically remove the document if the URL becomes unavailable. Only applicable when auto-sync is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_remove: Option<bool>,
}

impl BodyCreateUrlDocumentV1ConvaiKnowledgeBaseUrlPost {
    pub fn builder() -> BodyCreateUrlDocumentV1ConvaiKnowledgeBaseUrlPostBuilder {
        <BodyCreateUrlDocumentV1ConvaiKnowledgeBaseUrlPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyCreateUrlDocumentV1ConvaiKnowledgeBaseUrlPostBuilder {
    url: Option<String>,
    name: Option<String>,
    parent_folder_id: Option<String>,
    enable_auto_sync: Option<bool>,
    auto_remove: Option<bool>,
}

impl BodyCreateUrlDocumentV1ConvaiKnowledgeBaseUrlPostBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn parent_folder_id(mut self, value: impl Into<String>) -> Self {
        self.parent_folder_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`BodyCreateUrlDocumentV1ConvaiKnowledgeBaseUrlPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](BodyCreateUrlDocumentV1ConvaiKnowledgeBaseUrlPostBuilder::url)
    pub fn build(self) -> Result<BodyCreateUrlDocumentV1ConvaiKnowledgeBaseUrlPost, BuildError> {
        Ok(BodyCreateUrlDocumentV1ConvaiKnowledgeBaseUrlPost {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            name: self.name,
            parent_folder_id: self.parent_folder_id,
            enable_auto_sync: self.enable_auto_sync,
            auto_remove: self.auto_remove,
        })
    }
}

