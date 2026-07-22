pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAgentProcedureParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl CreateAgentProcedureParams {
    pub fn builder() -> CreateAgentProcedureParamsBuilder {
        <CreateAgentProcedureParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAgentProcedureParamsBuilder {
    smb_tool_type: Option<String>,
}

impl CreateAgentProcedureParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAgentProcedureParams`].
    pub fn build(self) -> Result<CreateAgentProcedureParams, BuildError> {
        Ok(CreateAgentProcedureParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
