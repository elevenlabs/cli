pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentReasoningResponsePartClientEventData {
    #[serde(default)]
    pub text: String,
    pub r#type: AgentChatResponsePartType,
    #[serde(default)]
    pub event_id: i64,
}

impl AgentReasoningResponsePartClientEventData {
    pub fn builder() -> AgentReasoningResponsePartClientEventDataBuilder {
        <AgentReasoningResponsePartClientEventDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentReasoningResponsePartClientEventDataBuilder {
    text: Option<String>,
    r#type: Option<AgentChatResponsePartType>,
    event_id: Option<i64>,
}

impl AgentReasoningResponsePartClientEventDataBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: AgentChatResponsePartType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn event_id(mut self, value: i64) -> Self {
        self.event_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentReasoningResponsePartClientEventData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](AgentReasoningResponsePartClientEventDataBuilder::text)
    /// - [`r#type`](AgentReasoningResponsePartClientEventDataBuilder::r#type)
    /// - [`event_id`](AgentReasoningResponsePartClientEventDataBuilder::event_id)
    pub fn build(self) -> Result<AgentReasoningResponsePartClientEventData, BuildError> {
        Ok(AgentReasoningResponsePartClientEventData {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            event_id: self.event_id.ok_or_else(|| BuildError::missing_field("event_id"))?,
        })
    }
}
