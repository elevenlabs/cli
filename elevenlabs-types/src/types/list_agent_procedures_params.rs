pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListAgentProceduresParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl ListAgentProceduresParams {
    pub fn builder() -> ListAgentProceduresParamsBuilder {
        <ListAgentProceduresParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAgentProceduresParamsBuilder {
    smb_tool_type: Option<String>,
}

impl ListAgentProceduresParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListAgentProceduresParams`].
    pub fn build(self) -> Result<ListAgentProceduresParams, BuildError> {
        Ok(ListAgentProceduresParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
