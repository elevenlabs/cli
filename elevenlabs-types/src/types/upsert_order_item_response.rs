pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpsertOrderItemResponse {
    /// The ID of the upserted order item.
    #[serde(default)]
    pub item_id: ItemId,
    /// The quoted price for this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<QuoteInfo>,
}

impl UpsertOrderItemResponse {
    pub fn builder() -> UpsertOrderItemResponseBuilder {
        <UpsertOrderItemResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpsertOrderItemResponseBuilder {
    item_id: Option<ItemId>,
    quote: Option<QuoteInfo>,
}

impl UpsertOrderItemResponseBuilder {
    pub fn item_id(mut self, value: ItemId) -> Self {
        self.item_id = Some(value);
        self
    }

    pub fn quote(mut self, value: QuoteInfo) -> Self {
        self.quote = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpsertOrderItemResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item_id`](UpsertOrderItemResponseBuilder::item_id)
    pub fn build(self) -> Result<UpsertOrderItemResponse, BuildError> {
        Ok(UpsertOrderItemResponse {
            item_id: self.item_id.ok_or_else(|| BuildError::missing_field("item_id"))?,
            quote: self.quote,
        })
    }
}
