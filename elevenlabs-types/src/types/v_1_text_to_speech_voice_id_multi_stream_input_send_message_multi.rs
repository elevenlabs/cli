pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum SendMessageMulti {
        InitializeConnectionMulti(InitializeConnectionMulti),

        InitialiseContext(InitialiseContext),

        SendTextMulti(SendTextMulti),

        FlushContext(FlushContext),

        CloseContext(CloseContext),

        CloseSocket(CloseSocket),

        KeepContextAlive(KeepContextAlive),
}

impl SendMessageMulti {
    pub fn is_initialize_connection_multi(&self) -> bool {
        matches!(self, Self::InitializeConnectionMulti(_))
    }

    pub fn is_initialise_context(&self) -> bool {
        matches!(self, Self::InitialiseContext(_))
    }

    pub fn is_send_text_multi(&self) -> bool {
        matches!(self, Self::SendTextMulti(_))
    }

    pub fn is_flush_context(&self) -> bool {
        matches!(self, Self::FlushContext(_))
    }

    pub fn is_close_context(&self) -> bool {
        matches!(self, Self::CloseContext(_))
    }

    pub fn is_close_socket(&self) -> bool {
        matches!(self, Self::CloseSocket(_))
    }

    pub fn is_keep_context_alive(&self) -> bool {
        matches!(self, Self::KeepContextAlive(_))
    }


    pub fn as_initialize_connection_multi(&self) -> Option<&InitializeConnectionMulti> {
        match self {
                    Self::InitializeConnectionMulti(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_initialize_connection_multi(self) -> Option<InitializeConnectionMulti> {
        match self {
                    Self::InitializeConnectionMulti(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_initialise_context(&self) -> Option<&InitialiseContext> {
        match self {
                    Self::InitialiseContext(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_initialise_context(self) -> Option<InitialiseContext> {
        match self {
                    Self::InitialiseContext(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_send_text_multi(&self) -> Option<&SendTextMulti> {
        match self {
                    Self::SendTextMulti(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_send_text_multi(self) -> Option<SendTextMulti> {
        match self {
                    Self::SendTextMulti(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_flush_context(&self) -> Option<&FlushContext> {
        match self {
                    Self::FlushContext(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_flush_context(self) -> Option<FlushContext> {
        match self {
                    Self::FlushContext(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_close_context(&self) -> Option<&CloseContext> {
        match self {
                    Self::CloseContext(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_close_context(self) -> Option<CloseContext> {
        match self {
                    Self::CloseContext(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_close_socket(&self) -> Option<&CloseSocket> {
        match self {
                    Self::CloseSocket(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_close_socket(self) -> Option<CloseSocket> {
        match self {
                    Self::CloseSocket(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_keep_context_alive(&self) -> Option<&KeepContextAlive> {
        match self {
                    Self::KeepContextAlive(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_keep_context_alive(self) -> Option<KeepContextAlive> {
        match self {
                    Self::KeepContextAlive(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for SendMessageMulti {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InitializeConnectionMulti(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::InitialiseContext(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SendTextMulti(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::FlushContext(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::CloseContext(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::CloseSocket(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::KeepContextAlive(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
