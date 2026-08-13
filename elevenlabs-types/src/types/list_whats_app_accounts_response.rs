pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListWhatsAppAccountsResponse {
    #[serde(default)]
    pub items: Vec<GetWhatsAppAccountResponse>,
}

impl ListWhatsAppAccountsResponse {
    pub fn builder() -> ListWhatsAppAccountsResponseBuilder {
        <ListWhatsAppAccountsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListWhatsAppAccountsResponseBuilder {
    items: Option<Vec<GetWhatsAppAccountResponse>>,
}

impl ListWhatsAppAccountsResponseBuilder {
    pub fn items(mut self, value: Vec<GetWhatsAppAccountResponse>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListWhatsAppAccountsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](ListWhatsAppAccountsResponseBuilder::items)
    pub fn build(self) -> Result<ListWhatsAppAccountsResponse, BuildError> {
        Ok(ListWhatsAppAccountsResponse {
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
        })
    }
}
