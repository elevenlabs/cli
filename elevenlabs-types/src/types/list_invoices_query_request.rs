pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_invoices
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListInvoicesQueryRequest {
    /// Pagination cursor from previous response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Number of invoices per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

impl ListInvoicesQueryRequest {
    pub fn builder() -> ListInvoicesQueryRequestBuilder {
        <ListInvoicesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListInvoicesQueryRequestBuilder {
    cursor: Option<String>,
    page_size: Option<i64>,
}

impl ListInvoicesQueryRequestBuilder {
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListInvoicesQueryRequest`].
    pub fn build(self) -> Result<ListInvoicesQueryRequest, BuildError> {
        Ok(ListInvoicesQueryRequest {
            cursor: self.cursor,
            page_size: self.page_size,
        })
    }
}

