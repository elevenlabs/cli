pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrderItemInfo {
    /// The ID of the order item.
    #[serde(default)]
    pub item_id: ItemId,
    /// The item configuration details.
    pub item: OrderItemRequestOutput,
    /// The quoted price for this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<QuoteInfo>,
}

impl OrderItemInfo {
    pub fn builder() -> OrderItemInfoBuilder {
        <OrderItemInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrderItemInfoBuilder {
    item_id: Option<ItemId>,
    item: Option<OrderItemRequestOutput>,
    quote: Option<QuoteInfo>,
}

impl OrderItemInfoBuilder {
    pub fn item_id(mut self, value: ItemId) -> Self {
        self.item_id = Some(value);
        self
    }

    pub fn item(mut self, value: OrderItemRequestOutput) -> Self {
        self.item = Some(value);
        self
    }

    pub fn quote(mut self, value: QuoteInfo) -> Self {
        self.quote = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrderItemInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item_id`](OrderItemInfoBuilder::item_id)
    /// - [`item`](OrderItemInfoBuilder::item)
    pub fn build(self) -> Result<OrderItemInfo, BuildError> {
        Ok(OrderItemInfo {
            item_id: self.item_id.ok_or_else(|| BuildError::missing_field("item_id"))?,
            item: self.item.ok_or_else(|| BuildError::missing_field("item"))?,
            quote: self.quote,
        })
    }
}
