pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One charge to the reseller's card, paying for one workspace's invoice.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ResellerInvoiceSummary {
    #[serde(default)]
    pub invoice_id: String,
    /// The workspace this charge paid for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[serde(default)]
    pub workspace_name: String,
    pub status: ResellerInvoiceStatus,
    /// Invoice total, in the currency's minor unit.
    #[serde(default)]
    pub amount_cents: i64,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub created_unix: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_unix: Option<i64>,
    /// Stripe's hosted payment page, only while open.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdf_url: Option<String>,
}

impl ResellerInvoiceSummary {
    pub fn builder() -> ResellerInvoiceSummaryBuilder {
        <ResellerInvoiceSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResellerInvoiceSummaryBuilder {
    invoice_id: Option<String>,
    workspace_id: Option<String>,
    workspace_name: Option<String>,
    status: Option<ResellerInvoiceStatus>,
    amount_cents: Option<i64>,
    currency: Option<String>,
    created_unix: Option<i64>,
    paid_unix: Option<i64>,
    pay_url: Option<String>,
    pdf_url: Option<String>,
}

impl ResellerInvoiceSummaryBuilder {
    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    pub fn workspace_name(mut self, value: impl Into<String>) -> Self {
        self.workspace_name = Some(value.into());
        self
    }

    pub fn status(mut self, value: ResellerInvoiceStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn amount_cents(mut self, value: i64) -> Self {
        self.amount_cents = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn created_unix(mut self, value: i64) -> Self {
        self.created_unix = Some(value);
        self
    }

    pub fn paid_unix(mut self, value: i64) -> Self {
        self.paid_unix = Some(value);
        self
    }

    pub fn pay_url(mut self, value: impl Into<String>) -> Self {
        self.pay_url = Some(value.into());
        self
    }

    pub fn pdf_url(mut self, value: impl Into<String>) -> Self {
        self.pdf_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResellerInvoiceSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoice_id`](ResellerInvoiceSummaryBuilder::invoice_id)
    /// - [`workspace_name`](ResellerInvoiceSummaryBuilder::workspace_name)
    /// - [`status`](ResellerInvoiceSummaryBuilder::status)
    /// - [`amount_cents`](ResellerInvoiceSummaryBuilder::amount_cents)
    /// - [`currency`](ResellerInvoiceSummaryBuilder::currency)
    /// - [`created_unix`](ResellerInvoiceSummaryBuilder::created_unix)
    pub fn build(self) -> Result<ResellerInvoiceSummary, BuildError> {
        Ok(ResellerInvoiceSummary {
            invoice_id: self.invoice_id.ok_or_else(|| BuildError::missing_field("invoice_id"))?,
            workspace_id: self.workspace_id,
            workspace_name: self.workspace_name.ok_or_else(|| BuildError::missing_field("workspace_name"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            amount_cents: self.amount_cents.ok_or_else(|| BuildError::missing_field("amount_cents"))?,
            currency: self.currency.ok_or_else(|| BuildError::missing_field("currency"))?,
            created_unix: self.created_unix.ok_or_else(|| BuildError::missing_field("created_unix"))?,
            paid_unix: self.paid_unix,
            pay_url: self.pay_url,
            pdf_url: self.pdf_url,
        })
    }
}
