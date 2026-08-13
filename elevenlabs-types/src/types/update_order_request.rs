pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateOrderRequest {
    /// The new name for the order.
    #[serde(default)]
    pub name: String,
}

impl UpdateOrderRequest {
    pub fn builder() -> UpdateOrderRequestBuilder {
        <UpdateOrderRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateOrderRequestBuilder {
    name: Option<String>,
}

impl UpdateOrderRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateOrderRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](UpdateOrderRequestBuilder::name)
    pub fn build(self) -> Result<UpdateOrderRequest, BuildError> {
        Ok(UpdateOrderRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
