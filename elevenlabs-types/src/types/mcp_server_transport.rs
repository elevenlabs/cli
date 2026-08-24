pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Supported MCP server transport types.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum McpServerTransport {
    Sse,
    StreamableHttp,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for McpServerTransport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Sse => serializer.serialize_str("SSE"),
            Self::StreamableHttp => serializer.serialize_str("STREAMABLE_HTTP"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for McpServerTransport {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "SSE" => Ok(Self::Sse),
            "STREAMABLE_HTTP" => Ok(Self::StreamableHttp),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for McpServerTransport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sse => write!(f, "SSE"),
            Self::StreamableHttp => write!(f, "STREAMABLE_HTTP"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
