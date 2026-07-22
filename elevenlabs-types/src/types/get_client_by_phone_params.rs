pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Look up a client by their exact phone number.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetClientByPhoneParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl GetClientByPhoneParams {
    pub fn builder() -> GetClientByPhoneParamsBuilder {
        <GetClientByPhoneParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetClientByPhoneParamsBuilder {
    smb_tool_type: Option<String>,
}

impl GetClientByPhoneParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetClientByPhoneParams`].
    pub fn build(self) -> Result<GetClientByPhoneParams, BuildError> {
        Ok(GetClientByPhoneParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
