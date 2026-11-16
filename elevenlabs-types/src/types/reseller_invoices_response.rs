pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResellerInvoicesResponse {
    #[serde(default)]
    pub invoices: Vec<ResellerInvoiceSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ResellerInvoicesResponse {
    pub fn builder() -> ResellerInvoicesResponseBuilder {
        <ResellerInvoicesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResellerInvoicesResponseBuilder {
    invoices: Option<Vec<ResellerInvoiceSummary>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl ResellerInvoicesResponseBuilder {
    pub fn invoices(mut self, value: Vec<ResellerInvoiceSummary>) -> Self {
        self.invoices = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResellerInvoicesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoices`](ResellerInvoicesResponseBuilder::invoices)
    pub fn build(self) -> Result<ResellerInvoicesResponse, BuildError> {
        Ok(ResellerInvoicesResponse {
            invoices: self.invoices.ok_or_else(|| BuildError::missing_field("invoices"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more,
        })
    }
}
