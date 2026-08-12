pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PromptAgentApiModelOverrideInput {
    /// The prompt for the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// The LLM to query with the prompt and the chat history. If using data residency, the LLM must be supported in the data residency environment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm: Option<Llm>,
    /// A list of IDs of tools used by the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_ids: Option<Vec<String>>,
    /// A list of Native MCP server ids to be used by the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_mcp_server_ids: Option<Vec<String>>,
    /// A list of knowledge bases to be used by the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base: Option<Vec<KnowledgeBaseLocator>>,
}

impl PromptAgentApiModelOverrideInput {
    pub fn builder() -> PromptAgentApiModelOverrideInputBuilder {
        <PromptAgentApiModelOverrideInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PromptAgentApiModelOverrideInputBuilder {
    prompt: Option<String>,
    llm: Option<Llm>,
    tool_ids: Option<Vec<String>>,
    native_mcp_server_ids: Option<Vec<String>>,
    knowledge_base: Option<Vec<KnowledgeBaseLocator>>,
}

impl PromptAgentApiModelOverrideInputBuilder {
    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    pub fn llm(mut self, value: Llm) -> Self {
        self.llm = Some(value);
        self
    }

    pub fn tool_ids(mut self, value: Vec<String>) -> Self {
        self.tool_ids = Some(value);
        self
    }

    pub fn native_mcp_server_ids(mut self, value: Vec<String>) -> Self {
        self.native_mcp_server_ids = Some(value);
        self
    }

    pub fn knowledge_base(mut self, value: Vec<KnowledgeBaseLocator>) -> Self {
        self.knowledge_base = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PromptAgentApiModelOverrideInput`].
    pub fn build(self) -> Result<PromptAgentApiModelOverrideInput, BuildError> {
        Ok(PromptAgentApiModelOverrideInput {
            prompt: self.prompt,
            llm: self.llm,
            tool_ids: self.tool_ids,
            native_mcp_server_ids: self.native_mcp_server_ids,
            knowledge_base: self.knowledge_base,
        })
    }
}
