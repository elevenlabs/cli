pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum SendMessage {
        InitializeConnection(InitializeConnection),

        SendText(SendText),

        CloseConnection(CloseConnection),
}

impl SendMessage {
    pub fn is_initialize_connection(&self) -> bool {
        matches!(self, Self::InitializeConnection(_))
    }

    pub fn is_send_text(&self) -> bool {
        matches!(self, Self::SendText(_))
    }

    pub fn is_close_connection(&self) -> bool {
        matches!(self, Self::CloseConnection(_))
    }


    pub fn as_initialize_connection(&self) -> Option<&InitializeConnection> {
        match self {
                    Self::InitializeConnection(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_initialize_connection(self) -> Option<InitializeConnection> {
        match self {
                    Self::InitializeConnection(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_send_text(&self) -> Option<&SendText> {
        match self {
                    Self::SendText(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_send_text(self) -> Option<SendText> {
        match self {
                    Self::SendText(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_close_connection(&self) -> Option<&CloseConnection> {
        match self {
                    Self::CloseConnection(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_close_connection(self) -> Option<CloseConnection> {
        match self {
                    Self::CloseConnection(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for SendMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InitializeConnection(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SendText(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::CloseConnection(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
