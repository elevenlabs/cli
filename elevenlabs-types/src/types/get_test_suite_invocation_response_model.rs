pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetTestSuiteInvocationResponseModel {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_count: Option<i64>,
    /// None when repeat_count==1 (no bucketing). Otherwise tracks bucketing lifecycle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucketing_status: Option<BucketingStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_groups: Option<Vec<TestRunResultSummary>>,
    #[serde(default)]
    pub test_runs: Vec<UnitTestRunResponseModel>,
}

impl GetTestSuiteInvocationResponseModel {
    pub fn builder() -> GetTestSuiteInvocationResponseModelBuilder {
        <GetTestSuiteInvocationResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTestSuiteInvocationResponseModelBuilder {
    id: Option<String>,
    agent_id: Option<String>,
    branch_id: Option<String>,
    created_at: Option<i64>,
    folder_id: Option<String>,
    repeat_count: Option<i64>,
    bucketing_status: Option<BucketingStatus>,
    result_groups: Option<Vec<TestRunResultSummary>>,
    test_runs: Option<Vec<UnitTestRunResponseModel>>,
}

impl GetTestSuiteInvocationResponseModelBuilder {
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

    pub fn created_at(mut self, value: i64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn folder_id(mut self, value: impl Into<String>) -> Self {
        self.folder_id = Some(value.into());
        self
    }

    pub fn repeat_count(mut self, value: i64) -> Self {
        self.repeat_count = Some(value);
        self
    }

    pub fn bucketing_status(mut self, value: BucketingStatus) -> Self {
        self.bucketing_status = Some(value);
        self
    }

    pub fn result_groups(mut self, value: Vec<TestRunResultSummary>) -> Self {
        self.result_groups = Some(value);
        self
    }

    pub fn test_runs(mut self, value: Vec<UnitTestRunResponseModel>) -> Self {
        self.test_runs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetTestSuiteInvocationResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetTestSuiteInvocationResponseModelBuilder::id)
    /// - [`test_runs`](GetTestSuiteInvocationResponseModelBuilder::test_runs)
    pub fn build(self) -> Result<GetTestSuiteInvocationResponseModel, BuildError> {
        Ok(GetTestSuiteInvocationResponseModel {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            agent_id: self.agent_id,
            branch_id: self.branch_id,
            created_at: self.created_at,
            folder_id: self.folder_id,
            repeat_count: self.repeat_count,
            bucketing_status: self.bucketing_status,
            result_groups: self.result_groups,
            test_runs: self.test_runs.ok_or_else(|| BuildError::missing_field("test_runs"))?,
        })
    }
}
