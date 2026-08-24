pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VoicesAccentsGetQueryRequest {
    /// If provided, only accents for this language code are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// If provided, returns the accents available for this model. Defaults to the most complete accent list when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
}

impl VoicesAccentsGetQueryRequest {
    pub fn builder() -> VoicesAccentsGetQueryRequestBuilder {
        <VoicesAccentsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoicesAccentsGetQueryRequestBuilder {
    language: Option<String>,
    model_id: Option<String>,
}

impl VoicesAccentsGetQueryRequestBuilder {
    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn model_id(mut self, value: impl Into<String>) -> Self {
        self.model_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VoicesAccentsGetQueryRequest`].
    pub fn build(self) -> Result<VoicesAccentsGetQueryRequest, BuildError> {
        Ok(VoicesAccentsGetQueryRequest {
            language: self.language,
            model_id: self.model_id,
        })
    }
}

