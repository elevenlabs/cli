pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpsertOrderItemRequest {
    /// The order item to add or update.
    pub item: OrderItemRequestInput,
    /// The ID of an existing item to update. Omit to create a new item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<ItemId>,
}

impl UpsertOrderItemRequest {
    pub fn builder() -> UpsertOrderItemRequestBuilder {
        <UpsertOrderItemRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpsertOrderItemRequestBuilder {
    item: Option<OrderItemRequestInput>,
    item_id: Option<ItemId>,
}

impl UpsertOrderItemRequestBuilder {
    pub fn item(mut self, value: OrderItemRequestInput) -> Self {
        self.item = Some(value);
        self
    }

    pub fn item_id(mut self, value: ItemId) -> Self {
        self.item_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpsertOrderItemRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item`](UpsertOrderItemRequestBuilder::item)
    pub fn build(self) -> Result<UpsertOrderItemRequest, BuildError> {
        Ok(UpsertOrderItemRequest {
            item: self.item.ok_or_else(|| BuildError::missing_field("item"))?,
            item_id: self.item_id,
        })
    }
}
