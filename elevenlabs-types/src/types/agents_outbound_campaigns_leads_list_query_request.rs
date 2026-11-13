pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsOutboundCampaignsLeadsListQueryRequest {
    /// How many items to return at most.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Only leads whose ID, user ID or phone number contains this text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// `in_progress` for leads still being called, `finished` for leads that won't be called again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<LeadsListRequestStatus>,
    /// Only leads that finished in one of these ways.
    #[serde(default)]
    pub disposition: Vec<Option<TerminalDisposition>>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl AgentsOutboundCampaignsLeadsListQueryRequest {
    pub fn builder() -> AgentsOutboundCampaignsLeadsListQueryRequestBuilder {
        <AgentsOutboundCampaignsLeadsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsOutboundCampaignsLeadsListQueryRequestBuilder {
    page_size: Option<i64>,
    search: Option<String>,
    status: Option<LeadsListRequestStatus>,
    disposition: Option<Vec<Option<TerminalDisposition>>>,
    cursor: Option<String>,
}

impl AgentsOutboundCampaignsLeadsListQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn status(mut self, value: LeadsListRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn disposition(mut self, value: Vec<Option<TerminalDisposition>>) -> Self {
        self.disposition = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentsOutboundCampaignsLeadsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`disposition`](AgentsOutboundCampaignsLeadsListQueryRequestBuilder::disposition)
    pub fn build(self) -> Result<AgentsOutboundCampaignsLeadsListQueryRequest, BuildError> {
        Ok(AgentsOutboundCampaignsLeadsListQueryRequest {
            page_size: self.page_size,
            search: self.search,
            status: self.status,
            disposition: self.disposition.ok_or_else(|| BuildError::missing_field("disposition"))?,
            cursor: self.cursor,
        })
    }
}

