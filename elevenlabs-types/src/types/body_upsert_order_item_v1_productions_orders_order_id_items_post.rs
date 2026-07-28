pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BodyUpsertOrderItemV1ProductionsOrdersOrderIdItemsPost {
    pub request: UpsertOrderItemRequest,
}

impl BodyUpsertOrderItemV1ProductionsOrdersOrderIdItemsPost {
    pub fn builder() -> BodyUpsertOrderItemV1ProductionsOrdersOrderIdItemsPostBuilder {
        <BodyUpsertOrderItemV1ProductionsOrdersOrderIdItemsPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyUpsertOrderItemV1ProductionsOrdersOrderIdItemsPostBuilder {
    request: Option<UpsertOrderItemRequest>,
}

impl BodyUpsertOrderItemV1ProductionsOrdersOrderIdItemsPostBuilder {
    pub fn request(mut self, value: UpsertOrderItemRequest) -> Self {
        self.request = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyUpsertOrderItemV1ProductionsOrdersOrderIdItemsPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`request`](BodyUpsertOrderItemV1ProductionsOrdersOrderIdItemsPostBuilder::request)
    pub fn build(self) -> Result<BodyUpsertOrderItemV1ProductionsOrdersOrderIdItemsPost, BuildError> {
        Ok(BodyUpsertOrderItemV1ProductionsOrdersOrderIdItemsPost {
            request: self.request.ok_or_else(|| BuildError::missing_field("request"))?,
        })
    }
}

