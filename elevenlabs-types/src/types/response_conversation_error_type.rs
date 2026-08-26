pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// User-facing error types exposed on the public API.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResponseConversationErrorType {
    SystemError,
    CallInitializationError,
    LineBusy,
    NoAnswer,
    CallRejected,
    BlockedByUser,
    AgentConfigurationError,
    InvalidClientRequest,
    PermissionError,
    EntitlementExceeded,
    ClientDisconnected,
    LlmError,
    SpeechError,
    ToolError,
    IntegrationError,
    GuardrailTriggered,
    SafetyViolation,
    MaxDurationExceeded,
    PostProcessingError,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ResponseConversationErrorType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SystemError => serializer.serialize_str("system_error"),
            Self::CallInitializationError => serializer.serialize_str("call_initialization_error"),
            Self::LineBusy => serializer.serialize_str("line_busy"),
            Self::NoAnswer => serializer.serialize_str("no_answer"),
            Self::CallRejected => serializer.serialize_str("call_rejected"),
            Self::BlockedByUser => serializer.serialize_str("blocked_by_user"),
            Self::AgentConfigurationError => serializer.serialize_str("agent_configuration_error"),
            Self::InvalidClientRequest => serializer.serialize_str("invalid_client_request"),
            Self::PermissionError => serializer.serialize_str("permission_error"),
            Self::EntitlementExceeded => serializer.serialize_str("entitlement_exceeded"),
            Self::ClientDisconnected => serializer.serialize_str("client_disconnected"),
            Self::LlmError => serializer.serialize_str("llm_error"),
            Self::SpeechError => serializer.serialize_str("speech_error"),
            Self::ToolError => serializer.serialize_str("tool_error"),
            Self::IntegrationError => serializer.serialize_str("integration_error"),
            Self::GuardrailTriggered => serializer.serialize_str("guardrail_triggered"),
            Self::SafetyViolation => serializer.serialize_str("safety_violation"),
            Self::MaxDurationExceeded => serializer.serialize_str("max_duration_exceeded"),
            Self::PostProcessingError => serializer.serialize_str("post_processing_error"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ResponseConversationErrorType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "system_error" => Ok(Self::SystemError),
            "call_initialization_error" => Ok(Self::CallInitializationError),
            "line_busy" => Ok(Self::LineBusy),
            "no_answer" => Ok(Self::NoAnswer),
            "call_rejected" => Ok(Self::CallRejected),
            "blocked_by_user" => Ok(Self::BlockedByUser),
            "agent_configuration_error" => Ok(Self::AgentConfigurationError),
            "invalid_client_request" => Ok(Self::InvalidClientRequest),
            "permission_error" => Ok(Self::PermissionError),
            "entitlement_exceeded" => Ok(Self::EntitlementExceeded),
            "client_disconnected" => Ok(Self::ClientDisconnected),
            "llm_error" => Ok(Self::LlmError),
            "speech_error" => Ok(Self::SpeechError),
            "tool_error" => Ok(Self::ToolError),
            "integration_error" => Ok(Self::IntegrationError),
            "guardrail_triggered" => Ok(Self::GuardrailTriggered),
            "safety_violation" => Ok(Self::SafetyViolation),
            "max_duration_exceeded" => Ok(Self::MaxDurationExceeded),
            "post_processing_error" => Ok(Self::PostProcessingError),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ResponseConversationErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SystemError => write!(f, "system_error"),
            Self::CallInitializationError => write!(f, "call_initialization_error"),
            Self::LineBusy => write!(f, "line_busy"),
            Self::NoAnswer => write!(f, "no_answer"),
            Self::CallRejected => write!(f, "call_rejected"),
            Self::BlockedByUser => write!(f, "blocked_by_user"),
            Self::AgentConfigurationError => write!(f, "agent_configuration_error"),
            Self::InvalidClientRequest => write!(f, "invalid_client_request"),
            Self::PermissionError => write!(f, "permission_error"),
            Self::EntitlementExceeded => write!(f, "entitlement_exceeded"),
            Self::ClientDisconnected => write!(f, "client_disconnected"),
            Self::LlmError => write!(f, "llm_error"),
            Self::SpeechError => write!(f, "speech_error"),
            Self::ToolError => write!(f, "tool_error"),
            Self::IntegrationError => write!(f, "integration_error"),
            Self::GuardrailTriggered => write!(f, "guardrail_triggered"),
            Self::SafetyViolation => write!(f, "safety_violation"),
            Self::MaxDurationExceeded => write!(f, "max_duration_exceeded"),
            Self::PostProcessingError => write!(f, "post_processing_error"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
