pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Cancel a single client's registration for a group session.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CancelGroupSessionRegistrationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl CancelGroupSessionRegistrationParams {
    pub fn builder() -> CancelGroupSessionRegistrationParamsBuilder {
        <CancelGroupSessionRegistrationParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelGroupSessionRegistrationParamsBuilder {
    smb_tool_type: Option<String>,
}

impl CancelGroupSessionRegistrationParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CancelGroupSessionRegistrationParams`].
    pub fn build(self) -> Result<CancelGroupSessionRegistrationParams, BuildError> {
        Ok(CancelGroupSessionRegistrationParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
