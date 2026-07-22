pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UnitTestRunResponseModel {
    #[serde(default)]
    pub test_run_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_info: Option<UnitTestRunResponseModelTestInfo>,
    #[serde(default)]
    pub test_invocation_id: String,
    #[serde(default)]
    pub agent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_node_id: Option<String>,
    pub status: TestRunStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_responses: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>,
    #[serde(default)]
    pub test_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_result: Option<TestConditionResultCommonModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at_unix: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<TestRunMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_folder_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_folder_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
}

impl UnitTestRunResponseModel {
    pub fn builder() -> UnitTestRunResponseModelBuilder {
        <UnitTestRunResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnitTestRunResponseModelBuilder {
    test_run_id: Option<String>,
    test_info: Option<UnitTestRunResponseModelTestInfo>,
    test_invocation_id: Option<String>,
    agent_id: Option<String>,
    branch_id: Option<String>,
    workflow_node_id: Option<String>,
    status: Option<TestRunStatus>,
    agent_responses: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>,
    test_id: Option<String>,
    test_name: Option<String>,
    condition_result: Option<TestConditionResultCommonModel>,
    last_updated_at_unix: Option<i64>,
    metadata: Option<TestRunMetadata>,
    root_folder_id: Option<String>,
    root_folder_name: Option<String>,
    environment: Option<String>,
}

impl UnitTestRunResponseModelBuilder {
    pub fn test_run_id(mut self, value: impl Into<String>) -> Self {
        self.test_run_id = Some(value.into());
        self
    }

    pub fn test_info(mut self, value: UnitTestRunResponseModelTestInfo) -> Self {
        self.test_info = Some(value);
        self
    }

    pub fn test_invocation_id(mut self, value: impl Into<String>) -> Self {
        self.test_invocation_id = Some(value.into());
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    pub fn workflow_node_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_node_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: TestRunStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn agent_responses(mut self, value: Vec<ConversationHistoryTranscriptCommonModelOutput>) -> Self {
        self.agent_responses = Some(value);
        self
    }

    pub fn test_id(mut self, value: impl Into<String>) -> Self {
        self.test_id = Some(value.into());
        self
    }

    pub fn test_name(mut self, value: impl Into<String>) -> Self {
        self.test_name = Some(value.into());
        self
    }

    pub fn condition_result(mut self, value: TestConditionResultCommonModel) -> Self {
        self.condition_result = Some(value);
        self
    }

    pub fn last_updated_at_unix(mut self, value: i64) -> Self {
        self.last_updated_at_unix = Some(value);
        self
    }

    pub fn metadata(mut self, value: TestRunMetadata) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn root_folder_id(mut self, value: impl Into<String>) -> Self {
        self.root_folder_id = Some(value.into());
        self
    }

    pub fn root_folder_name(mut self, value: impl Into<String>) -> Self {
        self.root_folder_name = Some(value.into());
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UnitTestRunResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`test_run_id`](UnitTestRunResponseModelBuilder::test_run_id)
    /// - [`test_invocation_id`](UnitTestRunResponseModelBuilder::test_invocation_id)
    /// - [`agent_id`](UnitTestRunResponseModelBuilder::agent_id)
    /// - [`status`](UnitTestRunResponseModelBuilder::status)
    /// - [`test_id`](UnitTestRunResponseModelBuilder::test_id)
    pub fn build(self) -> Result<UnitTestRunResponseModel, BuildError> {
        Ok(UnitTestRunResponseModel {
            test_run_id: self.test_run_id.ok_or_else(|| BuildError::missing_field("test_run_id"))?,
            test_info: self.test_info,
            test_invocation_id: self.test_invocation_id.ok_or_else(|| BuildError::missing_field("test_invocation_id"))?,
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            branch_id: self.branch_id,
            workflow_node_id: self.workflow_node_id,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            agent_responses: self.agent_responses,
            test_id: self.test_id.ok_or_else(|| BuildError::missing_field("test_id"))?,
            test_name: self.test_name,
            condition_result: self.condition_result,
            last_updated_at_unix: self.last_updated_at_unix,
            metadata: self.metadata,
            root_folder_id: self.root_folder_id,
            root_folder_name: self.root_folder_name,
            environment: self.environment,
        })
    }
}
