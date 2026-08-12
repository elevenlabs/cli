pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateOrderResponse {
    /// The updated order name.
    #[serde(default)]
    pub name: String,
}

impl UpdateOrderResponse {
    pub fn builder() -> UpdateOrderResponseBuilder {
        <UpdateOrderResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateOrderResponseBuilder {
    name: Option<String>,
}

impl UpdateOrderResponseBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateOrderResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](UpdateOrderResponseBuilder::name)
    pub fn build(self) -> Result<UpdateOrderResponse, BuildError> {
        Ok(UpdateOrderResponse {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
