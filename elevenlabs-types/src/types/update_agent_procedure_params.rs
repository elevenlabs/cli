pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAgentProcedureParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl UpdateAgentProcedureParams {
    pub fn builder() -> UpdateAgentProcedureParamsBuilder {
        <UpdateAgentProcedureParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAgentProcedureParamsBuilder {
    smb_tool_type: Option<String>,
}

impl UpdateAgentProcedureParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateAgentProcedureParams`].
    pub fn build(self) -> Result<UpdateAgentProcedureParams, BuildError> {
        Ok(UpdateAgentProcedureParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
