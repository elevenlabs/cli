pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Allows the agent to explicitly skip its turn.
/// 
/// This tool should be invoked by the LLM when the user indicates they would like
/// to think or take a short pause before continuing the conversation—e.g. when
/// they say: "Give me a second", "Let me think", or "One moment please".  After
/// calling this tool, the assistant should not speak until the user speaks
/// again, or if wait_timeout_secs is set, until that wait elapses and the
/// agent generates a check-in.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SkipTurnToolConfig {
    /// Seconds to wait after skip_turn before the agent generates a contextual check-in. The "End conversation after silence" timer is paused during the wait. -1 disables the wait: after skip_turn the agent stays silent until the caller speaks, and that timer keeps running. Applies to voice conversations only.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub wait_timeout_secs: Option<f64>,
}

impl SkipTurnToolConfig {
    pub fn builder() -> SkipTurnToolConfigBuilder {
        <SkipTurnToolConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SkipTurnToolConfigBuilder {
    wait_timeout_secs: Option<f64>,
}

impl SkipTurnToolConfigBuilder {
    pub fn wait_timeout_secs(mut self, value: f64) -> Self {
        self.wait_timeout_secs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SkipTurnToolConfig`].
    pub fn build(self) -> Result<SkipTurnToolConfig, BuildError> {
        Ok(SkipTurnToolConfig {
            wait_timeout_secs: self.wait_timeout_secs,
        })
    }
}
