pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TestRunMetadata {
    #[serde(default)]
    pub workspace_id: String,
    #[serde(default)]
    pub test_name: String,
    #[serde(default)]
    pub ran_by_user_email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_type: Option<TestRunMetadataTestType>,
}

impl TestRunMetadata {
    pub fn builder() -> TestRunMetadataBuilder {
        <TestRunMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TestRunMetadataBuilder {
    workspace_id: Option<String>,
    test_name: Option<String>,
    ran_by_user_email: Option<String>,
    test_type: Option<TestRunMetadataTestType>,
}

impl TestRunMetadataBuilder {
    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    pub fn test_name(mut self, value: impl Into<String>) -> Self {
        self.test_name = Some(value.into());
        self
    }

    pub fn ran_by_user_email(mut self, value: impl Into<String>) -> Self {
        self.ran_by_user_email = Some(value.into());
        self
    }

    pub fn test_type(mut self, value: TestRunMetadataTestType) -> Self {
        self.test_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TestRunMetadata`].
    /// This method will fail if any of the following fields are not set:
    /// - [`workspace_id`](TestRunMetadataBuilder::workspace_id)
    /// - [`test_name`](TestRunMetadataBuilder::test_name)
    /// - [`ran_by_user_email`](TestRunMetadataBuilder::ran_by_user_email)
    pub fn build(self) -> Result<TestRunMetadata, BuildError> {
        Ok(TestRunMetadata {
            workspace_id: self.workspace_id.ok_or_else(|| BuildError::missing_field("workspace_id"))?,
            test_name: self.test_name.ok_or_else(|| BuildError::missing_field("test_name"))?,
            ran_by_user_email: self.ran_by_user_email.ok_or_else(|| BuildError::missing_field("ran_by_user_email"))?,
            test_type: self.test_type,
        })
    }
}
