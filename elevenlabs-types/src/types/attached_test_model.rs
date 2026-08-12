pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AttachedTestModel {
    #[serde(default)]
    pub test_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_node_id: Option<String>,
}

impl AttachedTestModel {
    pub fn builder() -> AttachedTestModelBuilder {
        <AttachedTestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AttachedTestModelBuilder {
    test_id: Option<String>,
    workflow_node_id: Option<String>,
}

impl AttachedTestModelBuilder {
    pub fn test_id(mut self, value: impl Into<String>) -> Self {
        self.test_id = Some(value.into());
        self
    }

    pub fn workflow_node_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_node_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AttachedTestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`test_id`](AttachedTestModelBuilder::test_id)
    pub fn build(self) -> Result<AttachedTestModel, BuildError> {
        Ok(AttachedTestModel {
            test_id: self.test_id.ok_or_else(|| BuildError::missing_field("test_id"))?,
            workflow_node_id: self.workflow_node_id,
        })
    }
}
