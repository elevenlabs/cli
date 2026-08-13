pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An asset uploaded via the assets API.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StaticAssetReference {
    /// The ID of an asset uploaded via the assets API (`POST /v1/assets`), as returned in that response's `asset_id`.
    #[serde(default)]
    pub asset_id: String,
}

impl StaticAssetReference {
    pub fn builder() -> StaticAssetReferenceBuilder {
        <StaticAssetReferenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StaticAssetReferenceBuilder {
    asset_id: Option<String>,
}

impl StaticAssetReferenceBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StaticAssetReference`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](StaticAssetReferenceBuilder::asset_id)
    pub fn build(self) -> Result<StaticAssetReference, BuildError> {
        Ok(StaticAssetReference {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
        })
    }
}
