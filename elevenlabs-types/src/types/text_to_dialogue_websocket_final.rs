pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct TextToDialogueWebsocketFinal {
    /// Marks the end of the closing flush sequence.
    pub is_final: bool,
}

impl TextToDialogueWebsocketFinal {
    pub fn builder() -> TextToDialogueWebsocketFinalBuilder {
        <TextToDialogueWebsocketFinalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TextToDialogueWebsocketFinalBuilder {
    is_final: Option<bool>,
}

impl TextToDialogueWebsocketFinalBuilder {
    pub fn is_final(mut self, value: bool) -> Self {
        self.is_final = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TextToDialogueWebsocketFinal`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_final`](TextToDialogueWebsocketFinalBuilder::is_final)
    pub fn build(self) -> Result<TextToDialogueWebsocketFinal, BuildError> {
        Ok(TextToDialogueWebsocketFinal {
            is_final: self.is_final.ok_or_else(|| BuildError::missing_field("is_final"))?,
        })
    }
}
