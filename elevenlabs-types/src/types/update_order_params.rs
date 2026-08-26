pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateOrderParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl UpdateOrderParams {
    pub fn builder() -> UpdateOrderParamsBuilder {
        <UpdateOrderParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateOrderParamsBuilder {
    smb_tool_type: Option<String>,
}

impl UpdateOrderParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateOrderParams`].
    pub fn build(self) -> Result<UpdateOrderParams, BuildError> {
        Ok(UpdateOrderParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
