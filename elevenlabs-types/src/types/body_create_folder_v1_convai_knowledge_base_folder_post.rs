pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyCreateFolderV1ConvaiKnowledgeBaseFolderPost {
    /// A custom, human-readable name for the document.
    #[serde(default)]
    pub name: String,
    /// If set, the created document or folder will be placed inside the given folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// Whether to enable auto-sync for this URL document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_sync: Option<bool>,
    /// Whether to automatically remove the document if the URL becomes unavailable. Only applicable when auto-sync is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_remove: Option<bool>,
    /// Minimum frequency (in days) at which the underlying eligible documents are refreshed. The actual interval may be shorter, never longer. Defaults to 7, tightened to the parent folder's frequency if that is stricter. Only applicable when auto-sync is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_frequency_days: Option<i64>,
}

impl BodyCreateFolderV1ConvaiKnowledgeBaseFolderPost {
    pub fn builder() -> BodyCreateFolderV1ConvaiKnowledgeBaseFolderPostBuilder {
        <BodyCreateFolderV1ConvaiKnowledgeBaseFolderPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyCreateFolderV1ConvaiKnowledgeBaseFolderPostBuilder {
    name: Option<String>,
    parent_folder_id: Option<String>,
    enable_auto_sync: Option<bool>,
    auto_remove: Option<bool>,
    minimum_frequency_days: Option<i64>,
}

impl BodyCreateFolderV1ConvaiKnowledgeBaseFolderPostBuilder {
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

    pub fn minimum_frequency_days(mut self, value: i64) -> Self {
        self.minimum_frequency_days = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyCreateFolderV1ConvaiKnowledgeBaseFolderPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](BodyCreateFolderV1ConvaiKnowledgeBaseFolderPostBuilder::name)
    pub fn build(self) -> Result<BodyCreateFolderV1ConvaiKnowledgeBaseFolderPost, BuildError> {
        Ok(BodyCreateFolderV1ConvaiKnowledgeBaseFolderPost {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            parent_folder_id: self.parent_folder_id,
            enable_auto_sync: self.enable_auto_sync,
            auto_remove: self.auto_remove,
            minimum_frequency_days: self.minimum_frequency_days,
        })
    }
}

