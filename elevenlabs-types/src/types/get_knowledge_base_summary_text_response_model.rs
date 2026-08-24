pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetKnowledgeBaseSummaryTextResponseModel {
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
    pub dependent_agents: Vec<GetKnowledgeBaseSummaryTextResponseModelDependentAgentsItem>,
}

impl GetKnowledgeBaseSummaryTextResponseModel {
    pub fn builder() -> GetKnowledgeBaseSummaryTextResponseModelBuilder {
        <GetKnowledgeBaseSummaryTextResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetKnowledgeBaseSummaryTextResponseModelBuilder {
    id: Option<String>,
    name: Option<String>,
    metadata: Option<KnowledgeBaseDocumentMetadataResponseModel>,
    supported_usages: Option<Vec<DocumentUsageModeEnum>>,
    access_info: Option<ResourceAccessInfo>,
    folder_parent_id: Option<String>,
    folder_path: Option<Vec<KnowledgeBaseFolderPathSegmentSummaryResponseModel>>,
    dependent_agents: Option<Vec<GetKnowledgeBaseSummaryTextResponseModelDependentAgentsItem>>,
}

impl GetKnowledgeBaseSummaryTextResponseModelBuilder {
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

    pub fn dependent_agents(mut self, value: Vec<GetKnowledgeBaseSummaryTextResponseModelDependentAgentsItem>) -> Self {
        self.dependent_agents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetKnowledgeBaseSummaryTextResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetKnowledgeBaseSummaryTextResponseModelBuilder::id)
    /// - [`name`](GetKnowledgeBaseSummaryTextResponseModelBuilder::name)
    /// - [`metadata`](GetKnowledgeBaseSummaryTextResponseModelBuilder::metadata)
    /// - [`supported_usages`](GetKnowledgeBaseSummaryTextResponseModelBuilder::supported_usages)
    /// - [`access_info`](GetKnowledgeBaseSummaryTextResponseModelBuilder::access_info)
    /// - [`dependent_agents`](GetKnowledgeBaseSummaryTextResponseModelBuilder::dependent_agents)
    pub fn build(self) -> Result<GetKnowledgeBaseSummaryTextResponseModel, BuildError> {
        Ok(GetKnowledgeBaseSummaryTextResponseModel {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            metadata: self.metadata.ok_or_else(|| BuildError::missing_field("metadata"))?,
            supported_usages: self.supported_usages.ok_or_else(|| BuildError::missing_field("supported_usages"))?,
            access_info: self.access_info.ok_or_else(|| BuildError::missing_field("access_info"))?,
            folder_parent_id: self.folder_parent_id,
            folder_path: self.folder_path,
            dependent_agents: self.dependent_agents.ok_or_else(|| BuildError::missing_field("dependent_agents"))?,
        })
    }
}
