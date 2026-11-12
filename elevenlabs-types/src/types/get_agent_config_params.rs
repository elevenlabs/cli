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
/// 
/// Also returns the read-only call-handling settings the owner edits in the
/// app: ``staff_answers_first`` (whether the staff phone rings before the
/// receptionist, the staff phone number, the maximum ring time, the staff
/// hours per day with their timezone, and whether those hours cover the
/// current time), ``turn_eagerness`` (response speed: eager / normal /
/// patient), ``background_voice_detection`` (filter background speech on/off),
/// ``tool_call_sound`` (hold sound; null means off) and
/// ``enable_play_keypad_touch_tone_tool`` (allow touch tones). Call it to
/// answer what any of these are currently set to, or to check the setup when
/// the owner says calls are not reaching the staff phone.
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
