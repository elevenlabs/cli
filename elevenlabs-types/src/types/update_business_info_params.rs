pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateBusinessInfoParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl UpdateBusinessInfoParams {
    pub fn builder() -> UpdateBusinessInfoParamsBuilder {
        <UpdateBusinessInfoParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateBusinessInfoParamsBuilder {
    smb_tool_type: Option<String>,
}

impl UpdateBusinessInfoParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateBusinessInfoParams`].
    pub fn build(self) -> Result<UpdateBusinessInfoParams, BuildError> {
        Ok(UpdateBusinessInfoParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
