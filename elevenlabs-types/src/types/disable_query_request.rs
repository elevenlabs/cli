pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for disable
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisableQueryRequest {
    /// Must be set to `self` to disable the API key used to authenticate this request. Required as an explicit confirmation to avoid accidentally disabling the wrong key.
    #[serde(default)]
    pub api_key_name: String,
}

impl DisableQueryRequest {
    pub fn builder() -> DisableQueryRequestBuilder {
        <DisableQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisableQueryRequestBuilder {
    api_key_name: Option<String>,
}

impl DisableQueryRequestBuilder {
    pub fn api_key_name(mut self, value: impl Into<String>) -> Self {
        self.api_key_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DisableQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key_name`](DisableQueryRequestBuilder::api_key_name)
    pub fn build(self) -> Result<DisableQueryRequest, BuildError> {
        Ok(DisableQueryRequest {
            api_key_name: self.api_key_name.ok_or_else(|| BuildError::missing_field("api_key_name"))?,
        })
    }
}

