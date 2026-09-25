pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetPhoneNumbersPageResponseModel {
    /// The phone numbers on this page
    #[serde(default)]
    pub phone_numbers: Vec<GetPhoneNumbersPageResponseModelPhoneNumbersItem>,
    /// Pass this value as `cursor` to fetch the next page. Null when there are no more results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Whether there are more results available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl GetPhoneNumbersPageResponseModel {
    pub fn builder() -> GetPhoneNumbersPageResponseModelBuilder {
        <GetPhoneNumbersPageResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPhoneNumbersPageResponseModelBuilder {
    phone_numbers: Option<Vec<GetPhoneNumbersPageResponseModelPhoneNumbersItem>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl GetPhoneNumbersPageResponseModelBuilder {
    pub fn phone_numbers(mut self, value: Vec<GetPhoneNumbersPageResponseModelPhoneNumbersItem>) -> Self {
        self.phone_numbers = Some(value);
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

    /// Consumes the builder and constructs a [`GetPhoneNumbersPageResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`phone_numbers`](GetPhoneNumbersPageResponseModelBuilder::phone_numbers)
    pub fn build(self) -> Result<GetPhoneNumbersPageResponseModel, BuildError> {
        Ok(GetPhoneNumbersPageResponseModel {
            phone_numbers: self.phone_numbers.ok_or_else(|| BuildError::missing_field("phone_numbers"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more,
        })
    }
}
