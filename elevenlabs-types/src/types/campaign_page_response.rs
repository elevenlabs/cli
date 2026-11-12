pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CampaignPageResponse {
    #[serde(default)]
    pub campaigns: Vec<CampaignResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub has_more: bool,
}

impl CampaignPageResponse {
    pub fn builder() -> CampaignPageResponseBuilder {
        <CampaignPageResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignPageResponseBuilder {
    campaigns: Option<Vec<CampaignResponse>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl CampaignPageResponseBuilder {
    pub fn campaigns(mut self, value: Vec<CampaignResponse>) -> Self {
        self.campaigns = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignPageResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`campaigns`](CampaignPageResponseBuilder::campaigns)
    /// - [`has_more`](CampaignPageResponseBuilder::has_more)
    pub fn build(self) -> Result<CampaignPageResponse, BuildError> {
        Ok(CampaignPageResponse {
            campaigns: self.campaigns.ok_or_else(|| BuildError::missing_field("campaigns"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
        })
    }
}
