pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListOrdersResponse {
    /// The list of orders matching the query.
    #[serde(default)]
    pub orders: Vec<OrderSummary>,
}

impl ListOrdersResponse {
    pub fn builder() -> ListOrdersResponseBuilder {
        <ListOrdersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOrdersResponseBuilder {
    orders: Option<Vec<OrderSummary>>,
}

impl ListOrdersResponseBuilder {
    pub fn orders(mut self, value: Vec<OrderSummary>) -> Self {
        self.orders = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListOrdersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`orders`](ListOrdersResponseBuilder::orders)
    pub fn build(self) -> Result<ListOrdersResponse, BuildError> {
        Ok(ListOrdersResponse {
            orders: self.orders.ok_or_else(|| BuildError::missing_field("orders"))?,
        })
    }
}
