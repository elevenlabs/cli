pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TestRunResultSummary {
    #[serde(default)]
    pub test_id: String,
    #[serde(default)]
    pub test_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_node_id: Option<String>,
    #[serde(default)]
    pub buckets: Vec<TestRunResultBucket>,
}

impl TestRunResultSummary {
    pub fn builder() -> TestRunResultSummaryBuilder {
        <TestRunResultSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TestRunResultSummaryBuilder {
    test_id: Option<String>,
    test_name: Option<String>,
    workflow_node_id: Option<String>,
    buckets: Option<Vec<TestRunResultBucket>>,
}

impl TestRunResultSummaryBuilder {
    pub fn test_id(mut self, value: impl Into<String>) -> Self {
        self.test_id = Some(value.into());
        self
    }

    pub fn test_name(mut self, value: impl Into<String>) -> Self {
        self.test_name = Some(value.into());
        self
    }

    pub fn workflow_node_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_node_id = Some(value.into());
        self
    }

    pub fn buckets(mut self, value: Vec<TestRunResultBucket>) -> Self {
        self.buckets = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TestRunResultSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`test_id`](TestRunResultSummaryBuilder::test_id)
    /// - [`test_name`](TestRunResultSummaryBuilder::test_name)
    /// - [`buckets`](TestRunResultSummaryBuilder::buckets)
    pub fn build(self) -> Result<TestRunResultSummary, BuildError> {
        Ok(TestRunResultSummary {
            test_id: self.test_id.ok_or_else(|| BuildError::missing_field("test_id"))?,
            test_name: self.test_name.ok_or_else(|| BuildError::missing_field("test_name"))?,
            workflow_node_id: self.workflow_node_id,
            buckets: self.buckets.ok_or_else(|| BuildError::missing_field("buckets"))?,
        })
    }
}
