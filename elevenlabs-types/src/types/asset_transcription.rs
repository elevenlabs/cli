pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AssetTranscription {
    pub status: AssetTranscriptionStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<AssetTranscriptionData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_ms: Option<i64>,
}

impl AssetTranscription {
    pub fn builder() -> AssetTranscriptionBuilder {
        <AssetTranscriptionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetTranscriptionBuilder {
    status: Option<AssetTranscriptionStatus>,
    data: Option<AssetTranscriptionData>,
    updated_at_ms: Option<i64>,
}

impl AssetTranscriptionBuilder {
    pub fn status(mut self, value: AssetTranscriptionStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn data(mut self, value: AssetTranscriptionData) -> Self {
        self.data = Some(value);
        self
    }

    pub fn updated_at_ms(mut self, value: i64) -> Self {
        self.updated_at_ms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AssetTranscription`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](AssetTranscriptionBuilder::status)
    pub fn build(self) -> Result<AssetTranscription, BuildError> {
        Ok(AssetTranscription {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            data: self.data,
            updated_at_ms: self.updated_at_ms,
        })
    }
}
