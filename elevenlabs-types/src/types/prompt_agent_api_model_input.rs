pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PromptAgentApiModelInput {
    /// The prompt for the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// The LLM to query with the prompt and the chat history. If using data residency, the LLM must be supported in the data residency environment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm: Option<Llm>,
    /// Reasoning effort of the model. Only available for some models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<LlmReasoningEffort>,
    /// Max number of tokens used for thinking. Use 0 to turn off if supported by the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking_budget: Option<i64>,
    /// Enable model reasoning summaries. When disabled, we do not request summaries from provider if possible for faster TTFB. Not ZRM compatible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_reasoning_summary: Option<bool>,
    /// The temperature for the LLM. Defaults to 0. Set to null to omit the parameter from the LLM request entirely (useful for custom LLMs that reject the temperature field).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub temperature: Option<f64>,
    /// If greater than 0, maximum number of tokens the LLM can predict
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i64>,
    /// A list of IDs of tools used by the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_ids: Option<Vec<String>>,
    /// Built-in system tools to be used by the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub built_in_tools: Option<BuiltInToolsInput>,
    /// A list of MCP server ids to be used by the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp_server_ids: Option<Vec<String>>,
    /// A list of Native MCP server ids to be used by the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_mcp_server_ids: Option<Vec<String>>,
    /// A list of knowledge bases to be used by the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base: Option<Vec<KnowledgeBaseLocator>>,
    /// Definition for a custom LLM if LLM field is set to 'CUSTOM_LLM'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_llm: Option<CustomLlm>,
    /// Whether to remove the default personality lines from the system prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_default_personality: Option<bool>,
    /// Configuration for RAG
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rag: Option<RagConfigInput>,
    /// Timezone for displaying current time in system prompt. If set, the current time will be included in the system prompt using this timezone. Must be a valid timezone name (e.g., 'America/New_York', 'Europe/London', 'UTC'). Recommended for accurate time-aware responses; without this, the agent has no knowledge of the current date/time unless you provide it via dynamic variables or tools, which can lead to incorrect or hallucinated time references.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Configuration for backup LLM cascading. Can be disabled, use system defaults, or specify custom order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_llm_config: Option<PromptAgentApiModelInputBackupLlmConfig>,
    /// Time in seconds before cascading to backup LLM. Must be between 2 and 15 seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cascade_timeout_seconds: Option<f64>,
    /// A list of tools that the agent can use over the course of the conversation, use tool_ids instead
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<PromptAgentApiModelInputToolsItem>>,
}

impl PromptAgentApiModelInput {
    pub fn builder() -> PromptAgentApiModelInputBuilder {
        <PromptAgentApiModelInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PromptAgentApiModelInputBuilder {
    prompt: Option<String>,
    llm: Option<Llm>,
    reasoning_effort: Option<LlmReasoningEffort>,
    thinking_budget: Option<i64>,
    enable_reasoning_summary: Option<bool>,
    temperature: Option<f64>,
    max_tokens: Option<i64>,
    tool_ids: Option<Vec<String>>,
    built_in_tools: Option<BuiltInToolsInput>,
    mcp_server_ids: Option<Vec<String>>,
    native_mcp_server_ids: Option<Vec<String>>,
    knowledge_base: Option<Vec<KnowledgeBaseLocator>>,
    custom_llm: Option<CustomLlm>,
    ignore_default_personality: Option<bool>,
    rag: Option<RagConfigInput>,
    timezone: Option<String>,
    backup_llm_config: Option<PromptAgentApiModelInputBackupLlmConfig>,
    cascade_timeout_seconds: Option<f64>,
    tools: Option<Vec<PromptAgentApiModelInputToolsItem>>,
}

impl PromptAgentApiModelInputBuilder {
    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    pub fn llm(mut self, value: Llm) -> Self {
        self.llm = Some(value);
        self
    }

    pub fn reasoning_effort(mut self, value: LlmReasoningEffort) -> Self {
        self.reasoning_effort = Some(value);
        self
    }

    pub fn thinking_budget(mut self, value: i64) -> Self {
        self.thinking_budget = Some(value);
        self
    }

    pub fn enable_reasoning_summary(mut self, value: bool) -> Self {
        self.enable_reasoning_summary = Some(value);
        self
    }

    pub fn temperature(mut self, value: f64) -> Self {
        self.temperature = Some(value);
        self
    }

    pub fn max_tokens(mut self, value: i64) -> Self {
        self.max_tokens = Some(value);
        self
    }

    pub fn tool_ids(mut self, value: Vec<String>) -> Self {
        self.tool_ids = Some(value);
        self
    }

    pub fn built_in_tools(mut self, value: BuiltInToolsInput) -> Self {
        self.built_in_tools = Some(value);
        self
    }

    pub fn mcp_server_ids(mut self, value: Vec<String>) -> Self {
        self.mcp_server_ids = Some(value);
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

    pub fn custom_llm(mut self, value: CustomLlm) -> Self {
        self.custom_llm = Some(value);
        self
    }

    pub fn ignore_default_personality(mut self, value: bool) -> Self {
        self.ignore_default_personality = Some(value);
        self
    }

    pub fn rag(mut self, value: RagConfigInput) -> Self {
        self.rag = Some(value);
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    pub fn backup_llm_config(mut self, value: PromptAgentApiModelInputBackupLlmConfig) -> Self {
        self.backup_llm_config = Some(value);
        self
    }

    pub fn cascade_timeout_seconds(mut self, value: f64) -> Self {
        self.cascade_timeout_seconds = Some(value);
        self
    }

    pub fn tools(mut self, value: Vec<PromptAgentApiModelInputToolsItem>) -> Self {
        self.tools = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PromptAgentApiModelInput`].
    pub fn build(self) -> Result<PromptAgentApiModelInput, BuildError> {
        Ok(PromptAgentApiModelInput {
            prompt: self.prompt,
            llm: self.llm,
            reasoning_effort: self.reasoning_effort,
            thinking_budget: self.thinking_budget,
            enable_reasoning_summary: self.enable_reasoning_summary,
            temperature: self.temperature,
            max_tokens: self.max_tokens,
            tool_ids: self.tool_ids,
            built_in_tools: self.built_in_tools,
            mcp_server_ids: self.mcp_server_ids,
            native_mcp_server_ids: self.native_mcp_server_ids,
            knowledge_base: self.knowledge_base,
            custom_llm: self.custom_llm,
            ignore_default_personality: self.ignore_default_personality,
            rag: self.rag,
            timezone: self.timezone,
            backup_llm_config: self.backup_llm_config,
            cascade_timeout_seconds: self.cascade_timeout_seconds,
            tools: self.tools,
        })
    }
}
