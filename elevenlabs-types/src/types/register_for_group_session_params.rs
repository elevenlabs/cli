pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Register a client for a scheduled group session.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RegisterForGroupSessionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl RegisterForGroupSessionParams {
    pub fn builder() -> RegisterForGroupSessionParamsBuilder {
        <RegisterForGroupSessionParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RegisterForGroupSessionParamsBuilder {
    smb_tool_type: Option<String>,
}

impl RegisterForGroupSessionParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RegisterForGroupSessionParams`].
    pub fn build(self) -> Result<RegisterForGroupSessionParams, BuildError> {
        Ok(RegisterForGroupSessionParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
