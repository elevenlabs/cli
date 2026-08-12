pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateClientInteractionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl CreateClientInteractionParams {
    pub fn builder() -> CreateClientInteractionParamsBuilder {
        <CreateClientInteractionParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateClientInteractionParamsBuilder {
    smb_tool_type: Option<String>,
}

impl CreateClientInteractionParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateClientInteractionParams`].
    pub fn build(self) -> Result<CreateClientInteractionParams, BuildError> {
        Ok(CreateClientInteractionParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
