pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LeaveMessageParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl LeaveMessageParams {
    pub fn builder() -> LeaveMessageParamsBuilder {
        <LeaveMessageParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LeaveMessageParamsBuilder {
    smb_tool_type: Option<String>,
}

impl LeaveMessageParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LeaveMessageParams`].
    pub fn build(self) -> Result<LeaveMessageParams, BuildError> {
        Ok(LeaveMessageParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
