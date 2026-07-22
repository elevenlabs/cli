pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Delete an existing staff member from the system.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteStaffParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl DeleteStaffParams {
    pub fn builder() -> DeleteStaffParamsBuilder {
        <DeleteStaffParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteStaffParamsBuilder {
    smb_tool_type: Option<String>,
}

impl DeleteStaffParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteStaffParams`].
    pub fn build(self) -> Result<DeleteStaffParams, BuildError> {
        Ok(DeleteStaffParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
