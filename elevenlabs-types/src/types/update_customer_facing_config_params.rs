pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCustomerFacingConfigParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl UpdateCustomerFacingConfigParams {
    pub fn builder() -> UpdateCustomerFacingConfigParamsBuilder {
        <UpdateCustomerFacingConfigParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCustomerFacingConfigParamsBuilder {
    smb_tool_type: Option<String>,
}

impl UpdateCustomerFacingConfigParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateCustomerFacingConfigParams`].
    pub fn build(self) -> Result<UpdateCustomerFacingConfigParams, BuildError> {
        Ok(UpdateCustomerFacingConfigParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
