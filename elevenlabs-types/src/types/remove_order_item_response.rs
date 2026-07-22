pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RemoveOrderItemResponse {
    /// Whether the item was successfully removed.
    #[serde(default)]
    pub success: bool,
}

impl RemoveOrderItemResponse {
    pub fn builder() -> RemoveOrderItemResponseBuilder {
        <RemoveOrderItemResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RemoveOrderItemResponseBuilder {
    success: Option<bool>,
}

impl RemoveOrderItemResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RemoveOrderItemResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](RemoveOrderItemResponseBuilder::success)
    pub fn build(self) -> Result<RemoveOrderItemResponse, BuildError> {
        Ok(RemoveOrderItemResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
