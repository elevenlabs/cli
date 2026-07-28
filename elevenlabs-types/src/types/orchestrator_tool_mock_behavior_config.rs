pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Orchestrator-side config: tools are identified by resolved names.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrchestratorToolMockBehaviorConfig {
    /// Which tools to mock: 'all' mocks every mockable tool, 'selected' mocks only those in mocked_tool_names/mocked_tool_ids, 'none' disables mocking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mocking_strategy: Option<MockingStrategy>,
    /// Behavior when no mock matches a tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_strategy: Option<MockNoMatchBehavior>,
    /// Tool names to mock. Only used when mocking_strategy is 'selected'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mocked_tool_names: Option<Vec<String>>,
}

impl OrchestratorToolMockBehaviorConfig {
    pub fn builder() -> OrchestratorToolMockBehaviorConfigBuilder {
        <OrchestratorToolMockBehaviorConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrchestratorToolMockBehaviorConfigBuilder {
    mocking_strategy: Option<MockingStrategy>,
    fallback_strategy: Option<MockNoMatchBehavior>,
    mocked_tool_names: Option<Vec<String>>,
}

impl OrchestratorToolMockBehaviorConfigBuilder {
    pub fn mocking_strategy(mut self, value: MockingStrategy) -> Self {
        self.mocking_strategy = Some(value);
        self
    }

    pub fn fallback_strategy(mut self, value: MockNoMatchBehavior) -> Self {
        self.fallback_strategy = Some(value);
        self
    }

    pub fn mocked_tool_names(mut self, value: Vec<String>) -> Self {
        self.mocked_tool_names = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrchestratorToolMockBehaviorConfig`].
    pub fn build(self) -> Result<OrchestratorToolMockBehaviorConfig, BuildError> {
        Ok(OrchestratorToolMockBehaviorConfig {
            mocking_strategy: self.mocking_strategy,
            fallback_strategy: self.fallback_strategy,
            mocked_tool_names: self.mocked_tool_names,
        })
    }
}
