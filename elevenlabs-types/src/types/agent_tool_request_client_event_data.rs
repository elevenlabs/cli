pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentToolRequestClientEventData {
    #[serde(default)]
    pub tool_name: String,
    #[serde(default)]
    pub tool_call_id: String,
    #[serde(default)]
    pub tool_type: String,
    #[serde(default)]
    pub event_id: i64,
    #[serde(default)]
    pub expects_response: bool,
    #[serde(default)]
    pub disable_interruptions: bool,
    #[serde(default)]
    pub response_timeout_secs: i64,
    #[serde(default)]
    pub execution_mode: String,
}

impl AgentToolRequestClientEventData {
    pub fn builder() -> AgentToolRequestClientEventDataBuilder {
        <AgentToolRequestClientEventDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentToolRequestClientEventDataBuilder {
    tool_name: Option<String>,
    tool_call_id: Option<String>,
    tool_type: Option<String>,
    event_id: Option<i64>,
    expects_response: Option<bool>,
    disable_interruptions: Option<bool>,
    response_timeout_secs: Option<i64>,
    execution_mode: Option<String>,
}

impl AgentToolRequestClientEventDataBuilder {
    pub fn tool_name(mut self, value: impl Into<String>) -> Self {
        self.tool_name = Some(value.into());
        self
    }

    pub fn tool_call_id(mut self, value: impl Into<String>) -> Self {
        self.tool_call_id = Some(value.into());
        self
    }

    pub fn tool_type(mut self, value: impl Into<String>) -> Self {
        self.tool_type = Some(value.into());
        self
    }

    pub fn event_id(mut self, value: i64) -> Self {
        self.event_id = Some(value);
        self
    }

    pub fn expects_response(mut self, value: bool) -> Self {
        self.expects_response = Some(value);
        self
    }

    pub fn disable_interruptions(mut self, value: bool) -> Self {
        self.disable_interruptions = Some(value);
        self
    }

    pub fn response_timeout_secs(mut self, value: i64) -> Self {
        self.response_timeout_secs = Some(value);
        self
    }

    pub fn execution_mode(mut self, value: impl Into<String>) -> Self {
        self.execution_mode = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentToolRequestClientEventData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tool_name`](AgentToolRequestClientEventDataBuilder::tool_name)
    /// - [`tool_call_id`](AgentToolRequestClientEventDataBuilder::tool_call_id)
    /// - [`tool_type`](AgentToolRequestClientEventDataBuilder::tool_type)
    /// - [`event_id`](AgentToolRequestClientEventDataBuilder::event_id)
    /// - [`expects_response`](AgentToolRequestClientEventDataBuilder::expects_response)
    /// - [`disable_interruptions`](AgentToolRequestClientEventDataBuilder::disable_interruptions)
    /// - [`response_timeout_secs`](AgentToolRequestClientEventDataBuilder::response_timeout_secs)
    /// - [`execution_mode`](AgentToolRequestClientEventDataBuilder::execution_mode)
    pub fn build(self) -> Result<AgentToolRequestClientEventData, BuildError> {
        Ok(AgentToolRequestClientEventData {
            tool_name: self.tool_name.ok_or_else(|| BuildError::missing_field("tool_name"))?,
            tool_call_id: self.tool_call_id.ok_or_else(|| BuildError::missing_field("tool_call_id"))?,
            tool_type: self.tool_type.ok_or_else(|| BuildError::missing_field("tool_type"))?,
            event_id: self.event_id.ok_or_else(|| BuildError::missing_field("event_id"))?,
            expects_response: self.expects_response.ok_or_else(|| BuildError::missing_field("expects_response"))?,
            disable_interruptions: self.disable_interruptions.ok_or_else(|| BuildError::missing_field("disable_interruptions"))?,
            response_timeout_secs: self.response_timeout_secs.ok_or_else(|| BuildError::missing_field("response_timeout_secs"))?,
            execution_mode: self.execution_mode.ok_or_else(|| BuildError::missing_field("execution_mode"))?,
        })
    }
}
