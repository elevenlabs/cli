pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_v2
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListV2QueryRequest {
    /// Number of phone numbers per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Filter by phone number ID, label, or phone number. A phone number ID must match exactly; label and phone number matching is a case-insensitive substring.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Filter by label. Matching is a case-insensitive substring.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Filter by phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// Filter by telephony provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<TelephonyProvider>,
    /// Filter by whether the phone number can place outbound calls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_outbound: Option<bool>,
    /// Filter by assigned agent ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// Filter by assigned branch ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    /// The field to sort the results by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<PhoneNumberSortBy>,
    /// The direction to sort the results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<SortDirection>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl ListV2QueryRequest {
    pub fn builder() -> ListV2QueryRequestBuilder {
        <ListV2QueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListV2QueryRequestBuilder {
    page_size: Option<i64>,
    search: Option<String>,
    label: Option<String>,
    phone_number: Option<String>,
    provider: Option<TelephonyProvider>,
    supports_outbound: Option<bool>,
    agent_id: Option<String>,
    branch_id: Option<String>,
    sort_by: Option<PhoneNumberSortBy>,
    sort_direction: Option<SortDirection>,
    cursor: Option<String>,
}

impl ListV2QueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn phone_number(mut self, value: impl Into<String>) -> Self {
        self.phone_number = Some(value.into());
        self
    }

    pub fn provider(mut self, value: TelephonyProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn supports_outbound(mut self, value: bool) -> Self {
        self.supports_outbound = Some(value);
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    pub fn sort_by(mut self, value: PhoneNumberSortBy) -> Self {
        self.sort_by = Some(value);
        self
    }

    pub fn sort_direction(mut self, value: SortDirection) -> Self {
        self.sort_direction = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListV2QueryRequest`].
    pub fn build(self) -> Result<ListV2QueryRequest, BuildError> {
        Ok(ListV2QueryRequest {
            page_size: self.page_size,
            search: self.search,
            label: self.label,
            phone_number: self.phone_number,
            provider: self.provider,
            supports_outbound: self.supports_outbound,
            agent_id: self.agent_id,
            branch_id: self.branch_id,
            sort_by: self.sort_by,
            sort_direction: self.sort_direction,
            cursor: self.cursor,
        })
    }
}

