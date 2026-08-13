pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The public Asset object.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AssetResponse {
    /// Unique identifier for the asset.
    #[serde(default)]
    pub asset_id: String,
    /// Display name of the asset.
    #[serde(default)]
    pub name: String,
    /// MIME type of the uploaded file (e.g. `audio/mpeg`).
    #[serde(default)]
    pub mime_type: String,
    /// Unix timestamp (seconds) the asset was created.
    #[serde(default)]
    pub created_at_unix: i64,
    /// Signed URL to fetch the asset's content. May be `null` if the asset has not finished processing. Do not rely on it being valid for more than 1 hour; fetch the asset again for a fresh URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_url: Option<String>,
}

impl AssetResponse {
    pub fn builder() -> AssetResponseBuilder {
        <AssetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetResponseBuilder {
    asset_id: Option<String>,
    name: Option<String>,
    mime_type: Option<String>,
    created_at_unix: Option<i64>,
    content_url: Option<String>,
}

impl AssetResponseBuilder {
    pub fn asset_id(mut self, value: impl Into<String>) -> Self {
        self.asset_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn mime_type(mut self, value: impl Into<String>) -> Self {
        self.mime_type = Some(value.into());
        self
    }

    pub fn created_at_unix(mut self, value: i64) -> Self {
        self.created_at_unix = Some(value);
        self
    }

    pub fn content_url(mut self, value: impl Into<String>) -> Self {
        self.content_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset_id`](AssetResponseBuilder::asset_id)
    /// - [`name`](AssetResponseBuilder::name)
    /// - [`mime_type`](AssetResponseBuilder::mime_type)
    /// - [`created_at_unix`](AssetResponseBuilder::created_at_unix)
    pub fn build(self) -> Result<AssetResponse, BuildError> {
        Ok(AssetResponse {
            asset_id: self.asset_id.ok_or_else(|| BuildError::missing_field("asset_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            mime_type: self.mime_type.ok_or_else(|| BuildError::missing_field("mime_type"))?,
            created_at_unix: self.created_at_unix.ok_or_else(|| BuildError::missing_field("created_at_unix"))?,
            content_url: self.content_url,
        })
    }
}
