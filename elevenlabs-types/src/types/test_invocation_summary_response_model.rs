pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TestInvocationSummaryResponseModel {
    /// The ID of the test invocation
    #[serde(default)]
    pub id: String,
    /// The ID of the agent this test invocation belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// The ID of the branch this test invocation was run on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    /// Creation time of the test invocation in unix seconds
    #[serde(default)]
    pub created_at_unix_secs: i64,
    /// Number of test runs in this invocation
    #[serde(default)]
    pub test_run_count: i64,
    /// Number of test runs that passed
    #[serde(default)]
    pub passed_count: i64,
    /// Number of test runs that failed
    #[serde(default)]
    pub failed_count: i64,
    /// Number of test runs that are pending
    #[serde(default)]
    pub pending_count: i64,
    /// Title of the test invocation - the folder name for folder runs, otherwise the single test name or count of tests
    #[serde(default)]
    pub title: String,
    /// The access information of the test invocation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_info: Option<ResourceAccessInfo>,
    /// Number of times each test was repeated in this invocation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_count: Option<i64>,
}

impl TestInvocationSummaryResponseModel {
    pub fn builder() -> TestInvocationSummaryResponseModelBuilder {
        <TestInvocationSummaryResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TestInvocationSummaryResponseModelBuilder {
    id: Option<String>,
    agent_id: Option<String>,
    branch_id: Option<String>,
    created_at_unix_secs: Option<i64>,
    test_run_count: Option<i64>,
    passed_count: Option<i64>,
    failed_count: Option<i64>,
    pending_count: Option<i64>,
    title: Option<String>,
    access_info: Option<ResourceAccessInfo>,
    repeat_count: Option<i64>,
}

impl TestInvocationSummaryResponseModelBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    pub fn created_at_unix_secs(mut self, value: i64) -> Self {
        self.created_at_unix_secs = Some(value);
        self
    }

    pub fn test_run_count(mut self, value: i64) -> Self {
        self.test_run_count = Some(value);
        self
    }

    pub fn passed_count(mut self, value: i64) -> Self {
        self.passed_count = Some(value);
        self
    }

    pub fn failed_count(mut self, value: i64) -> Self {
        self.failed_count = Some(value);
        self
    }

    pub fn pending_count(mut self, value: i64) -> Self {
        self.pending_count = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn access_info(mut self, value: ResourceAccessInfo) -> Self {
        self.access_info = Some(value);
        self
    }

    pub fn repeat_count(mut self, value: i64) -> Self {
        self.repeat_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TestInvocationSummaryResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TestInvocationSummaryResponseModelBuilder::id)
    /// - [`created_at_unix_secs`](TestInvocationSummaryResponseModelBuilder::created_at_unix_secs)
    /// - [`test_run_count`](TestInvocationSummaryResponseModelBuilder::test_run_count)
    /// - [`passed_count`](TestInvocationSummaryResponseModelBuilder::passed_count)
    /// - [`failed_count`](TestInvocationSummaryResponseModelBuilder::failed_count)
    /// - [`pending_count`](TestInvocationSummaryResponseModelBuilder::pending_count)
    /// - [`title`](TestInvocationSummaryResponseModelBuilder::title)
    pub fn build(self) -> Result<TestInvocationSummaryResponseModel, BuildError> {
        Ok(TestInvocationSummaryResponseModel {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            agent_id: self.agent_id,
            branch_id: self.branch_id,
            created_at_unix_secs: self.created_at_unix_secs.ok_or_else(|| BuildError::missing_field("created_at_unix_secs"))?,
            test_run_count: self.test_run_count.ok_or_else(|| BuildError::missing_field("test_run_count"))?,
            passed_count: self.passed_count.ok_or_else(|| BuildError::missing_field("passed_count"))?,
            failed_count: self.failed_count.ok_or_else(|| BuildError::missing_field("failed_count"))?,
            pending_count: self.pending_count.ok_or_else(|| BuildError::missing_field("pending_count"))?,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            access_info: self.access_info,
            repeat_count: self.repeat_count,
        })
    }
}
