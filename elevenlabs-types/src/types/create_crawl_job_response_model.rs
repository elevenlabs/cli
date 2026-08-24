pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateCrawlJobResponseModel {
    #[serde(default)]
    pub id: String,
    pub r#type: CrawlType,
    #[serde(default)]
    pub root_folder_id: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub created_at: i64,
    /// The folder path segments leading to the root folder, from root to parent folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_path: Option<Vec<KnowledgeBaseFolderPathSegmentSummaryResponseModel>>,
}

impl CreateCrawlJobResponseModel {
    pub fn builder() -> CreateCrawlJobResponseModelBuilder {
        <CreateCrawlJobResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCrawlJobResponseModelBuilder {
    id: Option<String>,
    r#type: Option<CrawlType>,
    root_folder_id: Option<String>,
    status: Option<String>,
    created_at: Option<i64>,
    folder_path: Option<Vec<KnowledgeBaseFolderPathSegmentSummaryResponseModel>>,
}

impl CreateCrawlJobResponseModelBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: CrawlType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn root_folder_id(mut self, value: impl Into<String>) -> Self {
        self.root_folder_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: i64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn folder_path(mut self, value: Vec<KnowledgeBaseFolderPathSegmentSummaryResponseModel>) -> Self {
        self.folder_path = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateCrawlJobResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateCrawlJobResponseModelBuilder::id)
    /// - [`r#type`](CreateCrawlJobResponseModelBuilder::r#type)
    /// - [`root_folder_id`](CreateCrawlJobResponseModelBuilder::root_folder_id)
    /// - [`status`](CreateCrawlJobResponseModelBuilder::status)
    /// - [`created_at`](CreateCrawlJobResponseModelBuilder::created_at)
    pub fn build(self) -> Result<CreateCrawlJobResponseModel, BuildError> {
        Ok(CreateCrawlJobResponseModel {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            root_folder_id: self.root_folder_id.ok_or_else(|| BuildError::missing_field("root_folder_id"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            folder_path: self.folder_path,
        })
    }
}
