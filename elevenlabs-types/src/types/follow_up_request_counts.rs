pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FollowUpRequestCounts {
    /// How many times contacts asked.
    #[serde(default)]
    pub requested: i64,
    /// Requests by what the campaign did about them; adds up to `requested`. Missing keys mean none.
    #[serde(default)]
    pub requested_by_disposition: HashMap<String, i64>,
    /// Leads that asked at least once.
    #[serde(default)]
    pub distinct_leads: i64,
}

impl FollowUpRequestCounts {
    pub fn builder() -> FollowUpRequestCountsBuilder {
        <FollowUpRequestCountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FollowUpRequestCountsBuilder {
    requested: Option<i64>,
    requested_by_disposition: Option<HashMap<String, i64>>,
    distinct_leads: Option<i64>,
}

impl FollowUpRequestCountsBuilder {
    pub fn requested(mut self, value: i64) -> Self {
        self.requested = Some(value);
        self
    }

    pub fn requested_by_disposition(mut self, value: HashMap<String, i64>) -> Self {
        self.requested_by_disposition = Some(value);
        self
    }

    pub fn distinct_leads(mut self, value: i64) -> Self {
        self.distinct_leads = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FollowUpRequestCounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`requested`](FollowUpRequestCountsBuilder::requested)
    /// - [`requested_by_disposition`](FollowUpRequestCountsBuilder::requested_by_disposition)
    /// - [`distinct_leads`](FollowUpRequestCountsBuilder::distinct_leads)
    pub fn build(self) -> Result<FollowUpRequestCounts, BuildError> {
        Ok(FollowUpRequestCounts {
            requested: self.requested.ok_or_else(|| BuildError::missing_field("requested"))?,
            requested_by_disposition: self.requested_by_disposition.ok_or_else(|| BuildError::missing_field("requested_by_disposition"))?,
            distinct_leads: self.distinct_leads.ok_or_else(|| BuildError::missing_field("distinct_leads"))?,
        })
    }
}
