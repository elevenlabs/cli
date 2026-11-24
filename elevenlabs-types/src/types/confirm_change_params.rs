pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The owner's spoken answer to changes the assistant read back to them.
/// 
/// Runs in the agent runtime rather than Reception: it needs the conversation's
/// chat to check the owner actually answered after hearing the changes.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConfirmChangeParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl ConfirmChangeParams {
    pub fn builder() -> ConfirmChangeParamsBuilder {
        <ConfirmChangeParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConfirmChangeParamsBuilder {
    smb_tool_type: Option<String>,
}

impl ConfirmChangeParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConfirmChangeParams`].
    pub fn build(self) -> Result<ConfirmChangeParams, BuildError> {
        Ok(ConfirmChangeParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
