pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
#[non_exhaustive]
pub enum AnalysisItemVersionResponseModelDefinition {
        #[serde(rename = "data_collection")]
        #[non_exhaustive]
        DataCollection {
            #[serde(flatten)]
            data: UserDataCollectionAnalysisItemOutput,
        },

        #[serde(rename = "evaluation")]
        #[non_exhaustive]
        Evaluation {
            #[serde(flatten)]
            data: UserEvaluationAnalysisItem,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl AnalysisItemVersionResponseModelDefinition {
    pub fn data_collection(data: UserDataCollectionAnalysisItemOutput) -> Self {
        Self::DataCollection { data }
    }

    pub fn evaluation(data: UserEvaluationAnalysisItem) -> Self {
        Self::Evaluation { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
