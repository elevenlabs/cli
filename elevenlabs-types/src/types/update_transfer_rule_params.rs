pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateTransferRuleParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
    /// Whether to offer the post_dial_digits parameter, set from the receptionists' enable_play_keypad_touch_tone_tool config. Digits saved while that is off are dropped when the receptionist is built, so the parameter is hidden rather than accepted and ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_dial_digits_enabled: Option<bool>,
}

impl UpdateTransferRuleParams {
    pub fn builder() -> UpdateTransferRuleParamsBuilder {
        <UpdateTransferRuleParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateTransferRuleParamsBuilder {
    smb_tool_type: Option<String>,
    post_dial_digits_enabled: Option<bool>,
}

impl UpdateTransferRuleParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    pub fn post_dial_digits_enabled(mut self, value: bool) -> Self {
        self.post_dial_digits_enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateTransferRuleParams`].
    pub fn build(self) -> Result<UpdateTransferRuleParams, BuildError> {
        Ok(UpdateTransferRuleParams {
            smb_tool_type: self.smb_tool_type,
            post_dial_digits_enabled: self.post_dial_digits_enabled,
        })
    }
}
