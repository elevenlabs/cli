pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsOutboundCampaignsCallsListQueryRequest {
    /// How many items to return at most.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Only this lead's calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lead_id: Option<String>,
    /// Only calls in this status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CallStatus>,
    /// Only calls that ended in one of these results.
    #[serde(default)]
    pub call_result: Vec<Option<CallResult>>,
    /// Only messages at this delivery stage. Message campaigns only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<CallsListRequestStage>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl AgentsOutboundCampaignsCallsListQueryRequest {
    pub fn builder() -> AgentsOutboundCampaignsCallsListQueryRequestBuilder {
        <AgentsOutboundCampaignsCallsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsOutboundCampaignsCallsListQueryRequestBuilder {
    page_size: Option<i64>,
    lead_id: Option<String>,
    status: Option<CallStatus>,
    call_result: Option<Vec<Option<CallResult>>>,
    stage: Option<CallsListRequestStage>,
    cursor: Option<String>,
}

impl AgentsOutboundCampaignsCallsListQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn lead_id(mut self, value: impl Into<String>) -> Self {
        self.lead_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: CallStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn call_result(mut self, value: Vec<Option<CallResult>>) -> Self {
        self.call_result = Some(value);
        self
    }

    pub fn stage(mut self, value: CallsListRequestStage) -> Self {
        self.stage = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentsOutboundCampaignsCallsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`call_result`](AgentsOutboundCampaignsCallsListQueryRequestBuilder::call_result)
    pub fn build(self) -> Result<AgentsOutboundCampaignsCallsListQueryRequest, BuildError> {
        Ok(AgentsOutboundCampaignsCallsListQueryRequest {
            page_size: self.page_size,
            lead_id: self.lead_id,
            status: self.status,
            call_result: self.call_result.ok_or_else(|| BuildError::missing_field("call_result"))?,
            stage: self.stage,
            cursor: self.cursor,
        })
    }
}

