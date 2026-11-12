pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SuppressLeadResponse {
    #[serde(default)]
    pub campaign_id: String,
    #[serde(default)]
    pub lead_id: String,
    /// Whether a call already handed off for dialing may still go through.
    #[serde(default)]
    pub has_calls_in_flight: bool,
}

impl SuppressLeadResponse {
    pub fn builder() -> SuppressLeadResponseBuilder {
        <SuppressLeadResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SuppressLeadResponseBuilder {
    campaign_id: Option<String>,
    lead_id: Option<String>,
    has_calls_in_flight: Option<bool>,
}

impl SuppressLeadResponseBuilder {
    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    pub fn lead_id(mut self, value: impl Into<String>) -> Self {
        self.lead_id = Some(value.into());
        self
    }

    pub fn has_calls_in_flight(mut self, value: bool) -> Self {
        self.has_calls_in_flight = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SuppressLeadResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`campaign_id`](SuppressLeadResponseBuilder::campaign_id)
    /// - [`lead_id`](SuppressLeadResponseBuilder::lead_id)
    /// - [`has_calls_in_flight`](SuppressLeadResponseBuilder::has_calls_in_flight)
    pub fn build(self) -> Result<SuppressLeadResponse, BuildError> {
        Ok(SuppressLeadResponse {
            campaign_id: self.campaign_id.ok_or_else(|| BuildError::missing_field("campaign_id"))?,
            lead_id: self.lead_id.ok_or_else(|| BuildError::missing_field("lead_id"))?,
            has_calls_in_flight: self.has_calls_in_flight.ok_or_else(|| BuildError::missing_field("has_calls_in_flight"))?,
        })
    }
}
