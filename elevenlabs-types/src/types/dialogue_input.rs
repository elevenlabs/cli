pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DialogueInput {
    /// The text to be converted into speech.
    #[serde(default)]
    pub text: String,
    /// The ID of the voice to be used for the generation.
    #[serde(default)]
    pub voice_id: String,
}

impl DialogueInput {
    pub fn builder() -> DialogueInputBuilder {
        <DialogueInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DialogueInputBuilder {
    text: Option<String>,
    voice_id: Option<String>,
}

impl DialogueInputBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DialogueInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](DialogueInputBuilder::text)
    /// - [`voice_id`](DialogueInputBuilder::voice_id)
    pub fn build(self) -> Result<DialogueInput, BuildError> {
        Ok(DialogueInput {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            voice_id: self.voice_id.ok_or_else(|| BuildError::missing_field("voice_id"))?,
        })
    }
}
