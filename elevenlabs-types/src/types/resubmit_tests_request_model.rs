pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResubmitTestsRequestModel {
    /// List of test run IDs to resubmit
    #[serde(default)]
    pub test_run_ids: Vec<String>,
    /// Configuration overrides to use for testing. If not provided, the agent's default configuration will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_config_override: Option<AdhocAgentConfigOverrideForTestRequestModel>,
    /// Agent ID to resubmit tests for
    #[serde(default)]
    pub agent_id: String,
    /// ID of the branch to run the tests on. If not provided, the tests will be run on the agent's main branch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
}

impl ResubmitTestsRequestModel {
    pub fn builder() -> ResubmitTestsRequestModelBuilder {
        <ResubmitTestsRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResubmitTestsRequestModelBuilder {
    test_run_ids: Option<Vec<String>>,
    agent_config_override: Option<AdhocAgentConfigOverrideForTestRequestModel>,
    agent_id: Option<String>,
    branch_id: Option<String>,
}

impl ResubmitTestsRequestModelBuilder {
    pub fn test_run_ids(mut self, value: Vec<String>) -> Self {
        self.test_run_ids = Some(value);
        self
    }

    pub fn agent_config_override(mut self, value: AdhocAgentConfigOverrideForTestRequestModel) -> Self {
        self.agent_config_override = Some(value);
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

    /// Consumes the builder and constructs a [`ResubmitTestsRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`test_run_ids`](ResubmitTestsRequestModelBuilder::test_run_ids)
    /// - [`agent_id`](ResubmitTestsRequestModelBuilder::agent_id)
    pub fn build(self) -> Result<ResubmitTestsRequestModel, BuildError> {
        Ok(ResubmitTestsRequestModel {
            test_run_ids: self.test_run_ids.ok_or_else(|| BuildError::missing_field("test_run_ids"))?,
            agent_config_override: self.agent_config_override,
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            branch_id: self.branch_id,
        })
    }
}

