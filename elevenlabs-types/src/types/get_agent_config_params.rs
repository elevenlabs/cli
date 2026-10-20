pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Return the full editable config of a single receptionist, including the
/// current ``additional_instructions`` text.
/// 
/// ``list_customer_facing_agents`` is intentionally lightweight (metadata and a
/// truncated greeting preview) and omits ``additional_instructions``. Call this
/// tool whenever you need the current full text before calling
/// ``update_agent_config`` on ``first_message`` or ``additional_instructions``
/// so you can preserve what is already there.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAgentConfigParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl GetAgentConfigParams {
    pub fn builder() -> GetAgentConfigParamsBuilder {
        <GetAgentConfigParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAgentConfigParamsBuilder {
    smb_tool_type: Option<String>,
}

impl GetAgentConfigParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetAgentConfigParams`].
    pub fn build(self) -> Result<GetAgentConfigParams, BuildError> {
        Ok(GetAgentConfigParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
