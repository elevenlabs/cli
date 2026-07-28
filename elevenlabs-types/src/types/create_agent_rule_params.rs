pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAgentRuleParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl CreateAgentRuleParams {
    pub fn builder() -> CreateAgentRuleParamsBuilder {
        <CreateAgentRuleParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAgentRuleParamsBuilder {
    smb_tool_type: Option<String>,
}

impl CreateAgentRuleParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAgentRuleParams`].
    pub fn build(self) -> Result<CreateAgentRuleParams, BuildError> {
        Ok(CreateAgentRuleParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
