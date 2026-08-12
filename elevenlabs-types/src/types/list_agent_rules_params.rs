pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListAgentRulesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl ListAgentRulesParams {
    pub fn builder() -> ListAgentRulesParamsBuilder {
        <ListAgentRulesParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAgentRulesParamsBuilder {
    smb_tool_type: Option<String>,
}

impl ListAgentRulesParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListAgentRulesParams`].
    pub fn build(self) -> Result<ListAgentRulesParams, BuildError> {
        Ok(ListAgentRulesParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
