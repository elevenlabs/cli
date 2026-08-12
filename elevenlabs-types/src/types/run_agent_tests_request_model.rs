pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RunAgentTestsRequestModel {
    /// List of tests to run on the agent
    #[serde(default)]
    pub tests: Vec<SingleTestRunRequestModel>,
    /// Configuration overrides to use for testing. If not provided, the agent's default configuration will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_config_override: Option<AdhocAgentConfigOverrideForTestRequestModel>,
    /// ID of the branch to run the tests on. If not provided, the tests will be run on the agent's main branch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    /// Number of times to run each test. When greater than 1, results are grouped and summarized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_count: Option<i64>,
}

impl RunAgentTestsRequestModel {
    pub fn builder() -> RunAgentTestsRequestModelBuilder {
        <RunAgentTestsRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunAgentTestsRequestModelBuilder {
    tests: Option<Vec<SingleTestRunRequestModel>>,
    agent_config_override: Option<AdhocAgentConfigOverrideForTestRequestModel>,
    branch_id: Option<String>,
    repeat_count: Option<i64>,
}

impl RunAgentTestsRequestModelBuilder {
    pub fn tests(mut self, value: Vec<SingleTestRunRequestModel>) -> Self {
        self.tests = Some(value);
        self
    }

    pub fn agent_config_override(mut self, value: AdhocAgentConfigOverrideForTestRequestModel) -> Self {
        self.agent_config_override = Some(value);
        self
    }

    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    pub fn repeat_count(mut self, value: i64) -> Self {
        self.repeat_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RunAgentTestsRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tests`](RunAgentTestsRequestModelBuilder::tests)
    pub fn build(self) -> Result<RunAgentTestsRequestModel, BuildError> {
        Ok(RunAgentTestsRequestModel {
            tests: self.tests.ok_or_else(|| BuildError::missing_field("tests"))?,
            agent_config_override: self.agent_config_override,
            branch_id: self.branch_id,
            repeat_count: self.repeat_count,
        })
    }
}

