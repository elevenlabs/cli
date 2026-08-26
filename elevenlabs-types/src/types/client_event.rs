pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClientEvent {
    ConversationInitiationMetadata,
    AsrInitiationMetadata,
    Ping,
    Audio,
    Interruption,
    UserTranscript,
    TentativeUserTranscript,
    AgentResponse,
    AgentResponseCorrection,
    ClientToolCall,
    McpToolCall,
    McpConnectionStatus,
    AgentToolRequest,
    AgentToolResponse,
    AgentToolResponseFullPayload,
    AgentResponseMetadata,
    VadScore,
    AgentChatResponsePart,
    ClientError,
    GuardrailTriggered,
    DtmfRequest,
    AgentResponseComplete,
    ContextUsage,
    InternalTurnProbability,
    InternalTentativeAgentResponse,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ClientEvent {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ConversationInitiationMetadata => serializer.serialize_str("conversation_initiation_metadata"),
            Self::AsrInitiationMetadata => serializer.serialize_str("asr_initiation_metadata"),
            Self::Ping => serializer.serialize_str("ping"),
            Self::Audio => serializer.serialize_str("audio"),
            Self::Interruption => serializer.serialize_str("interruption"),
            Self::UserTranscript => serializer.serialize_str("user_transcript"),
            Self::TentativeUserTranscript => serializer.serialize_str("tentative_user_transcript"),
            Self::AgentResponse => serializer.serialize_str("agent_response"),
            Self::AgentResponseCorrection => serializer.serialize_str("agent_response_correction"),
            Self::ClientToolCall => serializer.serialize_str("client_tool_call"),
            Self::McpToolCall => serializer.serialize_str("mcp_tool_call"),
            Self::McpConnectionStatus => serializer.serialize_str("mcp_connection_status"),
            Self::AgentToolRequest => serializer.serialize_str("agent_tool_request"),
            Self::AgentToolResponse => serializer.serialize_str("agent_tool_response"),
            Self::AgentToolResponseFullPayload => serializer.serialize_str("agent_tool_response_full_payload"),
            Self::AgentResponseMetadata => serializer.serialize_str("agent_response_metadata"),
            Self::VadScore => serializer.serialize_str("vad_score"),
            Self::AgentChatResponsePart => serializer.serialize_str("agent_chat_response_part"),
            Self::ClientError => serializer.serialize_str("client_error"),
            Self::GuardrailTriggered => serializer.serialize_str("guardrail_triggered"),
            Self::DtmfRequest => serializer.serialize_str("dtmf_request"),
            Self::AgentResponseComplete => serializer.serialize_str("agent_response_complete"),
            Self::ContextUsage => serializer.serialize_str("context_usage"),
            Self::InternalTurnProbability => serializer.serialize_str("internal_turn_probability"),
            Self::InternalTentativeAgentResponse => serializer.serialize_str("internal_tentative_agent_response"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ClientEvent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "conversation_initiation_metadata" => Ok(Self::ConversationInitiationMetadata),
            "asr_initiation_metadata" => Ok(Self::AsrInitiationMetadata),
            "ping" => Ok(Self::Ping),
            "audio" => Ok(Self::Audio),
            "interruption" => Ok(Self::Interruption),
            "user_transcript" => Ok(Self::UserTranscript),
            "tentative_user_transcript" => Ok(Self::TentativeUserTranscript),
            "agent_response" => Ok(Self::AgentResponse),
            "agent_response_correction" => Ok(Self::AgentResponseCorrection),
            "client_tool_call" => Ok(Self::ClientToolCall),
            "mcp_tool_call" => Ok(Self::McpToolCall),
            "mcp_connection_status" => Ok(Self::McpConnectionStatus),
            "agent_tool_request" => Ok(Self::AgentToolRequest),
            "agent_tool_response" => Ok(Self::AgentToolResponse),
            "agent_tool_response_full_payload" => Ok(Self::AgentToolResponseFullPayload),
            "agent_response_metadata" => Ok(Self::AgentResponseMetadata),
            "vad_score" => Ok(Self::VadScore),
            "agent_chat_response_part" => Ok(Self::AgentChatResponsePart),
            "client_error" => Ok(Self::ClientError),
            "guardrail_triggered" => Ok(Self::GuardrailTriggered),
            "dtmf_request" => Ok(Self::DtmfRequest),
            "agent_response_complete" => Ok(Self::AgentResponseComplete),
            "context_usage" => Ok(Self::ContextUsage),
            "internal_turn_probability" => Ok(Self::InternalTurnProbability),
            "internal_tentative_agent_response" => Ok(Self::InternalTentativeAgentResponse),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ClientEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConversationInitiationMetadata => write!(f, "conversation_initiation_metadata"),
            Self::AsrInitiationMetadata => write!(f, "asr_initiation_metadata"),
            Self::Ping => write!(f, "ping"),
            Self::Audio => write!(f, "audio"),
            Self::Interruption => write!(f, "interruption"),
            Self::UserTranscript => write!(f, "user_transcript"),
            Self::TentativeUserTranscript => write!(f, "tentative_user_transcript"),
            Self::AgentResponse => write!(f, "agent_response"),
            Self::AgentResponseCorrection => write!(f, "agent_response_correction"),
            Self::ClientToolCall => write!(f, "client_tool_call"),
            Self::McpToolCall => write!(f, "mcp_tool_call"),
            Self::McpConnectionStatus => write!(f, "mcp_connection_status"),
            Self::AgentToolRequest => write!(f, "agent_tool_request"),
            Self::AgentToolResponse => write!(f, "agent_tool_response"),
            Self::AgentToolResponseFullPayload => write!(f, "agent_tool_response_full_payload"),
            Self::AgentResponseMetadata => write!(f, "agent_response_metadata"),
            Self::VadScore => write!(f, "vad_score"),
            Self::AgentChatResponsePart => write!(f, "agent_chat_response_part"),
            Self::ClientError => write!(f, "client_error"),
            Self::GuardrailTriggered => write!(f, "guardrail_triggered"),
            Self::DtmfRequest => write!(f, "dtmf_request"),
            Self::AgentResponseComplete => write!(f, "agent_response_complete"),
            Self::ContextUsage => write!(f, "context_usage"),
            Self::InternalTurnProbability => write!(f, "internal_turn_probability"),
            Self::InternalTentativeAgentResponse => write!(f, "internal_tentative_agent_response"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
