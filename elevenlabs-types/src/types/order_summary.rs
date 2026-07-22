pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrderSummary {
    /// The ID of the order.
    #[serde(default)]
    pub order_id: OrderId,
    /// The display name of the order.
    #[serde(default)]
    pub name: String,
    /// The current state of the order.
    pub state: OrderState,
    /// The total price for all items in USD. Excluded from response until quotes are available.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total_amount_usd: Option<f64>,
    /// Whether this is a sandbox order that auto-progresses without producer intervention.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox: Option<bool>,
    /// The timestamp when the order was submitted, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub submitted_at: Option<DateTime<FixedOffset>>,
    /// The timestamp when the order was last modified, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl OrderSummary {
    pub fn builder() -> OrderSummaryBuilder {
        <OrderSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrderSummaryBuilder {
    order_id: Option<OrderId>,
    name: Option<String>,
    state: Option<OrderState>,
    total_amount_usd: Option<f64>,
    sandbox: Option<bool>,
    submitted_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl OrderSummaryBuilder {
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

    pub fn total_amount_usd(mut self, value: f64) -> Self {
        self.total_amount_usd = Some(value);
        self
    }

    pub fn sandbox(mut self, value: bool) -> Self {
        self.sandbox = Some(value);
        self
    }

    pub fn submitted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.submitted_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrderSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`order_id`](OrderSummaryBuilder::order_id)
    /// - [`name`](OrderSummaryBuilder::name)
    /// - [`state`](OrderSummaryBuilder::state)
    pub fn build(self) -> Result<OrderSummary, BuildError> {
        Ok(OrderSummary {
            order_id: self.order_id.ok_or_else(|| BuildError::missing_field("order_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
            total_amount_usd: self.total_amount_usd,
            sandbox: self.sandbox,
            submitted_at: self.submitted_at,
            updated_at: self.updated_at,
        })
    }
}
