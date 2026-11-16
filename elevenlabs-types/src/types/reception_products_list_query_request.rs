pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceptionProductsListQueryRequest {
    /// Only products filed under the group with this label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Only active (`true`) or inactive (`false`) products.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

impl ReceptionProductsListQueryRequest {
    pub fn builder() -> ReceptionProductsListQueryRequestBuilder {
        <ReceptionProductsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionProductsListQueryRequestBuilder {
    group: Option<String>,
    is_active: Option<bool>,
}

impl ReceptionProductsListQueryRequestBuilder {
    pub fn group(mut self, value: impl Into<String>) -> Self {
        self.group = Some(value.into());
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceptionProductsListQueryRequest`].
    pub fn build(self) -> Result<ReceptionProductsListQueryRequest, BuildError> {
        Ok(ReceptionProductsListQueryRequest {
            group: self.group,
            is_active: self.is_active,
        })
    }
}

