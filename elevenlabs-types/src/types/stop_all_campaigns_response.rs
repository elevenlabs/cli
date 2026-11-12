pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StopAllCampaignsResponse {
    #[serde(default)]
    pub stopped_campaign_ids: Vec<String>,
    /// Campaigns whose calls already placed are still finishing.
    #[serde(default)]
    pub stopping_campaign_ids: Vec<String>,
}

impl StopAllCampaignsResponse {
    pub fn builder() -> StopAllCampaignsResponseBuilder {
        <StopAllCampaignsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StopAllCampaignsResponseBuilder {
    stopped_campaign_ids: Option<Vec<String>>,
    stopping_campaign_ids: Option<Vec<String>>,
}

impl StopAllCampaignsResponseBuilder {
    pub fn stopped_campaign_ids(mut self, value: Vec<String>) -> Self {
        self.stopped_campaign_ids = Some(value);
        self
    }

    pub fn stopping_campaign_ids(mut self, value: Vec<String>) -> Self {
        self.stopping_campaign_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StopAllCampaignsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`stopped_campaign_ids`](StopAllCampaignsResponseBuilder::stopped_campaign_ids)
    /// - [`stopping_campaign_ids`](StopAllCampaignsResponseBuilder::stopping_campaign_ids)
    pub fn build(self) -> Result<StopAllCampaignsResponse, BuildError> {
        Ok(StopAllCampaignsResponse {
            stopped_campaign_ids: self.stopped_campaign_ids.ok_or_else(|| BuildError::missing_field("stopped_campaign_ids"))?,
            stopping_campaign_ids: self.stopping_campaign_ids.ok_or_else(|| BuildError::missing_field("stopping_campaign_ids"))?,
        })
    }
}
