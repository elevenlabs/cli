pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// List every customer-facing agent on the workspace.
/// 
/// The assistant uses this whenever it needs to act on a specific customer-facing
/// agent (rules, config edits, etc.) so it can pick the right ``agent_id`` to pass
/// to mutating tools. Mirrors the ``list_services`` / ``list_clients``
/// pattern: read once, then mutate by id.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListCustomerFacingAgentsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl ListCustomerFacingAgentsParams {
    pub fn builder() -> ListCustomerFacingAgentsParamsBuilder {
        <ListCustomerFacingAgentsParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCustomerFacingAgentsParamsBuilder {
    smb_tool_type: Option<String>,
}

impl ListCustomerFacingAgentsParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListCustomerFacingAgentsParams`].
    pub fn build(self) -> Result<ListCustomerFacingAgentsParams, BuildError> {
        Ok(ListCustomerFacingAgentsParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
