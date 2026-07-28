pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct McpApprovalPolicyUpdateRequestModel {
    /// The approval mode to set for the MCP server
    pub approval_policy: McpApprovalPolicy,
}

impl McpApprovalPolicyUpdateRequestModel {
    pub fn builder() -> McpApprovalPolicyUpdateRequestModelBuilder {
        <McpApprovalPolicyUpdateRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpApprovalPolicyUpdateRequestModelBuilder {
    approval_policy: Option<McpApprovalPolicy>,
}

impl McpApprovalPolicyUpdateRequestModelBuilder {
    pub fn approval_policy(mut self, value: McpApprovalPolicy) -> Self {
        self.approval_policy = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`McpApprovalPolicyUpdateRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`approval_policy`](McpApprovalPolicyUpdateRequestModelBuilder::approval_policy)
    pub fn build(self) -> Result<McpApprovalPolicyUpdateRequestModel, BuildError> {
        Ok(McpApprovalPolicyUpdateRequestModel {
            approval_policy: self.approval_policy.ok_or_else(|| BuildError::missing_field("approval_policy"))?,
        })
    }
}

