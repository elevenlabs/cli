pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Settings for agent testing configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentTestingSettings {
    /// List of test IDs that should be run for this agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_tests: Option<Vec<AttachedTestModel>>,
}

impl AgentTestingSettings {
    pub fn builder() -> AgentTestingSettingsBuilder {
        <AgentTestingSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentTestingSettingsBuilder {
    attached_tests: Option<Vec<AttachedTestModel>>,
}

impl AgentTestingSettingsBuilder {
    pub fn attached_tests(mut self, value: Vec<AttachedTestModel>) -> Self {
        self.attached_tests = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentTestingSettings`].
    pub fn build(self) -> Result<AgentTestingSettings, BuildError> {
        Ok(AgentTestingSettings {
            attached_tests: self.attached_tests,
        })
    }
}
