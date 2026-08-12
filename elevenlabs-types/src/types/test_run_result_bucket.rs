pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TestRunResultBucket {
    #[serde(default)]
    pub test_run_ids: Vec<String>,
    /// Short one-line title for this bucket
    #[serde(default)]
    pub title: String,
    /// Short summary of why the test runs in this bucket passed or failed
    #[serde(default)]
    pub reason: String,
    pub status: TestRunStatus,
}

impl TestRunResultBucket {
    pub fn builder() -> TestRunResultBucketBuilder {
        <TestRunResultBucketBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TestRunResultBucketBuilder {
    test_run_ids: Option<Vec<String>>,
    title: Option<String>,
    reason: Option<String>,
    status: Option<TestRunStatus>,
}

impl TestRunResultBucketBuilder {
    pub fn test_run_ids(mut self, value: Vec<String>) -> Self {
        self.test_run_ids = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn status(mut self, value: TestRunStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TestRunResultBucket`].
    /// This method will fail if any of the following fields are not set:
    /// - [`test_run_ids`](TestRunResultBucketBuilder::test_run_ids)
    /// - [`title`](TestRunResultBucketBuilder::title)
    /// - [`reason`](TestRunResultBucketBuilder::reason)
    /// - [`status`](TestRunResultBucketBuilder::status)
    pub fn build(self) -> Result<TestRunResultBucket, BuildError> {
        Ok(TestRunResultBucket {
            test_run_ids: self.test_run_ids.ok_or_else(|| BuildError::missing_field("test_run_ids"))?,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            reason: self.reason.ok_or_else(|| BuildError::missing_field("reason"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
