pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BatchCallingCampaignInformation {
    #[serde(default)]
    pub campaign_id: String,
    #[serde(default)]
    pub campaign_lead_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cold_attempt: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_type: Option<CampaignConversationType>,
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
    cold_attempt: Option<i64>,
    conversation_type: Option<CampaignConversationType>,
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

    pub fn cold_attempt(mut self, value: i64) -> Self {
        self.cold_attempt = Some(value);
        self
    }

    pub fn conversation_type(mut self, value: CampaignConversationType) -> Self {
        self.conversation_type = Some(value);
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
            cold_attempt: self.cold_attempt,
            conversation_type: self.conversation_type,
        })
    }
}
