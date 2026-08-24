pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RegionConfigRequest {
    /// Region ID
    pub region_id: TwilioRegionId,
    /// Auth Token for this region
    #[serde(default)]
    pub token: String,
    /// Edge location for this region
    pub edge_location: TwilioEdgeLocation,
}

impl RegionConfigRequest {
    pub fn builder() -> RegionConfigRequestBuilder {
        <RegionConfigRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RegionConfigRequestBuilder {
    region_id: Option<TwilioRegionId>,
    token: Option<String>,
    edge_location: Option<TwilioEdgeLocation>,
}

impl RegionConfigRequestBuilder {
    pub fn region_id(mut self, value: TwilioRegionId) -> Self {
        self.region_id = Some(value);
        self
    }

    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    pub fn edge_location(mut self, value: TwilioEdgeLocation) -> Self {
        self.edge_location = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RegionConfigRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`region_id`](RegionConfigRequestBuilder::region_id)
    /// - [`token`](RegionConfigRequestBuilder::token)
    /// - [`edge_location`](RegionConfigRequestBuilder::edge_location)
    pub fn build(self) -> Result<RegionConfigRequest, BuildError> {
        Ok(RegionConfigRequest {
            region_id: self.region_id.ok_or_else(|| BuildError::missing_field("region_id"))?,
            token: self.token.ok_or_else(|| BuildError::missing_field("token"))?,
            edge_location: self.edge_location.ok_or_else(|| BuildError::missing_field("edge_location"))?,
        })
    }
}
