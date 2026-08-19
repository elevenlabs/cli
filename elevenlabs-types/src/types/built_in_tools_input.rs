pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// System tools a conversational agent can be given.
/// 
/// Deliberately not named ConversationalBuiltInTools: BuiltInToolsInput and
/// BuiltInToolsOutput are part of the public API spec and the generated SDKs.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BuiltInToolsInput {
    /// The transfer to agent tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_to_agent: Option<SystemToolConfigInput>,
    /// The end call tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_call: Option<SystemToolConfigInput>,
    /// The language detection tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_detection: Option<SystemToolConfigInput>,
    /// The transfer to number tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_to_number: Option<SystemToolConfigInput>,
    /// The skip turn tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_turn: Option<SystemToolConfigInput>,
    /// The play DTMF tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_keypad_touch_tone: Option<SystemToolConfigInput>,
    /// The voicemail detection tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voicemail_detection: Option<SystemToolConfigInput>,
}

impl BuiltInToolsInput {
    pub fn builder() -> BuiltInToolsInputBuilder {
        <BuiltInToolsInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BuiltInToolsInputBuilder {
    transfer_to_agent: Option<SystemToolConfigInput>,
    end_call: Option<SystemToolConfigInput>,
    language_detection: Option<SystemToolConfigInput>,
    transfer_to_number: Option<SystemToolConfigInput>,
    skip_turn: Option<SystemToolConfigInput>,
    play_keypad_touch_tone: Option<SystemToolConfigInput>,
    voicemail_detection: Option<SystemToolConfigInput>,
}

impl BuiltInToolsInputBuilder {
    pub fn transfer_to_agent(mut self, value: SystemToolConfigInput) -> Self {
        self.transfer_to_agent = Some(value);
        self
    }

    pub fn end_call(mut self, value: SystemToolConfigInput) -> Self {
        self.end_call = Some(value);
        self
    }

    pub fn language_detection(mut self, value: SystemToolConfigInput) -> Self {
        self.language_detection = Some(value);
        self
    }

    pub fn transfer_to_number(mut self, value: SystemToolConfigInput) -> Self {
        self.transfer_to_number = Some(value);
        self
    }

    pub fn skip_turn(mut self, value: SystemToolConfigInput) -> Self {
        self.skip_turn = Some(value);
        self
    }

    pub fn play_keypad_touch_tone(mut self, value: SystemToolConfigInput) -> Self {
        self.play_keypad_touch_tone = Some(value);
        self
    }

    pub fn voicemail_detection(mut self, value: SystemToolConfigInput) -> Self {
        self.voicemail_detection = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BuiltInToolsInput`].
    pub fn build(self) -> Result<BuiltInToolsInput, BuildError> {
        Ok(BuiltInToolsInput {
            transfer_to_agent: self.transfer_to_agent,
            end_call: self.end_call,
            language_detection: self.language_detection,
            transfer_to_number: self.transfer_to_number,
            skip_turn: self.skip_turn,
            play_keypad_touch_tone: self.play_keypad_touch_tone,
            voicemail_detection: self.voicemail_detection,
        })
    }
}
