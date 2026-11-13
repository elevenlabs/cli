pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubmitCampaignResponse {
    pub campaign: CampaignResponse,
    /// False when the request repeated an earlier one and returned its campaign.
    #[serde(default)]
    pub created: bool,
    /// Whether calling starts right away rather than at the next calling window.
    #[serde(default)]
    pub within_calling_window: bool,
}

impl SubmitCampaignResponse {
    pub fn builder() -> SubmitCampaignResponseBuilder {
        <SubmitCampaignResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitCampaignResponseBuilder {
    campaign: Option<CampaignResponse>,
    created: Option<bool>,
    within_calling_window: Option<bool>,
}

impl SubmitCampaignResponseBuilder {
    pub fn campaign(mut self, value: CampaignResponse) -> Self {
        self.campaign = Some(value);
        self
    }

    pub fn created(mut self, value: bool) -> Self {
        self.created = Some(value);
        self
    }

    pub fn within_calling_window(mut self, value: bool) -> Self {
        self.within_calling_window = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubmitCampaignResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`campaign`](SubmitCampaignResponseBuilder::campaign)
    /// - [`created`](SubmitCampaignResponseBuilder::created)
    /// - [`within_calling_window`](SubmitCampaignResponseBuilder::within_calling_window)
    pub fn build(self) -> Result<SubmitCampaignResponse, BuildError> {
        Ok(SubmitCampaignResponse {
            campaign: self.campaign.ok_or_else(|| BuildError::missing_field("campaign"))?,
            created: self.created.ok_or_else(|| BuildError::missing_field("created"))?,
            within_calling_window: self.within_calling_window.ok_or_else(|| BuildError::missing_field("within_calling_window"))?,
        })
    }
}
