pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteHistoryItemResponse {
    /// The status of the deletion request. If the request was successful, the status will be 'ok'. Otherwise an error message with http code 500 will be returned.
    #[serde(default)]
    pub status: String,
}

impl DeleteHistoryItemResponse {
    pub fn builder() -> DeleteHistoryItemResponseBuilder {
        <DeleteHistoryItemResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteHistoryItemResponseBuilder {
    status: Option<String>,
}

impl DeleteHistoryItemResponseBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteHistoryItemResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](DeleteHistoryItemResponseBuilder::status)
    pub fn build(self) -> Result<DeleteHistoryItemResponse, BuildError> {
        Ok(DeleteHistoryItemResponse {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
