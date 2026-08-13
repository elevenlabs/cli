pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TextToDialogueWebsocketFinalMulti {
    /// Marks the end of this context's closing flush sequence.
    pub is_final: bool,
    /// The context that has been finalized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
}

impl TextToDialogueWebsocketFinalMulti {
    pub fn builder() -> TextToDialogueWebsocketFinalMultiBuilder {
        <TextToDialogueWebsocketFinalMultiBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TextToDialogueWebsocketFinalMultiBuilder {
    is_final: Option<bool>,
    context_id: Option<String>,
}

impl TextToDialogueWebsocketFinalMultiBuilder {
    pub fn is_final(mut self, value: bool) -> Self {
        self.is_final = Some(value);
        self
    }

    pub fn context_id(mut self, value: impl Into<String>) -> Self {
        self.context_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TextToDialogueWebsocketFinalMulti`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_final`](TextToDialogueWebsocketFinalMultiBuilder::is_final)
    pub fn build(self) -> Result<TextToDialogueWebsocketFinalMulti, BuildError> {
        Ok(TextToDialogueWebsocketFinalMulti {
            is_final: self.is_final.ok_or_else(|| BuildError::missing_field("is_final"))?,
            context_id: self.context_id,
        })
    }
}
