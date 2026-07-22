pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetKnowledgeBaseUrlResponseModel {
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
    pub folder_path: Option<Vec<KnowledgeBaseFolderPathSegmentResponseModel>>,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub extracted_inner_html: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_format: Option<ContentFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_sync_info: Option<AutoSyncInfo>,
}

impl GetKnowledgeBaseUrlResponseModel {
    pub fn builder() -> GetKnowledgeBaseUrlResponseModelBuilder {
        <GetKnowledgeBaseUrlResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetKnowledgeBaseUrlResponseModelBuilder {
    id: Option<String>,
    name: Option<String>,
    metadata: Option<KnowledgeBaseDocumentMetadataResponseModel>,
    supported_usages: Option<Vec<DocumentUsageModeEnum>>,
    access_info: Option<ResourceAccessInfo>,
    folder_parent_id: Option<String>,
    folder_path: Option<Vec<KnowledgeBaseFolderPathSegmentResponseModel>>,
    url: Option<String>,
    extracted_inner_html: Option<String>,
    content_format: Option<ContentFormat>,
    auto_sync_info: Option<AutoSyncInfo>,
}

impl GetKnowledgeBaseUrlResponseModelBuilder {
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

    pub fn folder_path(mut self, value: Vec<KnowledgeBaseFolderPathSegmentResponseModel>) -> Self {
        self.folder_path = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn extracted_inner_html(mut self, value: impl Into<String>) -> Self {
        self.extracted_inner_html = Some(value.into());
        self
    }

    pub fn content_format(mut self, value: ContentFormat) -> Self {
        self.content_format = Some(value);
        self
    }

    pub fn auto_sync_info(mut self, value: AutoSyncInfo) -> Self {
        self.auto_sync_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetKnowledgeBaseUrlResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetKnowledgeBaseUrlResponseModelBuilder::id)
    /// - [`name`](GetKnowledgeBaseUrlResponseModelBuilder::name)
    /// - [`metadata`](GetKnowledgeBaseUrlResponseModelBuilder::metadata)
    /// - [`supported_usages`](GetKnowledgeBaseUrlResponseModelBuilder::supported_usages)
    /// - [`access_info`](GetKnowledgeBaseUrlResponseModelBuilder::access_info)
    /// - [`url`](GetKnowledgeBaseUrlResponseModelBuilder::url)
    /// - [`extracted_inner_html`](GetKnowledgeBaseUrlResponseModelBuilder::extracted_inner_html)
    pub fn build(self) -> Result<GetKnowledgeBaseUrlResponseModel, BuildError> {
        Ok(GetKnowledgeBaseUrlResponseModel {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            metadata: self.metadata.ok_or_else(|| BuildError::missing_field("metadata"))?,
            supported_usages: self.supported_usages.ok_or_else(|| BuildError::missing_field("supported_usages"))?,
            access_info: self.access_info.ok_or_else(|| BuildError::missing_field("access_info"))?,
            folder_parent_id: self.folder_parent_id,
            folder_path: self.folder_path,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            extracted_inner_html: self.extracted_inner_html.ok_or_else(|| BuildError::missing_field("extracted_inner_html"))?,
            content_format: self.content_format,
            auto_sync_info: self.auto_sync_info,
        })
    }
}
