pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One line of the workspace's invoice; an `adjustment` line covers any remainder, so lines sum to the charge.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ResellerInvoiceLine {
    pub kind: ResellerInvoiceLineKind,
    #[serde(default)]
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    /// Negative for a proration credit.
    #[serde(default)]
    pub amount_cents: i64,
    #[serde(default)]
    pub period_start_unix: i64,
    #[serde(default)]
    pub period_end_unix: i64,
}

impl ResellerInvoiceLine {
    pub fn builder() -> ResellerInvoiceLineBuilder {
        <ResellerInvoiceLineBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResellerInvoiceLineBuilder {
    kind: Option<ResellerInvoiceLineKind>,
    description: Option<String>,
    quantity: Option<i64>,
    amount_cents: Option<i64>,
    period_start_unix: Option<i64>,
    period_end_unix: Option<i64>,
}

impl ResellerInvoiceLineBuilder {
    pub fn kind(mut self, value: ResellerInvoiceLineKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: i64) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn amount_cents(mut self, value: i64) -> Self {
        self.amount_cents = Some(value);
        self
    }

    pub fn period_start_unix(mut self, value: i64) -> Self {
        self.period_start_unix = Some(value);
        self
    }

    pub fn period_end_unix(mut self, value: i64) -> Self {
        self.period_end_unix = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResellerInvoiceLine`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](ResellerInvoiceLineBuilder::kind)
    /// - [`description`](ResellerInvoiceLineBuilder::description)
    /// - [`amount_cents`](ResellerInvoiceLineBuilder::amount_cents)
    /// - [`period_start_unix`](ResellerInvoiceLineBuilder::period_start_unix)
    /// - [`period_end_unix`](ResellerInvoiceLineBuilder::period_end_unix)
    pub fn build(self) -> Result<ResellerInvoiceLine, BuildError> {
        Ok(ResellerInvoiceLine {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            quantity: self.quantity,
            amount_cents: self.amount_cents.ok_or_else(|| BuildError::missing_field("amount_cents"))?,
            period_start_unix: self.period_start_unix.ok_or_else(|| BuildError::missing_field("period_start_unix"))?,
            period_end_unix: self.period_end_unix.ok_or_else(|| BuildError::missing_field("period_end_unix"))?,
        })
    }
}
