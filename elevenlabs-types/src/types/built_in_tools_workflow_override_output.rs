pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BuiltInToolsWorkflowOverrideOutput {
    /// The end call tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_call: Option<SystemToolConfigOutput>,
    /// The language detection tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_detection: Option<SystemToolConfigOutput>,
    /// The transfer to agent tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_to_agent: Option<SystemToolConfigOutput>,
    /// The transfer to number tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_to_number: Option<SystemToolConfigOutput>,
    /// The skip turn tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_turn: Option<SystemToolConfigOutput>,
    /// The play DTMF tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_keypad_touch_tone: Option<SystemToolConfigOutput>,
    /// The voicemail detection tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voicemail_detection: Option<SystemToolConfigOutput>,
}

impl BuiltInToolsWorkflowOverrideOutput {
    pub fn builder() -> BuiltInToolsWorkflowOverrideOutputBuilder {
        <BuiltInToolsWorkflowOverrideOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BuiltInToolsWorkflowOverrideOutputBuilder {
    end_call: Option<SystemToolConfigOutput>,
    language_detection: Option<SystemToolConfigOutput>,
    transfer_to_agent: Option<SystemToolConfigOutput>,
    transfer_to_number: Option<SystemToolConfigOutput>,
    skip_turn: Option<SystemToolConfigOutput>,
    play_keypad_touch_tone: Option<SystemToolConfigOutput>,
    voicemail_detection: Option<SystemToolConfigOutput>,
}

impl BuiltInToolsWorkflowOverrideOutputBuilder {
    pub fn end_call(mut self, value: SystemToolConfigOutput) -> Self {
        self.end_call = Some(value);
        self
    }

    pub fn language_detection(mut self, value: SystemToolConfigOutput) -> Self {
        self.language_detection = Some(value);
        self
    }

    pub fn transfer_to_agent(mut self, value: SystemToolConfigOutput) -> Self {
        self.transfer_to_agent = Some(value);
        self
    }

    pub fn transfer_to_number(mut self, value: SystemToolConfigOutput) -> Self {
        self.transfer_to_number = Some(value);
        self
    }

    pub fn skip_turn(mut self, value: SystemToolConfigOutput) -> Self {
        self.skip_turn = Some(value);
        self
    }

    pub fn play_keypad_touch_tone(mut self, value: SystemToolConfigOutput) -> Self {
        self.play_keypad_touch_tone = Some(value);
        self
    }

    pub fn voicemail_detection(mut self, value: SystemToolConfigOutput) -> Self {
        self.voicemail_detection = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BuiltInToolsWorkflowOverrideOutput`].
    pub fn build(self) -> Result<BuiltInToolsWorkflowOverrideOutput, BuildError> {
        Ok(BuiltInToolsWorkflowOverrideOutput {
            end_call: self.end_call,
            language_detection: self.language_detection,
            transfer_to_agent: self.transfer_to_agent,
            transfer_to_number: self.transfer_to_number,
            skip_turn: self.skip_turn,
            play_keypad_touch_tone: self.play_keypad_touch_tone,
            voicemail_detection: self.voicemail_detection,
        })
    }
}
