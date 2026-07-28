pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Allows the agent to explicitly skip its turn.
/// 
/// This tool should be invoked by the LLM when the user indicates they would like
/// to think or take a short pause before continuing the conversation—e.g. when
/// they say: "Give me a second", "Let me think", or "One moment please".  After
/// calling this tool, the assistant should not speak until the user speaks
/// again, or another normal turn-taking condition is met.  The tool itself has
/// no parameters and performs no side-effects other than informing the backend
/// that the current turn generation is complete.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SkipTurnToolConfig {
}

impl SkipTurnToolConfig {
    pub fn builder() -> SkipTurnToolConfigBuilder {
        <SkipTurnToolConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SkipTurnToolConfigBuilder {
}

impl SkipTurnToolConfigBuilder {

    /// Consumes the builder and constructs a [`SkipTurnToolConfig`].
    pub fn build(self) -> Result<SkipTurnToolConfig, BuildError> {
        Ok(SkipTurnToolConfig {
        })
    }
}
