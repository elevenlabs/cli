pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CampaignSummaryCounts {
    #[serde(default)]
    pub leads: i64,
    /// Leads that won't be called again.
    #[serde(default)]
    pub terminal_leads: i64,
    /// Attention items not yet resolved.
    #[serde(default)]
    pub open_attention: i64,
    /// How many messages reached each stage; set when getting one message campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<RecipientDeliveryCounts>,
}

impl CampaignSummaryCounts {
    pub fn builder() -> CampaignSummaryCountsBuilder {
        <CampaignSummaryCountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignSummaryCountsBuilder {
    leads: Option<i64>,
    terminal_leads: Option<i64>,
    open_attention: Option<i64>,
    delivery: Option<RecipientDeliveryCounts>,
}

impl CampaignSummaryCountsBuilder {
    pub fn leads(mut self, value: i64) -> Self {
        self.leads = Some(value);
        self
    }

    pub fn terminal_leads(mut self, value: i64) -> Self {
        self.terminal_leads = Some(value);
        self
    }

    pub fn open_attention(mut self, value: i64) -> Self {
        self.open_attention = Some(value);
        self
    }

    pub fn delivery(mut self, value: RecipientDeliveryCounts) -> Self {
        self.delivery = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignSummaryCounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`leads`](CampaignSummaryCountsBuilder::leads)
    /// - [`terminal_leads`](CampaignSummaryCountsBuilder::terminal_leads)
    /// - [`open_attention`](CampaignSummaryCountsBuilder::open_attention)
    pub fn build(self) -> Result<CampaignSummaryCounts, BuildError> {
        Ok(CampaignSummaryCounts {
            leads: self.leads.ok_or_else(|| BuildError::missing_field("leads"))?,
            terminal_leads: self.terminal_leads.ok_or_else(|| BuildError::missing_field("terminal_leads"))?,
            open_attention: self.open_attention.ok_or_else(|| BuildError::missing_field("open_attention"))?,
            delivery: self.delivery,
        })
    }
}
