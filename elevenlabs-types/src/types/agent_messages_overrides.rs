pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentMessagesOverrides {
    /// The prompt for the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// The LLM to query with the prompt and the chat history. If using data residency, the LLM must be supported in the data residency environment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm: Option<Llm>,
    /// Language of the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// If set, only these tools are available in the conversation, including tools on workflow nodes. Each ID must be attached to the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_ids: Option<Vec<String>>,
    /// If set, the agent uses only these native MCP servers. Each ID must be attached to the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_mcp_server_ids: Option<Vec<String>>,
    /// If set, the agent uses only these knowledge base documents. Each ID must be attached to the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_ids: Option<Vec<String>>,
    /// If set, only these procedures are available to the starting agent. Each ID must be attached to that agent. An empty list disables all of that agent's procedures. Not applied after an agent transfer. Requires enable_procedure_ids_from_client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procedure_ids: Option<Vec<String>>,
    /// Extra fields for the request body that goes to the custom LLM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_llm_extra_body: Option<HashMap<String, serde_json::Value>>,
}

impl AgentMessagesOverrides {
    pub fn builder() -> AgentMessagesOverridesBuilder {
        <AgentMessagesOverridesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentMessagesOverridesBuilder {
    prompt: Option<String>,
    llm: Option<Llm>,
    language: Option<String>,
    tool_ids: Option<Vec<String>>,
    native_mcp_server_ids: Option<Vec<String>>,
    knowledge_base_ids: Option<Vec<String>>,
    procedure_ids: Option<Vec<String>>,
    custom_llm_extra_body: Option<HashMap<String, serde_json::Value>>,
}

impl AgentMessagesOverridesBuilder {
    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    pub fn llm(mut self, value: Llm) -> Self {
        self.llm = Some(value);
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
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

    pub fn knowledge_base_ids(mut self, value: Vec<String>) -> Self {
        self.knowledge_base_ids = Some(value);
        self
    }

    pub fn procedure_ids(mut self, value: Vec<String>) -> Self {
        self.procedure_ids = Some(value);
        self
    }

    pub fn custom_llm_extra_body(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.custom_llm_extra_body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentMessagesOverrides`].
    pub fn build(self) -> Result<AgentMessagesOverrides, BuildError> {
        Ok(AgentMessagesOverrides {
            prompt: self.prompt,
            llm: self.llm,
            language: self.language,
            tool_ids: self.tool_ids,
            native_mcp_server_ids: self.native_mcp_server_ids,
            knowledge_base_ids: self.knowledge_base_ids,
            procedure_ids: self.procedure_ids,
            custom_llm_extra_body: self.custom_llm_extra_body,
        })
    }
}
