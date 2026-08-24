pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UnitTestSummaryResponseModel {
    /// The ID of the test
    #[serde(default)]
    pub id: String,
    /// Name of the test
    #[serde(default)]
    pub name: String,
    /// The access information of the test
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_info: Option<ResourceAccessInfo>,
    /// Creation time of the test in unix seconds
    #[serde(default)]
    pub created_at_unix_secs: i64,
    /// Last update time of the test in unix seconds
    #[serde(default)]
    pub last_updated_at_unix_secs: i64,
    /// Type of the test or entity
    pub r#type: TestType,
    /// The type of entity (test or folder)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<AgentTestEntityType>,
    /// The ID of the parent folder
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_parent_id: Option<String>,
    /// The folder path segments from root to this entity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_path: Option<Vec<AgentTestFolderPathSegmentResponseModel>>,
    /// Number of direct children (tests and subfolders) for folders only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children_count: Option<i64>,
    /// Channel the test simulates the conversation as. Null for folders or default behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_initiation_source: Option<ConversationInitiationSource>,
}

impl UnitTestSummaryResponseModel {
    pub fn builder() -> UnitTestSummaryResponseModelBuilder {
        <UnitTestSummaryResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnitTestSummaryResponseModelBuilder {
    id: Option<String>,
    name: Option<String>,
    access_info: Option<ResourceAccessInfo>,
    created_at_unix_secs: Option<i64>,
    last_updated_at_unix_secs: Option<i64>,
    r#type: Option<TestType>,
    entity_type: Option<AgentTestEntityType>,
    folder_parent_id: Option<String>,
    folder_path: Option<Vec<AgentTestFolderPathSegmentResponseModel>>,
    children_count: Option<i64>,
    conversation_initiation_source: Option<ConversationInitiationSource>,
}

impl UnitTestSummaryResponseModelBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn access_info(mut self, value: ResourceAccessInfo) -> Self {
        self.access_info = Some(value);
        self
    }

    pub fn created_at_unix_secs(mut self, value: i64) -> Self {
        self.created_at_unix_secs = Some(value);
        self
    }

    pub fn last_updated_at_unix_secs(mut self, value: i64) -> Self {
        self.last_updated_at_unix_secs = Some(value);
        self
    }

    pub fn r#type(mut self, value: TestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn entity_type(mut self, value: AgentTestEntityType) -> Self {
        self.entity_type = Some(value);
        self
    }

    pub fn folder_parent_id(mut self, value: impl Into<String>) -> Self {
        self.folder_parent_id = Some(value.into());
        self
    }

    pub fn folder_path(mut self, value: Vec<AgentTestFolderPathSegmentResponseModel>) -> Self {
        self.folder_path = Some(value);
        self
    }

    pub fn children_count(mut self, value: i64) -> Self {
        self.children_count = Some(value);
        self
    }

    pub fn conversation_initiation_source(mut self, value: ConversationInitiationSource) -> Self {
        self.conversation_initiation_source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UnitTestSummaryResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UnitTestSummaryResponseModelBuilder::id)
    /// - [`name`](UnitTestSummaryResponseModelBuilder::name)
    /// - [`created_at_unix_secs`](UnitTestSummaryResponseModelBuilder::created_at_unix_secs)
    /// - [`last_updated_at_unix_secs`](UnitTestSummaryResponseModelBuilder::last_updated_at_unix_secs)
    /// - [`r#type`](UnitTestSummaryResponseModelBuilder::r#type)
    pub fn build(self) -> Result<UnitTestSummaryResponseModel, BuildError> {
        Ok(UnitTestSummaryResponseModel {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            access_info: self.access_info,
            created_at_unix_secs: self.created_at_unix_secs.ok_or_else(|| BuildError::missing_field("created_at_unix_secs"))?,
            last_updated_at_unix_secs: self.last_updated_at_unix_secs.ok_or_else(|| BuildError::missing_field("last_updated_at_unix_secs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            entity_type: self.entity_type,
            folder_parent_id: self.folder_parent_id,
            folder_path: self.folder_path,
            children_count: self.children_count,
            conversation_initiation_source: self.conversation_initiation_source,
        })
    }
}
