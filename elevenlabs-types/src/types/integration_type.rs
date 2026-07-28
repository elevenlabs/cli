pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IntegrationType {
    McpServer,
    McpIntegration,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for IntegrationType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::McpServer => serializer.serialize_str("mcp_server"),
            Self::McpIntegration => serializer.serialize_str("mcp_integration"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for IntegrationType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "mcp_server" => Ok(Self::McpServer),
            "mcp_integration" => Ok(Self::McpIntegration),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for IntegrationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::McpServer => write!(f, "mcp_server"),
            Self::McpIntegration => write!(f, "mcp_integration"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
