pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateOrderResponse {
    /// The ID of the newly created order.
    #[serde(default)]
    pub order_id: OrderId,
    /// Whether this is a sandbox order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox: Option<bool>,
}

impl CreateOrderResponse {
    pub fn builder() -> CreateOrderResponseBuilder {
        <CreateOrderResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderResponseBuilder {
    order_id: Option<OrderId>,
    sandbox: Option<bool>,
}

impl CreateOrderResponseBuilder {
    pub fn order_id(mut self, value: OrderId) -> Self {
        self.order_id = Some(value);
        self
    }

    pub fn sandbox(mut self, value: bool) -> Self {
        self.sandbox = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateOrderResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`order_id`](CreateOrderResponseBuilder::order_id)
    pub fn build(self) -> Result<CreateOrderResponse, BuildError> {
        Ok(CreateOrderResponse {
            order_id: self.order_id.ok_or_else(|| BuildError::missing_field("order_id"))?,
            sandbox: self.sandbox,
        })
    }
}
