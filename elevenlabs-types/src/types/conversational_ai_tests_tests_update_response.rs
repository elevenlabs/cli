pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum TestsUpdateResponse {
        #[serde(rename = "llm")]
        #[non_exhaustive]
        Llm {
            #[serde(flatten)]
            data: GetResponseUnitTestResponseModel,
        },

        #[serde(rename = "tool")]
        #[non_exhaustive]
        Tool {
            #[serde(flatten)]
            data: GetToolCallUnitTestResponseModel,
        },

        #[serde(rename = "simulation")]
        #[non_exhaustive]
        Simulation {
            #[serde(flatten)]
            data: GetSimulationTestResponseModel,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl TestsUpdateResponse {
    pub fn llm(data: GetResponseUnitTestResponseModel) -> Self {
        Self::Llm { data }
    }

    pub fn tool(data: GetToolCallUnitTestResponseModel) -> Self {
        Self::Tool { data }
    }

    pub fn simulation(data: GetSimulationTestResponseModel) -> Self {
        Self::Simulation { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
