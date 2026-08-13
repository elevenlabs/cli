pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Delete an existing service from the system.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteServiceParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl DeleteServiceParams {
    pub fn builder() -> DeleteServiceParamsBuilder {
        <DeleteServiceParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteServiceParamsBuilder {
    smb_tool_type: Option<String>,
}

impl DeleteServiceParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteServiceParams`].
    pub fn build(self) -> Result<DeleteServiceParams, BuildError> {
        Ok(DeleteServiceParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
