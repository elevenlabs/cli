pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OptOutSmsReminderParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl OptOutSmsReminderParams {
    pub fn builder() -> OptOutSmsReminderParamsBuilder {
        <OptOutSmsReminderParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OptOutSmsReminderParamsBuilder {
    smb_tool_type: Option<String>,
}

impl OptOutSmsReminderParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OptOutSmsReminderParams`].
    pub fn build(self) -> Result<OptOutSmsReminderParams, BuildError> {
        Ok(OptOutSmsReminderParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
