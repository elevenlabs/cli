pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DialogueInputResponseModel {
    /// The text of the dialogue input line.
    #[serde(default)]
    pub text: String,
    /// The ID of the voice used for this dialogue input line.
    #[serde(default)]
    pub voice_id: String,
    /// The name of the voice used for this dialogue input line.
    #[serde(default)]
    pub voice_name: String,
}

impl DialogueInputResponseModel {
    pub fn builder() -> DialogueInputResponseModelBuilder {
        <DialogueInputResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DialogueInputResponseModelBuilder {
    text: Option<String>,
    voice_id: Option<String>,
    voice_name: Option<String>,
}

impl DialogueInputResponseModelBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    pub fn voice_name(mut self, value: impl Into<String>) -> Self {
        self.voice_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DialogueInputResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](DialogueInputResponseModelBuilder::text)
    /// - [`voice_id`](DialogueInputResponseModelBuilder::voice_id)
    /// - [`voice_name`](DialogueInputResponseModelBuilder::voice_name)
    pub fn build(self) -> Result<DialogueInputResponseModel, BuildError> {
        Ok(DialogueInputResponseModel {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            voice_id: self.voice_id.ok_or_else(|| BuildError::missing_field("voice_id"))?,
            voice_name: self.voice_name.ok_or_else(|| BuildError::missing_field("voice_name"))?,
        })
    }
}
