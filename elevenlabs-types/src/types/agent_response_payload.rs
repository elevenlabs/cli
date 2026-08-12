pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Text chunk sent from your server to ElevenLabs for speech synthesis.
/// Stream LLM output by sending multiple messages with `is_final: false`, then
/// terminate the response with a message where `is_final: true` and `content` is
/// an empty string.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentResponsePayload {
    /// The message type identifier.
    pub r#type: String,
    /// The text to synthesize. For streaming responses, send incremental chunks here.
    /// The final message in a response must have an empty string (`""`).
    #[serde(default)]
    pub content: String,
    /// The `event_id` from the `user_transcript` this response addresses. ElevenLabs
    /// uses this to discard responses that belong to an interrupted turn.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<i64>,
    /// Set to `true` on the last message of a response (with an empty `content`).
    /// Set to `false` on all preceding chunks.
    #[serde(default)]
    pub is_final: bool,
}

impl AgentResponsePayload {
    pub fn builder() -> AgentResponsePayloadBuilder {
        <AgentResponsePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentResponsePayloadBuilder {
    r#type: Option<String>,
    content: Option<String>,
    event_id: Option<i64>,
    is_final: Option<bool>,
}

impl AgentResponsePayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn event_id(mut self, value: i64) -> Self {
        self.event_id = Some(value);
        self
    }

    pub fn is_final(mut self, value: bool) -> Self {
        self.is_final = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentResponsePayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](AgentResponsePayloadBuilder::r#type)
    /// - [`content`](AgentResponsePayloadBuilder::content)
    /// - [`is_final`](AgentResponsePayloadBuilder::is_final)
    pub fn build(self) -> Result<AgentResponsePayload, BuildError> {
        Ok(AgentResponsePayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            event_id: self.event_id,
            is_final: self.is_final.ok_or_else(|| BuildError::missing_field("is_final"))?,
        })
    }
}
