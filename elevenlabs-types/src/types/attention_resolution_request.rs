pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AttentionResolutionRequest {
    /// How the item was handled.
    #[serde(default)]
    pub resolution: String,
}

impl AttentionResolutionRequest {
    pub fn builder() -> AttentionResolutionRequestBuilder {
        <AttentionResolutionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AttentionResolutionRequestBuilder {
    resolution: Option<String>,
}

impl AttentionResolutionRequestBuilder {
    pub fn resolution(mut self, value: impl Into<String>) -> Self {
        self.resolution = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AttentionResolutionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`resolution`](AttentionResolutionRequestBuilder::resolution)
    pub fn build(self) -> Result<AttentionResolutionRequest, BuildError> {
        Ok(AttentionResolutionRequest {
            resolution: self.resolution.ok_or_else(|| BuildError::missing_field("resolution"))?,
        })
    }
}

