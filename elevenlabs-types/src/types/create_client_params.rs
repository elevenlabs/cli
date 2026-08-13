pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Create a new client in the system.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateClientParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl CreateClientParams {
    pub fn builder() -> CreateClientParamsBuilder {
        <CreateClientParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateClientParamsBuilder {
    smb_tool_type: Option<String>,
}

impl CreateClientParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateClientParams`].
    pub fn build(self) -> Result<CreateClientParams, BuildError> {
        Ok(CreateClientParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
