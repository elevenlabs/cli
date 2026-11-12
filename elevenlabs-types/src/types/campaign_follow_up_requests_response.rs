pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CampaignFollowUpRequestsResponse {
    #[serde(default)]
    pub callbacks: FollowUpRequestCounts,
    #[serde(default)]
    pub redirects: FollowUpRequestCounts,
}

impl CampaignFollowUpRequestsResponse {
    pub fn builder() -> CampaignFollowUpRequestsResponseBuilder {
        <CampaignFollowUpRequestsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignFollowUpRequestsResponseBuilder {
    callbacks: Option<FollowUpRequestCounts>,
    redirects: Option<FollowUpRequestCounts>,
}

impl CampaignFollowUpRequestsResponseBuilder {
    pub fn callbacks(mut self, value: FollowUpRequestCounts) -> Self {
        self.callbacks = Some(value);
        self
    }

    pub fn redirects(mut self, value: FollowUpRequestCounts) -> Self {
        self.redirects = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignFollowUpRequestsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`callbacks`](CampaignFollowUpRequestsResponseBuilder::callbacks)
    /// - [`redirects`](CampaignFollowUpRequestsResponseBuilder::redirects)
    pub fn build(self) -> Result<CampaignFollowUpRequestsResponse, BuildError> {
        Ok(CampaignFollowUpRequestsResponse {
            callbacks: self.callbacks.ok_or_else(|| BuildError::missing_field("callbacks"))?,
            redirects: self.redirects.ok_or_else(|| BuildError::missing_field("redirects"))?,
        })
    }
}
