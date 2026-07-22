pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetKnowledgeBaseSummaryFolderResponseModel {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub metadata: KnowledgeBaseDocumentMetadataResponseModel,
    #[serde(default)]
    pub supported_usages: Vec<DocumentUsageModeEnum>,
    pub access_info: ResourceAccessInfo,
    /// The ID of the parent folder, or null if the document is at the root level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_parent_id: Option<String>,
    /// The folder path segments leading to this entity, from root to parent folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_path: Option<Vec<KnowledgeBaseFolderPathSegmentSummaryResponseModel>>,
    /// This field is deprecated and will be removed in the future, use the separate endpoint to get dependent agents instead.
    #[serde(default)]
    pub dependent_agents: Vec<GetKnowledgeBaseSummaryFolderResponseModelDependentAgentsItem>,
    #[serde(default)]
    pub children_count: i64,
    /// Number of non-folder documents anywhere in this folder's subtree (recursive). Counting stops past 1000;
    #[serde(default)]
    pub document_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_sync_info: Option<AutoSyncInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_sync_info: Option<ExternalFolderSyncInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_frozen: Option<bool>,
}

impl GetKnowledgeBaseSummaryFolderResponseModel {
    pub fn builder() -> GetKnowledgeBaseSummaryFolderResponseModelBuilder {
        <GetKnowledgeBaseSummaryFolderResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetKnowledgeBaseSummaryFolderResponseModelBuilder {
    id: Option<String>,
    name: Option<String>,
    metadata: Option<KnowledgeBaseDocumentMetadataResponseModel>,
    supported_usages: Option<Vec<DocumentUsageModeEnum>>,
    access_info: Option<ResourceAccessInfo>,
    folder_parent_id: Option<String>,
    folder_path: Option<Vec<KnowledgeBaseFolderPathSegmentSummaryResponseModel>>,
    dependent_agents: Option<Vec<GetKnowledgeBaseSummaryFolderResponseModelDependentAgentsItem>>,
    children_count: Option<i64>,
    document_count: Option<i64>,
    auto_sync_info: Option<AutoSyncInfo>,
    external_sync_info: Option<ExternalFolderSyncInfo>,
    is_frozen: Option<bool>,
}

impl GetKnowledgeBaseSummaryFolderResponseModelBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: KnowledgeBaseDocumentMetadataResponseModel) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn supported_usages(mut self, value: Vec<DocumentUsageModeEnum>) -> Self {
        self.supported_usages = Some(value);
        self
    }

    pub fn access_info(mut self, value: ResourceAccessInfo) -> Self {
        self.access_info = Some(value);
        self
    }

    pub fn folder_parent_id(mut self, value: impl Into<String>) -> Self {
        self.folder_parent_id = Some(value.into());
        self
    }

    pub fn folder_path(mut self, value: Vec<KnowledgeBaseFolderPathSegmentSummaryResponseModel>) -> Self {
        self.folder_path = Some(value);
        self
    }

    pub fn dependent_agents(mut self, value: Vec<GetKnowledgeBaseSummaryFolderResponseModelDependentAgentsItem>) -> Self {
        self.dependent_agents = Some(value);
        self
    }

    pub fn children_count(mut self, value: i64) -> Self {
        self.children_count = Some(value);
        self
    }

    pub fn document_count(mut self, value: i64) -> Self {
        self.document_count = Some(value);
        self
    }

    pub fn auto_sync_info(mut self, value: AutoSyncInfo) -> Self {
        self.auto_sync_info = Some(value);
        self
    }

    pub fn external_sync_info(mut self, value: ExternalFolderSyncInfo) -> Self {
        self.external_sync_info = Some(value);
        self
    }

    pub fn is_frozen(mut self, value: bool) -> Self {
        self.is_frozen = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetKnowledgeBaseSummaryFolderResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetKnowledgeBaseSummaryFolderResponseModelBuilder::id)
    /// - [`name`](GetKnowledgeBaseSummaryFolderResponseModelBuilder::name)
    /// - [`metadata`](GetKnowledgeBaseSummaryFolderResponseModelBuilder::metadata)
    /// - [`supported_usages`](GetKnowledgeBaseSummaryFolderResponseModelBuilder::supported_usages)
    /// - [`access_info`](GetKnowledgeBaseSummaryFolderResponseModelBuilder::access_info)
    /// - [`dependent_agents`](GetKnowledgeBaseSummaryFolderResponseModelBuilder::dependent_agents)
    /// - [`children_count`](GetKnowledgeBaseSummaryFolderResponseModelBuilder::children_count)
    /// - [`document_count`](GetKnowledgeBaseSummaryFolderResponseModelBuilder::document_count)
    pub fn build(self) -> Result<GetKnowledgeBaseSummaryFolderResponseModel, BuildError> {
        Ok(GetKnowledgeBaseSummaryFolderResponseModel {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            metadata: self.metadata.ok_or_else(|| BuildError::missing_field("metadata"))?,
            supported_usages: self.supported_usages.ok_or_else(|| BuildError::missing_field("supported_usages"))?,
            access_info: self.access_info.ok_or_else(|| BuildError::missing_field("access_info"))?,
            folder_parent_id: self.folder_parent_id,
            folder_path: self.folder_path,
            dependent_agents: self.dependent_agents.ok_or_else(|| BuildError::missing_field("dependent_agents"))?,
            children_count: self.children_count.ok_or_else(|| BuildError::missing_field("children_count"))?,
            document_count: self.document_count.ok_or_else(|| BuildError::missing_field("document_count"))?,
            auto_sync_info: self.auto_sync_info,
            external_sync_info: self.external_sync_info,
            is_frozen: self.is_frozen,
        })
    }
}
