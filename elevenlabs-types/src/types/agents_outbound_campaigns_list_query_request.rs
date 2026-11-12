pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsOutboundCampaignsListQueryRequest {
    /// How many items to return at most.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Only campaigns in one of these statuses.
    #[serde(default)]
    pub status: Vec<Option<CampaignStatus>>,
    /// Only campaigns run by this agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// Only campaigns on this channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<OutboundChannel>,
    /// Only voice or only message campaigns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<OutboundCampaignsListRequestMode>,
    /// Only campaigns whose name contains this text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl AgentsOutboundCampaignsListQueryRequest {
    pub fn builder() -> AgentsOutboundCampaignsListQueryRequestBuilder {
        <AgentsOutboundCampaignsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsOutboundCampaignsListQueryRequestBuilder {
    page_size: Option<i64>,
    status: Option<Vec<Option<CampaignStatus>>>,
    agent_id: Option<String>,
    channel: Option<OutboundChannel>,
    mode: Option<OutboundCampaignsListRequestMode>,
    search: Option<String>,
    cursor: Option<String>,
}

impl AgentsOutboundCampaignsListQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn status(mut self, value: Vec<Option<CampaignStatus>>) -> Self {
        self.status = Some(value);
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn channel(mut self, value: OutboundChannel) -> Self {
        self.channel = Some(value);
        self
    }

    pub fn mode(mut self, value: OutboundCampaignsListRequestMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentsOutboundCampaignsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](AgentsOutboundCampaignsListQueryRequestBuilder::status)
    pub fn build(self) -> Result<AgentsOutboundCampaignsListQueryRequest, BuildError> {
        Ok(AgentsOutboundCampaignsListQueryRequest {
            page_size: self.page_size,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            agent_id: self.agent_id,
            channel: self.channel,
            mode: self.mode,
            search: self.search,
            cursor: self.cursor,
        })
    }
}

