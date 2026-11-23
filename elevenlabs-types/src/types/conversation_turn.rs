pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConversationTurn {
    #[serde(default)]
    pub turn_id: String,
    pub status: AgentOutputStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<Vec<TextOutputItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<AgentOutputError>,
    /// The actions the agent waits on. Answer each `client_tool_call` with a `client_tool_result` item in `input` to continue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_actions: Option<Vec<ClientToolCallRequiredAction>>,
    /// Resume token for this turn's event stream. Present while the turn is in progress; pass it as Last-Event-ID to stream the turn's events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_cursor: Option<String>,
}

impl ConversationTurn {
    pub fn builder() -> ConversationTurnBuilder {
        <ConversationTurnBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationTurnBuilder {
    turn_id: Option<String>,
    status: Option<AgentOutputStatus>,
    output: Option<Vec<TextOutputItem>>,
    error: Option<AgentOutputError>,
    required_actions: Option<Vec<ClientToolCallRequiredAction>>,
    stream_cursor: Option<String>,
}

impl ConversationTurnBuilder {
    pub fn turn_id(mut self, value: impl Into<String>) -> Self {
        self.turn_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: AgentOutputStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn output(mut self, value: Vec<TextOutputItem>) -> Self {
        self.output = Some(value);
        self
    }

    pub fn error(mut self, value: AgentOutputError) -> Self {
        self.error = Some(value);
        self
    }

    pub fn required_actions(mut self, value: Vec<ClientToolCallRequiredAction>) -> Self {
        self.required_actions = Some(value);
        self
    }

    pub fn stream_cursor(mut self, value: impl Into<String>) -> Self {
        self.stream_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationTurn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`turn_id`](ConversationTurnBuilder::turn_id)
    /// - [`status`](ConversationTurnBuilder::status)
    pub fn build(self) -> Result<ConversationTurn, BuildError> {
        Ok(ConversationTurn {
            turn_id: self.turn_id.ok_or_else(|| BuildError::missing_field("turn_id"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            output: self.output,
            error: self.error,
            required_actions: self.required_actions,
            stream_cursor: self.stream_cursor,
        })
    }
}
