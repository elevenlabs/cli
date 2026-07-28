pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Delete a client, cascading deletion of all their appointments first.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteClientParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl DeleteClientParams {
    pub fn builder() -> DeleteClientParamsBuilder {
        <DeleteClientParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteClientParamsBuilder {
    smb_tool_type: Option<String>,
}

impl DeleteClientParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteClientParams`].
    pub fn build(self) -> Result<DeleteClientParams, BuildError> {
        Ok(DeleteClientParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
