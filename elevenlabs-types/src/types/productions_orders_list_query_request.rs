pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProductionsOrdersListQueryRequest {
    /// Maximum number of orders to return per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Number of orders to skip for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Filter orders by one or more statuses.
    #[serde(default)]
    pub status: Vec<Option<OrderState>>,
    /// Filter orders created on or after this date.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub start_date: Option<DateTime<FixedOffset>>,
    /// Filter orders created on or before this date.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub end_date: Option<DateTime<FixedOffset>>,
}

impl ProductionsOrdersListQueryRequest {
    pub fn builder() -> ProductionsOrdersListQueryRequestBuilder {
        <ProductionsOrdersListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProductionsOrdersListQueryRequestBuilder {
    page_size: Option<i64>,
    offset: Option<i64>,
    status: Option<Vec<Option<OrderState>>>,
    start_date: Option<DateTime<FixedOffset>>,
    end_date: Option<DateTime<FixedOffset>>,
}

impl ProductionsOrdersListQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn status(mut self, value: Vec<Option<OrderState>>) -> Self {
        self.status = Some(value);
        self
    }

    pub fn start_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.start_date = Some(value);
        self
    }

    pub fn end_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.end_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProductionsOrdersListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](ProductionsOrdersListQueryRequestBuilder::status)
    pub fn build(self) -> Result<ProductionsOrdersListQueryRequest, BuildError> {
        Ok(ProductionsOrdersListQueryRequest {
            page_size: self.page_size,
            offset: self.offset,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            start_date: self.start_date,
            end_date: self.end_date,
        })
    }
}

