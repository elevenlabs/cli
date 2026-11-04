pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
#[non_exhaustive]
pub enum AnalysisItemsCreateRequestBody {
        #[serde(rename = "evaluation")]
        #[non_exhaustive]
        Evaluation {
            #[serde(flatten)]
            data: UserEvaluationAnalysisItem,
        },

        #[serde(rename = "data_collection")]
        #[non_exhaustive]
        DataCollection {
            #[serde(flatten)]
            data: UserDataCollectionAnalysisItemInput,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl AnalysisItemsCreateRequestBody {
    pub fn evaluation(data: UserEvaluationAnalysisItem) -> Self {
        Self::Evaluation { data }
    }

    pub fn data_collection(data: UserDataCollectionAnalysisItemInput) -> Self {
        Self::DataCollection { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
