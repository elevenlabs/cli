pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The policy the campaign took when it was created, laid out like a submitted `policy`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CampaignPolicySnapshot {
    #[serde(default)]
    pub timezone: String,
    #[serde(default)]
    pub retry: RetryPolicy,
    #[serde(default)]
    pub calling_periods: Vec<CallingPeriodPolicy>,
    #[serde(default)]
    pub data_collection: DataCollectionPolicy,
    #[serde(default)]
    pub followup_context: FollowupContextPolicy,
}

impl CampaignPolicySnapshot {
    pub fn builder() -> CampaignPolicySnapshotBuilder {
        <CampaignPolicySnapshotBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignPolicySnapshotBuilder {
    timezone: Option<String>,
    retry: Option<RetryPolicy>,
    calling_periods: Option<Vec<CallingPeriodPolicy>>,
    data_collection: Option<DataCollectionPolicy>,
    followup_context: Option<FollowupContextPolicy>,
}

impl CampaignPolicySnapshotBuilder {
    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    pub fn retry(mut self, value: RetryPolicy) -> Self {
        self.retry = Some(value);
        self
    }

    pub fn calling_periods(mut self, value: Vec<CallingPeriodPolicy>) -> Self {
        self.calling_periods = Some(value);
        self
    }

    pub fn data_collection(mut self, value: DataCollectionPolicy) -> Self {
        self.data_collection = Some(value);
        self
    }

    pub fn followup_context(mut self, value: FollowupContextPolicy) -> Self {
        self.followup_context = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignPolicySnapshot`].
    /// This method will fail if any of the following fields are not set:
    /// - [`timezone`](CampaignPolicySnapshotBuilder::timezone)
    /// - [`retry`](CampaignPolicySnapshotBuilder::retry)
    /// - [`calling_periods`](CampaignPolicySnapshotBuilder::calling_periods)
    /// - [`data_collection`](CampaignPolicySnapshotBuilder::data_collection)
    /// - [`followup_context`](CampaignPolicySnapshotBuilder::followup_context)
    pub fn build(self) -> Result<CampaignPolicySnapshot, BuildError> {
        Ok(CampaignPolicySnapshot {
            timezone: self.timezone.ok_or_else(|| BuildError::missing_field("timezone"))?,
            retry: self.retry.ok_or_else(|| BuildError::missing_field("retry"))?,
            calling_periods: self.calling_periods.ok_or_else(|| BuildError::missing_field("calling_periods"))?,
            data_collection: self.data_collection.ok_or_else(|| BuildError::missing_field("data_collection"))?,
            followup_context: self.followup_context.ok_or_else(|| BuildError::missing_field("followup_context"))?,
        })
    }
}
