pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CampaignCallsResponse {
    #[serde(default)]
    pub calls: Vec<CallView>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub has_more: bool,
}

impl CampaignCallsResponse {
    pub fn builder() -> CampaignCallsResponseBuilder {
        <CampaignCallsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignCallsResponseBuilder {
    calls: Option<Vec<CallView>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl CampaignCallsResponseBuilder {
    pub fn calls(mut self, value: Vec<CallView>) -> Self {
        self.calls = Some(value);
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

    /// Consumes the builder and constructs a [`CampaignCallsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`calls`](CampaignCallsResponseBuilder::calls)
    /// - [`has_more`](CampaignCallsResponseBuilder::has_more)
    pub fn build(self) -> Result<CampaignCallsResponse, BuildError> {
        Ok(CampaignCallsResponse {
            calls: self.calls.ok_or_else(|| BuildError::missing_field("calls"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
        })
    }
}
