pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for export
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsOutboundCampaignsCallsExportQueryRequest {
    /// Only export the first N calls; used to preview the columns. Omit to export every call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl AgentsOutboundCampaignsCallsExportQueryRequest {
    pub fn builder() -> AgentsOutboundCampaignsCallsExportQueryRequestBuilder {
        <AgentsOutboundCampaignsCallsExportQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsOutboundCampaignsCallsExportQueryRequestBuilder {
    limit: Option<i64>,
}

impl AgentsOutboundCampaignsCallsExportQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentsOutboundCampaignsCallsExportQueryRequest`].
    pub fn build(self) -> Result<AgentsOutboundCampaignsCallsExportQueryRequest, BuildError> {
        Ok(AgentsOutboundCampaignsCallsExportQueryRequest {
            limit: self.limit,
        })
    }
}

