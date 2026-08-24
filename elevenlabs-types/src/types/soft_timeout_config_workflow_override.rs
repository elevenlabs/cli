pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SoftTimeoutConfigWorkflowOverride {
    /// Time in seconds before showing the predefined message while waiting for LLM response. Set to -1 to disable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub timeout_seconds: Option<f64>,
    /// Message to show when the first soft timeout is reached while waiting for LLM response. Supports dynamic variables (e.g., {{system__time}}, {{custom_variable}}).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Extra static filler messages for subsequent soft timeouts in the same LLM generation. The first timeout uses `message`. If fewer messages are configured than `max_soft_timeouts_per_generation`, the last configured message is repeated; otherwise a built-in filler is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_soft_timeout_messages: Option<Vec<String>>,
    /// If enabled, the soft timeout message will be generated dynamically instead of using the static message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_llm_generated_message: Option<bool>,
    /// If enabled, shuffle the order of static soft timeout messages once at the start of each turn. Only applies when use_llm_generated_message is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub randomize_fillers: Option<bool>,
    /// Maximum filler messages while waiting for a single LLM response. Fires every timeout_seconds until the LLM streams content or this limit is reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_soft_timeouts_per_generation: Option<i64>,
    /// Custom prompt for generating the soft timeout filler message when use_llm_generated_message is enabled. Recent conversation context is provided as a separate user message. If not set, the default prompt will be used. Supports dynamic variables (e.g., {{system__time}}, {{custom_variable}}).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm_generated_message_prompt_override: Option<String>,
    /// When true, soft timeout fillers are suppressed until the conversation has at least one real user message. Prevents fillers during the agent's opening turn (e.g. workflow generate-immediately / tool calls before the user speaks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_until_first_user_message: Option<bool>,
}

impl SoftTimeoutConfigWorkflowOverride {
    pub fn builder() -> SoftTimeoutConfigWorkflowOverrideBuilder {
        <SoftTimeoutConfigWorkflowOverrideBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SoftTimeoutConfigWorkflowOverrideBuilder {
    timeout_seconds: Option<f64>,
    message: Option<String>,
    additional_soft_timeout_messages: Option<Vec<String>>,
    use_llm_generated_message: Option<bool>,
    randomize_fillers: Option<bool>,
    max_soft_timeouts_per_generation: Option<i64>,
    llm_generated_message_prompt_override: Option<String>,
    disable_until_first_user_message: Option<bool>,
}

impl SoftTimeoutConfigWorkflowOverrideBuilder {
    pub fn timeout_seconds(mut self, value: f64) -> Self {
        self.timeout_seconds = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn additional_soft_timeout_messages(mut self, value: Vec<String>) -> Self {
        self.additional_soft_timeout_messages = Some(value);
        self
    }

    pub fn use_llm_generated_message(mut self, value: bool) -> Self {
        self.use_llm_generated_message = Some(value);
        self
    }

    pub fn randomize_fillers(mut self, value: bool) -> Self {
        self.randomize_fillers = Some(value);
        self
    }

    pub fn max_soft_timeouts_per_generation(mut self, value: i64) -> Self {
        self.max_soft_timeouts_per_generation = Some(value);
        self
    }

    pub fn llm_generated_message_prompt_override(mut self, value: impl Into<String>) -> Self {
        self.llm_generated_message_prompt_override = Some(value.into());
        self
    }

    pub fn disable_until_first_user_message(mut self, value: bool) -> Self {
        self.disable_until_first_user_message = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SoftTimeoutConfigWorkflowOverride`].
    pub fn build(self) -> Result<SoftTimeoutConfigWorkflowOverride, BuildError> {
        Ok(SoftTimeoutConfigWorkflowOverride {
            timeout_seconds: self.timeout_seconds,
            message: self.message,
            additional_soft_timeout_messages: self.additional_soft_timeout_messages,
            use_llm_generated_message: self.use_llm_generated_message,
            randomize_fillers: self.randomize_fillers,
            max_soft_timeouts_per_generation: self.max_soft_timeouts_per_generation,
            llm_generated_message_prompt_override: self.llm_generated_message_prompt_override,
            disable_until_first_user_message: self.disable_until_first_user_message,
        })
    }
}
