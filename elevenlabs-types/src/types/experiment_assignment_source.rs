pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExperimentAssignmentSource {
    ServerBranch,
    ClientDeclared,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ExperimentAssignmentSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ServerBranch => serializer.serialize_str("server_branch"),
            Self::ClientDeclared => serializer.serialize_str("client_declared"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ExperimentAssignmentSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "server_branch" => Ok(Self::ServerBranch),
            "client_declared" => Ok(Self::ClientDeclared),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ExperimentAssignmentSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ServerBranch => write!(f, "server_branch"),
            Self::ClientDeclared => write!(f, "client_declared"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
