pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum WorkflowToolResponseModelInputStepsItem {
        #[serde(rename = "edge")]
        #[non_exhaustive]
        Edge {
            #[serde(flatten)]
            data: WorkflowToolEdgeStepModel,
        },

        #[serde(rename = "max_iterations_exceeded")]
        #[non_exhaustive]
        MaxIterationsExceeded {
            #[serde(flatten)]
            data: WorkflowToolMaxIterationsExceededStepModel,
        },

        #[serde(rename = "nested_tools")]
        #[non_exhaustive]
        NestedTools {
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers")]
            step_latency_secs: f64,
            #[serde(default)]
            node_id: String,
            #[serde(default)]
            requests: Vec<ConversationHistoryTranscriptToolCallCommonModelInput>,
            #[serde(default)]
            results: Vec<Box<WorkflowToolNestedToolsStepModelInputResultsItem>>,
            #[serde(default)]
            is_successful: bool,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl WorkflowToolResponseModelInputStepsItem {
    pub fn edge(data: WorkflowToolEdgeStepModel) -> Self {
        Self::Edge { data }
    }

    pub fn max_iterations_exceeded(data: WorkflowToolMaxIterationsExceededStepModel) -> Self {
        Self::MaxIterationsExceeded { data }
    }

    pub fn nested_tools(step_latency_secs: f64, node_id: String, requests: Vec<ConversationHistoryTranscriptToolCallCommonModelInput>, results: Vec<Box<WorkflowToolNestedToolsStepModelInputResultsItem>>, is_successful: bool) -> Self {
        Self::NestedTools { step_latency_secs, node_id, requests, results, is_successful }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
