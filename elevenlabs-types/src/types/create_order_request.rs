pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateOrderRequest {
    /// When true, creates a sandbox order that auto-progresses without producer intervention.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox: Option<bool>,
}

impl CreateOrderRequest {
    pub fn builder() -> CreateOrderRequestBuilder {
        <CreateOrderRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderRequestBuilder {
    sandbox: Option<bool>,
}

impl CreateOrderRequestBuilder {
    pub fn sandbox(mut self, value: bool) -> Self {
        self.sandbox = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateOrderRequest`].
    pub fn build(self) -> Result<CreateOrderRequest, BuildError> {
        Ok(CreateOrderRequest {
            sandbox: self.sandbox,
        })
    }
}
