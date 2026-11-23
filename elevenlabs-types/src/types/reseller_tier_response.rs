pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A tier you can sell.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResellerTierResponse {
    #[serde(default)]
    pub tier_id: String,
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub tier_economics: ResellerTierEconomics,
}

impl ResellerTierResponse {
    pub fn builder() -> ResellerTierResponseBuilder {
        <ResellerTierResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResellerTierResponseBuilder {
    tier_id: Option<String>,
    display_name: Option<String>,
    tier_economics: Option<ResellerTierEconomics>,
}

impl ResellerTierResponseBuilder {
    pub fn tier_id(mut self, value: impl Into<String>) -> Self {
        self.tier_id = Some(value.into());
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn tier_economics(mut self, value: ResellerTierEconomics) -> Self {
        self.tier_economics = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResellerTierResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tier_id`](ResellerTierResponseBuilder::tier_id)
    /// - [`display_name`](ResellerTierResponseBuilder::display_name)
    /// - [`tier_economics`](ResellerTierResponseBuilder::tier_economics)
    pub fn build(self) -> Result<ResellerTierResponse, BuildError> {
        Ok(ResellerTierResponse {
            tier_id: self.tier_id.ok_or_else(|| BuildError::missing_field("tier_id"))?,
            display_name: self.display_name.ok_or_else(|| BuildError::missing_field("display_name"))?,
            tier_economics: self.tier_economics.ok_or_else(|| BuildError::missing_field("tier_economics"))?,
        })
    }
}
