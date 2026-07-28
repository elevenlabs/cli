pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Get a summary of key business analytics for a time period.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAnalyticsSummaryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl GetAnalyticsSummaryParams {
    pub fn builder() -> GetAnalyticsSummaryParamsBuilder {
        <GetAnalyticsSummaryParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAnalyticsSummaryParamsBuilder {
    smb_tool_type: Option<String>,
}

impl GetAnalyticsSummaryParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetAnalyticsSummaryParams`].
    pub fn build(self) -> Result<GetAnalyticsSummaryParams, BuildError> {
        Ok(GetAnalyticsSummaryParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
