pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateOrderParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl CreateOrderParams {
    pub fn builder() -> CreateOrderParamsBuilder {
        <CreateOrderParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderParamsBuilder {
    smb_tool_type: Option<String>,
}

impl CreateOrderParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateOrderParams`].
    pub fn build(self) -> Result<CreateOrderParams, BuildError> {
        Ok(CreateOrderParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
