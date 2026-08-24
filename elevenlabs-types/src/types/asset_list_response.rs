pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One page of the workspace's assets, most recently created first.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AssetListResponse {
    /// List of Asset objects.
    #[serde(default)]
    pub assets: Vec<AssetResponse>,
    /// Pass as `cursor` to fetch the next page. `null` if there are no more results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Whether there are more results to fetch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl AssetListResponse {
    pub fn builder() -> AssetListResponseBuilder {
        <AssetListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetListResponseBuilder {
    assets: Option<Vec<AssetResponse>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl AssetListResponseBuilder {
    pub fn assets(mut self, value: Vec<AssetResponse>) -> Self {
        self.assets = Some(value);
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

    /// Consumes the builder and constructs a [`AssetListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`assets`](AssetListResponseBuilder::assets)
    pub fn build(self) -> Result<AssetListResponse, BuildError> {
        Ok(AssetListResponse {
            assets: self.assets.ok_or_else(|| BuildError::missing_field("assets"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more,
        })
    }
}
