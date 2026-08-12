pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrderResponse {
    /// The ID of the order.
    #[serde(default)]
    pub order_id: OrderId,
    /// The display name of the order.
    #[serde(default)]
    pub name: String,
    /// The current state of the order.
    pub state: OrderState,
    /// The list of items in this order with their quotes.
    #[serde(default)]
    pub items: Vec<OrderItemInfo>,
    /// The total price for all items in USD. Excluded from response until quotes are available.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total_amount_usd: Option<f64>,
    /// Whether this is a sandbox order that auto-progresses without producer intervention.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox: Option<bool>,
    /// The timestamp when the order was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The timestamp when the order was submitted, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub submitted_at: Option<DateTime<FixedOffset>>,
    /// The timestamp when payment was received, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub paid_at: Option<DateTime<FixedOffset>>,
    /// The timestamp when the order was accepted for production, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub accepted_at: Option<DateTime<FixedOffset>>,
    /// The timestamp when the order was completed, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub completed_at: Option<DateTime<FixedOffset>>,
    /// The reason the order was cancelled, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_reason: Option<String>,
}

impl OrderResponse {
    pub fn builder() -> OrderResponseBuilder {
        <OrderResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrderResponseBuilder {
    order_id: Option<OrderId>,
    name: Option<String>,
    state: Option<OrderState>,
    items: Option<Vec<OrderItemInfo>>,
    total_amount_usd: Option<f64>,
    sandbox: Option<bool>,
    created_at: Option<DateTime<FixedOffset>>,
    submitted_at: Option<DateTime<FixedOffset>>,
    paid_at: Option<DateTime<FixedOffset>>,
    accepted_at: Option<DateTime<FixedOffset>>,
    completed_at: Option<DateTime<FixedOffset>>,
    cancel_reason: Option<String>,
}

impl OrderResponseBuilder {
    pub fn order_id(mut self, value: OrderId) -> Self {
        self.order_id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn state(mut self, value: OrderState) -> Self {
        self.state = Some(value);
        self
    }

    pub fn items(mut self, value: Vec<OrderItemInfo>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn total_amount_usd(mut self, value: f64) -> Self {
        self.total_amount_usd = Some(value);
        self
    }

    pub fn sandbox(mut self, value: bool) -> Self {
        self.sandbox = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn submitted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.submitted_at = Some(value);
        self
    }

    pub fn paid_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.paid_at = Some(value);
        self
    }

    pub fn accepted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.accepted_at = Some(value);
        self
    }

    pub fn completed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.completed_at = Some(value);
        self
    }

    pub fn cancel_reason(mut self, value: impl Into<String>) -> Self {
        self.cancel_reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OrderResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`order_id`](OrderResponseBuilder::order_id)
    /// - [`name`](OrderResponseBuilder::name)
    /// - [`state`](OrderResponseBuilder::state)
    /// - [`items`](OrderResponseBuilder::items)
    /// - [`created_at`](OrderResponseBuilder::created_at)
    pub fn build(self) -> Result<OrderResponse, BuildError> {
        Ok(OrderResponse {
            order_id: self.order_id.ok_or_else(|| BuildError::missing_field("order_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
            total_amount_usd: self.total_amount_usd,
            sandbox: self.sandbox,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            submitted_at: self.submitted_at,
            paid_at: self.paid_at,
            accepted_at: self.accepted_at,
            completed_at: self.completed_at,
            cancel_reason: self.cancel_reason,
        })
    }
}
