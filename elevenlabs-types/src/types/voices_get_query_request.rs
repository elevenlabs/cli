pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VoicesGetQueryRequest {
    /// This parameter is now deprecated. It is ignored and will be removed in a future version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_settings: Option<bool>,
}

impl VoicesGetQueryRequest {
    pub fn builder() -> VoicesGetQueryRequestBuilder {
        <VoicesGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoicesGetQueryRequestBuilder {
    with_settings: Option<bool>,
}

impl VoicesGetQueryRequestBuilder {
    pub fn with_settings(mut self, value: bool) -> Self {
        self.with_settings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VoicesGetQueryRequest`].
    pub fn build(self) -> Result<VoicesGetQueryRequest, BuildError> {
        Ok(VoicesGetQueryRequest {
            with_settings: self.with_settings,
        })
    }
}

