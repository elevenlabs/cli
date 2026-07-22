pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteAgentRuleParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl DeleteAgentRuleParams {
    pub fn builder() -> DeleteAgentRuleParamsBuilder {
        <DeleteAgentRuleParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteAgentRuleParamsBuilder {
    smb_tool_type: Option<String>,
}

impl DeleteAgentRuleParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteAgentRuleParams`].
    pub fn build(self) -> Result<DeleteAgentRuleParams, BuildError> {
        Ok(DeleteAgentRuleParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
