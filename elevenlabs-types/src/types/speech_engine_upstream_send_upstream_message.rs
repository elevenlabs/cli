pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum SendUpstreamMessage {
        AgentResponsePayload(AgentResponsePayload),

        PongPayload(PongPayload),
}

impl SendUpstreamMessage {
    pub fn is_agent_response_payload(&self) -> bool {
        matches!(self, Self::AgentResponsePayload(_))
    }

    pub fn is_pong_payload(&self) -> bool {
        matches!(self, Self::PongPayload(_))
    }


    pub fn as_agent_response_payload(&self) -> Option<&AgentResponsePayload> {
        match self {
                    Self::AgentResponsePayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_agent_response_payload(self) -> Option<AgentResponsePayload> {
        match self {
                    Self::AgentResponsePayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_pong_payload(&self) -> Option<&PongPayload> {
        match self {
                    Self::PongPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_pong_payload(self) -> Option<PongPayload> {
        match self {
                    Self::PongPayload(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for SendUpstreamMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AgentResponsePayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::PongPayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
