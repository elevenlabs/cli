pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A tier's prices and the limits shown on its tier card.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResellerTierEconomics {
    /// What one month is sold for.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub monthly_price_usd: f64,
    /// What a year up front is sold for. Null for the trial.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub annual_price_usd: Option<f64>,
    /// Phone minutes included each month. A web minute draws half a minute, a chat message a tenth.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub included_minutes_per_month: f64,
    /// Charged per minute past the included ones. Null for the trial.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub overage_price_per_minute_usd: Option<f64>,
    /// Phone numbers a workspace on this tier can hold at once.
    #[serde(default)]
    pub max_phone_numbers: i64,
    /// Calls a workspace on this tier can take at the same time.
    #[serde(default)]
    pub concurrent_calls: i64,
    /// Business locations a workspace on this tier can have active.
    #[serde(default)]
    pub max_active_locations: i64,
    /// Receptionists a workspace on this tier can have active.
    #[serde(default)]
    pub max_customer_facing_agents: i64,
    /// Team members a workspace on this tier can have, counting pending invites.
    #[serde(default)]
    pub max_team_members: i64,
    /// Messages the owner can send the Assistant each day.
    #[serde(default)]
    pub assistant_daily_messages: i64,
}

impl ResellerTierEconomics {
    pub fn builder() -> ResellerTierEconomicsBuilder {
        <ResellerTierEconomicsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResellerTierEconomicsBuilder {
    monthly_price_usd: Option<f64>,
    annual_price_usd: Option<f64>,
    included_minutes_per_month: Option<f64>,
    overage_price_per_minute_usd: Option<f64>,
    max_phone_numbers: Option<i64>,
    concurrent_calls: Option<i64>,
    max_active_locations: Option<i64>,
    max_customer_facing_agents: Option<i64>,
    max_team_members: Option<i64>,
    assistant_daily_messages: Option<i64>,
}

impl ResellerTierEconomicsBuilder {
    pub fn monthly_price_usd(mut self, value: f64) -> Self {
        self.monthly_price_usd = Some(value);
        self
    }

    pub fn annual_price_usd(mut self, value: f64) -> Self {
        self.annual_price_usd = Some(value);
        self
    }

    pub fn included_minutes_per_month(mut self, value: f64) -> Self {
        self.included_minutes_per_month = Some(value);
        self
    }

    pub fn overage_price_per_minute_usd(mut self, value: f64) -> Self {
        self.overage_price_per_minute_usd = Some(value);
        self
    }

    pub fn max_phone_numbers(mut self, value: i64) -> Self {
        self.max_phone_numbers = Some(value);
        self
    }

    pub fn concurrent_calls(mut self, value: i64) -> Self {
        self.concurrent_calls = Some(value);
        self
    }

    pub fn max_active_locations(mut self, value: i64) -> Self {
        self.max_active_locations = Some(value);
        self
    }

    pub fn max_customer_facing_agents(mut self, value: i64) -> Self {
        self.max_customer_facing_agents = Some(value);
        self
    }

    pub fn max_team_members(mut self, value: i64) -> Self {
        self.max_team_members = Some(value);
        self
    }

    pub fn assistant_daily_messages(mut self, value: i64) -> Self {
        self.assistant_daily_messages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResellerTierEconomics`].
    /// This method will fail if any of the following fields are not set:
    /// - [`monthly_price_usd`](ResellerTierEconomicsBuilder::monthly_price_usd)
    /// - [`included_minutes_per_month`](ResellerTierEconomicsBuilder::included_minutes_per_month)
    /// - [`max_phone_numbers`](ResellerTierEconomicsBuilder::max_phone_numbers)
    /// - [`concurrent_calls`](ResellerTierEconomicsBuilder::concurrent_calls)
    /// - [`max_active_locations`](ResellerTierEconomicsBuilder::max_active_locations)
    /// - [`max_customer_facing_agents`](ResellerTierEconomicsBuilder::max_customer_facing_agents)
    /// - [`max_team_members`](ResellerTierEconomicsBuilder::max_team_members)
    /// - [`assistant_daily_messages`](ResellerTierEconomicsBuilder::assistant_daily_messages)
    pub fn build(self) -> Result<ResellerTierEconomics, BuildError> {
        Ok(ResellerTierEconomics {
            monthly_price_usd: self.monthly_price_usd.ok_or_else(|| BuildError::missing_field("monthly_price_usd"))?,
            annual_price_usd: self.annual_price_usd,
            included_minutes_per_month: self.included_minutes_per_month.ok_or_else(|| BuildError::missing_field("included_minutes_per_month"))?,
            overage_price_per_minute_usd: self.overage_price_per_minute_usd,
            max_phone_numbers: self.max_phone_numbers.ok_or_else(|| BuildError::missing_field("max_phone_numbers"))?,
            concurrent_calls: self.concurrent_calls.ok_or_else(|| BuildError::missing_field("concurrent_calls"))?,
            max_active_locations: self.max_active_locations.ok_or_else(|| BuildError::missing_field("max_active_locations"))?,
            max_customer_facing_agents: self.max_customer_facing_agents.ok_or_else(|| BuildError::missing_field("max_customer_facing_agents"))?,
            max_team_members: self.max_team_members.ok_or_else(|| BuildError::missing_field("max_team_members"))?,
            assistant_daily_messages: self.assistant_daily_messages.ok_or_else(|| BuildError::missing_field("assistant_daily_messages"))?,
        })
    }
}
