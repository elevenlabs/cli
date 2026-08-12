pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Simulation/preview-side config: tools are identified by IDs, resolved to names at runtime.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SimulationToolMockBehaviorConfig {
    /// Which tools to mock: 'all' mocks every mockable tool, 'selected' mocks only those in mocked_tool_names/mocked_tool_ids, 'none' disables mocking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mocking_strategy: Option<MockingStrategy>,
    /// Behavior when no mock matches a tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_strategy: Option<MockNoMatchBehavior>,
    /// Tool IDs to mock. Resolved to tool names before being passed to the orchestrator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mocked_tool_ids: Option<Vec<String>>,
}

impl SimulationToolMockBehaviorConfig {
    pub fn builder() -> SimulationToolMockBehaviorConfigBuilder {
        <SimulationToolMockBehaviorConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SimulationToolMockBehaviorConfigBuilder {
    mocking_strategy: Option<MockingStrategy>,
    fallback_strategy: Option<MockNoMatchBehavior>,
    mocked_tool_ids: Option<Vec<String>>,
}

impl SimulationToolMockBehaviorConfigBuilder {
    pub fn mocking_strategy(mut self, value: MockingStrategy) -> Self {
        self.mocking_strategy = Some(value);
        self
    }

    pub fn fallback_strategy(mut self, value: MockNoMatchBehavior) -> Self {
        self.fallback_strategy = Some(value);
        self
    }

    pub fn mocked_tool_ids(mut self, value: Vec<String>) -> Self {
        self.mocked_tool_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SimulationToolMockBehaviorConfig`].
    pub fn build(self) -> Result<SimulationToolMockBehaviorConfig, BuildError> {
        Ok(SimulationToolMockBehaviorConfig {
            mocking_strategy: self.mocking_strategy,
            fallback_strategy: self.fallback_strategy,
            mocked_tool_ids: self.mocked_tool_ids,
        })
    }
}
