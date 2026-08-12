pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SingleTestRunRequestModel {
    /// ID of the test to run
    #[serde(default)]
    pub test_id: String,
    /// ID of the workflow node to run the test on. If not provided, the test will be run on the agent's default workflow node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_node_id: Option<String>,
    /// ID of the root folder to run the test on. If not provided, the test will be run on the agent's default folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_folder_id: Option<String>,
    /// Name of the root folder to run the test on. If not provided, the test will be run on the agent's default folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_folder_name: Option<String>,
}

impl SingleTestRunRequestModel {
    pub fn builder() -> SingleTestRunRequestModelBuilder {
        <SingleTestRunRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SingleTestRunRequestModelBuilder {
    test_id: Option<String>,
    workflow_node_id: Option<String>,
    root_folder_id: Option<String>,
    root_folder_name: Option<String>,
}

impl SingleTestRunRequestModelBuilder {
    pub fn test_id(mut self, value: impl Into<String>) -> Self {
        self.test_id = Some(value.into());
        self
    }

    pub fn workflow_node_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_node_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`SingleTestRunRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`test_id`](SingleTestRunRequestModelBuilder::test_id)
    pub fn build(self) -> Result<SingleTestRunRequestModel, BuildError> {
        Ok(SingleTestRunRequestModel {
            test_id: self.test_id.ok_or_else(|| BuildError::missing_field("test_id"))?,
            workflow_node_id: self.workflow_node_id,
            root_folder_id: self.root_folder_id,
            root_folder_name: self.root_folder_name,
        })
    }
}
