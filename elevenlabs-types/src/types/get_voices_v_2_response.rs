pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetVoicesV2Response {
    /// The list of voices matching the query.
    #[serde(default)]
    pub voices: Vec<Voice>,
    /// Indicates whether there are more voices available in subsequent pages. Use this flag (and next_page_token) for reliable pagination instead of relying on total_count.
    #[serde(default)]
    pub has_more: bool,
    /// The total count of voices matching the query. This value is a live snapshot that reflects the current state of the database and may change between requests as users create, modify, or delete voices. For reliable pagination, use the has_more flag instead of relying on this value. Only request this field when you actually need the total count (e.g., for display purposes), as calculating it incurs a performance cost.
    #[serde(default)]
    pub total_count: i64,
    /// Token to retrieve the next page of results. Pass this value to the next request to continue pagination. Null if there are no more results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl GetVoicesV2Response {
    pub fn builder() -> GetVoicesV2ResponseBuilder {
        <GetVoicesV2ResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetVoicesV2ResponseBuilder {
    voices: Option<Vec<Voice>>,
    has_more: Option<bool>,
    total_count: Option<i64>,
    next_page_token: Option<String>,
}

impl GetVoicesV2ResponseBuilder {
    pub fn voices(mut self, value: Vec<Voice>) -> Self {
        self.voices = Some(value);
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn total_count(mut self, value: i64) -> Self {
        self.total_count = Some(value);
        self
    }

    pub fn next_page_token(mut self, value: impl Into<String>) -> Self {
        self.next_page_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetVoicesV2Response`].
    /// This method will fail if any of the following fields are not set:
    /// - [`voices`](GetVoicesV2ResponseBuilder::voices)
    /// - [`has_more`](GetVoicesV2ResponseBuilder::has_more)
    /// - [`total_count`](GetVoicesV2ResponseBuilder::total_count)
    pub fn build(self) -> Result<GetVoicesV2Response, BuildError> {
        Ok(GetVoicesV2Response {
            voices: self.voices.ok_or_else(|| BuildError::missing_field("voices"))?,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
            total_count: self.total_count.ok_or_else(|| BuildError::missing_field("total_count"))?,
            next_page_token: self.next_page_token,
        })
    }
}
