pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BranchProtectionStatus {
    WriterPermsRequired,
    AdminPermsRequired,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BranchProtectionStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::WriterPermsRequired => serializer.serialize_str("writer_perms_required"),
            Self::AdminPermsRequired => serializer.serialize_str("admin_perms_required"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BranchProtectionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "writer_perms_required" => Ok(Self::WriterPermsRequired),
            "admin_perms_required" => Ok(Self::AdminPermsRequired),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BranchProtectionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WriterPermsRequired => write!(f, "writer_perms_required"),
            Self::AdminPermsRequired => write!(f, "admin_perms_required"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
