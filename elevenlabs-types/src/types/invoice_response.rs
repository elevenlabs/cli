pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InvoiceResponse {
    /// The amount due in cents.
    #[serde(default)]
    pub amount_due_cents: i64,
    /// The subtotal amount in cents before tax (exclusive of tax and discounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtotal_cents: Option<i64>,
    /// The tax amount in cents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_cents: Option<i64>,
    /// Deprecated. Use [discounts] instead. The discount applied to the invoice. E.g. [20.0f] for 20% off.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub discount_percent_off: Option<f64>,
    /// Deprecated. Use [discounts] instead. The discount applied to the invoice. E.g. [20.0f] for 20 cents off.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub discount_amount_off: Option<f64>,
    /// The discounts applied to the invoice.
    #[serde(default)]
    pub discounts: Vec<DiscountResponseModel>,
    /// The Unix timestamp of the next payment attempt. -1 when there is no next payment attempt.
    #[serde(default)]
    pub next_payment_attempt_unix: i64,
    /// Deprecated. Use [payment_intent_statusses] instead. The status of this invoice's first payment intent. None when there is no payment intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent_status: Option<InvoiceResponsePaymentIntentStatus>,
    /// The statuses of this invoice's payment intents. Empty list when there are no payment intents.
    #[serde(default)]
    pub payment_intent_statusses: Vec<InvoiceResponsePaymentIntentStatussesItem>,
}

impl InvoiceResponse {
    pub fn builder() -> InvoiceResponseBuilder {
        <InvoiceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoiceResponseBuilder {
    amount_due_cents: Option<i64>,
    subtotal_cents: Option<i64>,
    tax_cents: Option<i64>,
    discount_percent_off: Option<f64>,
    discount_amount_off: Option<f64>,
    discounts: Option<Vec<DiscountResponseModel>>,
    next_payment_attempt_unix: Option<i64>,
    payment_intent_status: Option<InvoiceResponsePaymentIntentStatus>,
    payment_intent_statusses: Option<Vec<InvoiceResponsePaymentIntentStatussesItem>>,
}

impl InvoiceResponseBuilder {
    pub fn amount_due_cents(mut self, value: i64) -> Self {
        self.amount_due_cents = Some(value);
        self
    }

    pub fn subtotal_cents(mut self, value: i64) -> Self {
        self.subtotal_cents = Some(value);
        self
    }

    pub fn tax_cents(mut self, value: i64) -> Self {
        self.tax_cents = Some(value);
        self
    }

    pub fn discount_percent_off(mut self, value: f64) -> Self {
        self.discount_percent_off = Some(value);
        self
    }

    pub fn discount_amount_off(mut self, value: f64) -> Self {
        self.discount_amount_off = Some(value);
        self
    }

    pub fn discounts(mut self, value: Vec<DiscountResponseModel>) -> Self {
        self.discounts = Some(value);
        self
    }

    pub fn next_payment_attempt_unix(mut self, value: i64) -> Self {
        self.next_payment_attempt_unix = Some(value);
        self
    }

    pub fn payment_intent_status(mut self, value: InvoiceResponsePaymentIntentStatus) -> Self {
        self.payment_intent_status = Some(value);
        self
    }

    pub fn payment_intent_statusses(mut self, value: Vec<InvoiceResponsePaymentIntentStatussesItem>) -> Self {
        self.payment_intent_statusses = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoiceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount_due_cents`](InvoiceResponseBuilder::amount_due_cents)
    /// - [`discounts`](InvoiceResponseBuilder::discounts)
    /// - [`next_payment_attempt_unix`](InvoiceResponseBuilder::next_payment_attempt_unix)
    /// - [`payment_intent_statusses`](InvoiceResponseBuilder::payment_intent_statusses)
    pub fn build(self) -> Result<InvoiceResponse, BuildError> {
        Ok(InvoiceResponse {
            amount_due_cents: self.amount_due_cents.ok_or_else(|| BuildError::missing_field("amount_due_cents"))?,
            subtotal_cents: self.subtotal_cents,
            tax_cents: self.tax_cents,
            discount_percent_off: self.discount_percent_off,
            discount_amount_off: self.discount_amount_off,
            discounts: self.discounts.ok_or_else(|| BuildError::missing_field("discounts"))?,
            next_payment_attempt_unix: self.next_payment_attempt_unix.ok_or_else(|| BuildError::missing_field("next_payment_attempt_unix"))?,
            payment_intent_status: self.payment_intent_status,
            payment_intent_statusses: self.payment_intent_statusses.ok_or_else(|| BuildError::missing_field("payment_intent_statusses"))?,
        })
    }
}
