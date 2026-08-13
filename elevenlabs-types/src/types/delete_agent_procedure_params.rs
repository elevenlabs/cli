pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteAgentProcedureParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl DeleteAgentProcedureParams {
    pub fn builder() -> DeleteAgentProcedureParamsBuilder {
        <DeleteAgentProcedureParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteAgentProcedureParamsBuilder {
    smb_tool_type: Option<String>,
}

impl DeleteAgentProcedureParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteAgentProcedureParams`].
    pub fn build(self) -> Result<DeleteAgentProcedureParams, BuildError> {
        Ok(DeleteAgentProcedureParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
