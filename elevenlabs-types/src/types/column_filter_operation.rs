pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ColumnFilterOperation {
    In,
    NotIn,
    Le,
    Ge,
    Lt,
    Gt,
    Eq,
    Neq,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ColumnFilterOperation {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::In => serializer.serialize_str("in"),
            Self::NotIn => serializer.serialize_str("not_in"),
            Self::Le => serializer.serialize_str("le"),
            Self::Ge => serializer.serialize_str("ge"),
            Self::Lt => serializer.serialize_str("lt"),
            Self::Gt => serializer.serialize_str("gt"),
            Self::Eq => serializer.serialize_str("eq"),
            Self::Neq => serializer.serialize_str("neq"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ColumnFilterOperation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "in" => Ok(Self::In),
            "not_in" => Ok(Self::NotIn),
            "le" => Ok(Self::Le),
            "ge" => Ok(Self::Ge),
            "lt" => Ok(Self::Lt),
            "gt" => Ok(Self::Gt),
            "eq" => Ok(Self::Eq),
            "neq" => Ok(Self::Neq),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ColumnFilterOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::In => write!(f, "in"),
            Self::NotIn => write!(f, "not_in"),
            Self::Le => write!(f, "le"),
            Self::Ge => write!(f, "ge"),
            Self::Lt => write!(f, "lt"),
            Self::Gt => write!(f, "gt"),
            Self::Eq => write!(f, "eq"),
            Self::Neq => write!(f, "neq"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
