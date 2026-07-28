pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Id of the referenced built-in system evaluation.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AttachedSystemEvaluationRefAnalysisItemId {
    SystemEvalCriteriaSentiment,
    SystemEvalCriteriaFrustration,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AttachedSystemEvaluationRefAnalysisItemId {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SystemEvalCriteriaSentiment => serializer.serialize_str("__system_eval_criteria_sentiment"),
            Self::SystemEvalCriteriaFrustration => serializer.serialize_str("__system_eval_criteria_frustration"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AttachedSystemEvaluationRefAnalysisItemId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "__system_eval_criteria_sentiment" => Ok(Self::SystemEvalCriteriaSentiment),
            "__system_eval_criteria_frustration" => Ok(Self::SystemEvalCriteriaFrustration),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AttachedSystemEvaluationRefAnalysisItemId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SystemEvalCriteriaSentiment => write!(f, "__system_eval_criteria_sentiment"),
            Self::SystemEvalCriteriaFrustration => write!(f, "__system_eval_criteria_frustration"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
