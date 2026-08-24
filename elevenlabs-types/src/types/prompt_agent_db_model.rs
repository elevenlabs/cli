pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PromptAgentDbModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<serde_json::Value>,
}

impl PromptAgentDbModel {
    pub fn builder() -> PromptAgentDbModelBuilder {
        <PromptAgentDbModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PromptAgentDbModelBuilder {
    tools: Option<serde_json::Value>,
}

impl PromptAgentDbModelBuilder {
    pub fn tools(mut self, value: serde_json::Value) -> Self {
        self.tools = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PromptAgentDbModel`].
    pub fn build(self) -> Result<PromptAgentDbModel, BuildError> {
        Ok(PromptAgentDbModel {
            tools: self.tools,
        })
    }
}
