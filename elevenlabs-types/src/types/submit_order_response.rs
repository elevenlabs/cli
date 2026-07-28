pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubmitOrderResponse {
    /// The ID of the submitted order.
    #[serde(default)]
    pub order_id: OrderId,
    /// The current state of the order after submission.
    pub state: OrderState,
    /// The timestamp when the order was submitted.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub submitted_at: DateTime<FixedOffset>,
}

impl SubmitOrderResponse {
    pub fn builder() -> SubmitOrderResponseBuilder {
        <SubmitOrderResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitOrderResponseBuilder {
    order_id: Option<OrderId>,
    state: Option<OrderState>,
    submitted_at: Option<DateTime<FixedOffset>>,
}

impl SubmitOrderResponseBuilder {
    pub fn order_id(mut self, value: OrderId) -> Self {
        self.order_id = Some(value);
        self
    }

    pub fn state(mut self, value: OrderState) -> Self {
        self.state = Some(value);
        self
    }

    pub fn submitted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.submitted_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubmitOrderResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`order_id`](SubmitOrderResponseBuilder::order_id)
    /// - [`state`](SubmitOrderResponseBuilder::state)
    /// - [`submitted_at`](SubmitOrderResponseBuilder::submitted_at)
    pub fn build(self) -> Result<SubmitOrderResponse, BuildError> {
        Ok(SubmitOrderResponse {
            order_id: self.order_id.ok_or_else(|| BuildError::missing_field("order_id"))?,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
            submitted_at: self.submitted_at.ok_or_else(|| BuildError::missing_field("submitted_at"))?,
        })
    }
}
