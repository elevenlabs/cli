pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebhooksListQueryRequest {
    /// Whether to include active usages of the webhook, only usable by admins
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_usages: Option<bool>,
}

impl WebhooksListQueryRequest {
    pub fn builder() -> WebhooksListQueryRequestBuilder {
        <WebhooksListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhooksListQueryRequestBuilder {
    include_usages: Option<bool>,
}

impl WebhooksListQueryRequestBuilder {
    pub fn include_usages(mut self, value: bool) -> Self {
        self.include_usages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebhooksListQueryRequest`].
    pub fn build(self) -> Result<WebhooksListQueryRequest, BuildError> {
        Ok(WebhooksListQueryRequest {
            include_usages: self.include_usages,
        })
    }
}

