pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TextToDialogueWebsocketVoiceInput {
    /// Text appended for this voice. Buffered with prior text until the server triggers generation.
    #[serde(default)]
    pub text: String,
    /// Must be one of the IDs from the initial `voices` array.
    #[serde(default)]
    pub voice_id: String,
    /// When `true`, the server finalizes the current pending segment (as if the speaker finished their turn) before applying this input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_turn: Option<bool>,
}

impl TextToDialogueWebsocketVoiceInput {
    pub fn builder() -> TextToDialogueWebsocketVoiceInputBuilder {
        <TextToDialogueWebsocketVoiceInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TextToDialogueWebsocketVoiceInputBuilder {
    text: Option<String>,
    voice_id: Option<String>,
    new_turn: Option<bool>,
}

impl TextToDialogueWebsocketVoiceInputBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    pub fn new_turn(mut self, value: bool) -> Self {
        self.new_turn = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TextToDialogueWebsocketVoiceInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](TextToDialogueWebsocketVoiceInputBuilder::text)
    /// - [`voice_id`](TextToDialogueWebsocketVoiceInputBuilder::voice_id)
    pub fn build(self) -> Result<TextToDialogueWebsocketVoiceInput, BuildError> {
        Ok(TextToDialogueWebsocketVoiceInput {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            voice_id: self.voice_id.ok_or_else(|| BuildError::missing_field("voice_id"))?,
            new_turn: self.new_turn,
        })
    }
}
