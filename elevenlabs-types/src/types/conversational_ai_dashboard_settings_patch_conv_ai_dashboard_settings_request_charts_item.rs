pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum PatchConvAiDashboardSettingsRequestChartsItem {
        #[serde(rename = "call_success")]
        #[non_exhaustive]
        CallSuccess {
            #[serde(flatten)]
            data: DashboardCallSuccessChartModel,
        },

        #[serde(rename = "criteria")]
        #[non_exhaustive]
        Criteria {
            #[serde(flatten)]
            data: DashboardCriteriaChartModel,
        },

        #[serde(rename = "data_collection")]
        #[non_exhaustive]
        DataCollection {
            #[serde(flatten)]
            data: DashboardDataCollectionChartModel,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl PatchConvAiDashboardSettingsRequestChartsItem {
    pub fn call_success(data: DashboardCallSuccessChartModel) -> Self {
        Self::CallSuccess { data }
    }

    pub fn criteria(data: DashboardCriteriaChartModel) -> Self {
        Self::Criteria { data }
    }

    pub fn data_collection(data: DashboardDataCollectionChartModel) -> Self {
        Self::DataCollection { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
