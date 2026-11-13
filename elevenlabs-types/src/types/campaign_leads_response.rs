pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CampaignLeadsResponse {
    #[serde(default)]
    pub leads: Vec<LeadView>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub has_more: bool,
}

impl CampaignLeadsResponse {
    pub fn builder() -> CampaignLeadsResponseBuilder {
        <CampaignLeadsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignLeadsResponseBuilder {
    leads: Option<Vec<LeadView>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl CampaignLeadsResponseBuilder {
    pub fn leads(mut self, value: Vec<LeadView>) -> Self {
        self.leads = Some(value);
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

    /// Consumes the builder and constructs a [`CampaignLeadsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`leads`](CampaignLeadsResponseBuilder::leads)
    /// - [`has_more`](CampaignLeadsResponseBuilder::has_more)
    pub fn build(self) -> Result<CampaignLeadsResponse, BuildError> {
        Ok(CampaignLeadsResponse {
            leads: self.leads.ok_or_else(|| BuildError::missing_field("leads"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
        })
    }
}
