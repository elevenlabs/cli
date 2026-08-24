pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Update an existing client's information.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateClientParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl UpdateClientParams {
    pub fn builder() -> UpdateClientParamsBuilder {
        <UpdateClientParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateClientParamsBuilder {
    smb_tool_type: Option<String>,
}

impl UpdateClientParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateClientParams`].
    pub fn build(self) -> Result<UpdateClientParams, BuildError> {
        Ok(UpdateClientParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
