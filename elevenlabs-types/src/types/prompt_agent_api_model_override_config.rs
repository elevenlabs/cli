pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PromptAgentApiModelOverrideConfig {
    /// Whether to allow overriding the prompt field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<bool>,
    /// Whether to allow overriding the llm field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm: Option<bool>,
    /// Whether to allow overriding the tool_ids field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_ids: Option<bool>,
    /// Whether to allow overriding the native_mcp_server_ids field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_mcp_server_ids: Option<bool>,
    /// Whether to allow overriding the knowledge_base field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base: Option<bool>,
}

impl PromptAgentApiModelOverrideConfig {
    pub fn builder() -> PromptAgentApiModelOverrideConfigBuilder {
        <PromptAgentApiModelOverrideConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PromptAgentApiModelOverrideConfigBuilder {
    prompt: Option<bool>,
    llm: Option<bool>,
    tool_ids: Option<bool>,
    native_mcp_server_ids: Option<bool>,
    knowledge_base: Option<bool>,
}

impl PromptAgentApiModelOverrideConfigBuilder {
    pub fn prompt(mut self, value: bool) -> Self {
        self.prompt = Some(value);
        self
    }

    pub fn llm(mut self, value: bool) -> Self {
        self.llm = Some(value);
        self
    }

    pub fn tool_ids(mut self, value: bool) -> Self {
        self.tool_ids = Some(value);
        self
    }

    pub fn native_mcp_server_ids(mut self, value: bool) -> Self {
        self.native_mcp_server_ids = Some(value);
        self
    }

    pub fn knowledge_base(mut self, value: bool) -> Self {
        self.knowledge_base = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PromptAgentApiModelOverrideConfig`].
    pub fn build(self) -> Result<PromptAgentApiModelOverrideConfig, BuildError> {
        Ok(PromptAgentApiModelOverrideConfig {
            prompt: self.prompt,
            llm: self.llm,
            tool_ids: self.tool_ids,
            native_mcp_server_ids: self.native_mcp_server_ids,
            knowledge_base: self.knowledge_base,
        })
    }
}
