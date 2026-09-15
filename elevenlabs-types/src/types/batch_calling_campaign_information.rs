pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BatchCallingCampaignInformation {
    #[serde(default)]
    pub campaign_id: String,
    #[serde(default)]
    pub campaign_lead_id: String,
}

impl BatchCallingCampaignInformation {
    pub fn builder() -> BatchCallingCampaignInformationBuilder {
        <BatchCallingCampaignInformationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchCallingCampaignInformationBuilder {
    campaign_id: Option<String>,
    campaign_lead_id: Option<String>,
}

impl BatchCallingCampaignInformationBuilder {
    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    pub fn campaign_lead_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_lead_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BatchCallingCampaignInformation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`campaign_id`](BatchCallingCampaignInformationBuilder::campaign_id)
    /// - [`campaign_lead_id`](BatchCallingCampaignInformationBuilder::campaign_lead_id)
    pub fn build(self) -> Result<BatchCallingCampaignInformation, BuildError> {
        Ok(BatchCallingCampaignInformation {
            campaign_id: self.campaign_id.ok_or_else(|| BuildError::missing_field("campaign_id"))?,
            campaign_lead_id: self.campaign_lead_id.ok_or_else(|| BuildError::missing_field("campaign_lead_id"))?,
        })
    }
}
