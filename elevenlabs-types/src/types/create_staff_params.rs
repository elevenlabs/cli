pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Create a new staff member in the system.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateStaffParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl CreateStaffParams {
    pub fn builder() -> CreateStaffParamsBuilder {
        <CreateStaffParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateStaffParamsBuilder {
    smb_tool_type: Option<String>,
}

impl CreateStaffParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateStaffParams`].
    pub fn build(self) -> Result<CreateStaffParams, BuildError> {
        Ok(CreateStaffParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
