pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OptInSmsReminderParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl OptInSmsReminderParams {
    pub fn builder() -> OptInSmsReminderParamsBuilder {
        <OptInSmsReminderParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OptInSmsReminderParamsBuilder {
    smb_tool_type: Option<String>,
}

impl OptInSmsReminderParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OptInSmsReminderParams`].
    pub fn build(self) -> Result<OptInSmsReminderParams, BuildError> {
        Ok(OptInSmsReminderParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
