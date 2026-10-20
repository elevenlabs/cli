pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAgentConfigParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl UpdateAgentConfigParams {
    pub fn builder() -> UpdateAgentConfigParamsBuilder {
        <UpdateAgentConfigParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAgentConfigParamsBuilder {
    smb_tool_type: Option<String>,
}

impl UpdateAgentConfigParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateAgentConfigParams`].
    pub fn build(self) -> Result<UpdateAgentConfigParams, BuildError> {
        Ok(UpdateAgentConfigParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
