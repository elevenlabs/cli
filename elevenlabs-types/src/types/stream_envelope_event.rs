pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum StreamEnvelopeEvent {
        #[serde(rename = "agent_chat_response_part")]
        #[non_exhaustive]
        AgentChatResponsePart {
            text_response_part: AgentChatResponsePartClientEventData,
        },

        #[serde(rename = "agent_reasoning_response_part")]
        #[non_exhaustive]
        AgentReasoningResponsePart {
            reasoning_response_part: AgentReasoningResponsePartClientEventData,
        },

        #[serde(rename = "agent_response")]
        #[non_exhaustive]
        AgentResponse {
            #[serde(default)]
            agent_response_event: AgentResponseClientEventData,
        },

        #[serde(rename = "agent_tool_request")]
        #[non_exhaustive]
        AgentToolRequest {
            #[serde(default)]
            agent_tool_request: AgentToolRequestClientEventData,
        },

        #[serde(rename = "agent_tool_response")]
        #[non_exhaustive]
        AgentToolResponse {
            #[serde(default)]
            agent_tool_response: AgentToolResponseClientEventData,
        },

        #[serde(rename = "client_tool_call")]
        #[non_exhaustive]
        ClientToolCall {
            #[serde(default)]
            client_tool_call: ClientToolCallClientEventData,
        },

        #[serde(rename = "stream_ended")]
        #[non_exhaustive]
        StreamEnded {
            stream_ended_event: StreamEndedData,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl StreamEnvelopeEvent {
    pub fn agent_chat_response_part(text_response_part: AgentChatResponsePartClientEventData) -> Self {
        Self::AgentChatResponsePart { text_response_part }
    }

    pub fn agent_reasoning_response_part(reasoning_response_part: AgentReasoningResponsePartClientEventData) -> Self {
        Self::AgentReasoningResponsePart { reasoning_response_part }
    }

    pub fn agent_response(agent_response_event: AgentResponseClientEventData) -> Self {
        Self::AgentResponse { agent_response_event }
    }

    pub fn agent_tool_request(agent_tool_request: AgentToolRequestClientEventData) -> Self {
        Self::AgentToolRequest { agent_tool_request }
    }

    pub fn agent_tool_response(agent_tool_response: AgentToolResponseClientEventData) -> Self {
        Self::AgentToolResponse { agent_tool_response }
    }

    pub fn client_tool_call(client_tool_call: ClientToolCallClientEventData) -> Self {
        Self::ClientToolCall { client_tool_call }
    }

    pub fn stream_ended(stream_ended_event: StreamEndedData) -> Self {
        Self::StreamEnded { stream_ended_event }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
