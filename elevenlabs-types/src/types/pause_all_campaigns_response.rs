pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PauseAllCampaignsResponse {
    #[serde(default)]
    pub paused_campaign_ids: Vec<String>,
}

impl PauseAllCampaignsResponse {
    pub fn builder() -> PauseAllCampaignsResponseBuilder {
        <PauseAllCampaignsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PauseAllCampaignsResponseBuilder {
    paused_campaign_ids: Option<Vec<String>>,
}

impl PauseAllCampaignsResponseBuilder {
    pub fn paused_campaign_ids(mut self, value: Vec<String>) -> Self {
        self.paused_campaign_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PauseAllCampaignsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`paused_campaign_ids`](PauseAllCampaignsResponseBuilder::paused_campaign_ids)
    pub fn build(self) -> Result<PauseAllCampaignsResponse, BuildError> {
        Ok(PauseAllCampaignsResponse {
            paused_campaign_ids: self.paused_campaign_ids.ok_or_else(|| BuildError::missing_field("paused_campaign_ids"))?,
        })
    }
}
