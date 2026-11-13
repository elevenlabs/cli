pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddVoiceIvcResponseModel {
    /// The ID of the newly created voice.
    #[serde(default)]
    pub voice_id: String,
    /// Whether the voice requires verification
    #[serde(default)]
    pub requires_verification: bool,
    /// Set when the samples match a protected voice that is available in the Voice Library. The new voice requires verification before use; adding the library voice is the alternative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub library_voice_match: Option<LibraryVoiceMatchResponseModel>,
}

impl AddVoiceIvcResponseModel {
    pub fn builder() -> AddVoiceIvcResponseModelBuilder {
        <AddVoiceIvcResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddVoiceIvcResponseModelBuilder {
    voice_id: Option<String>,
    requires_verification: Option<bool>,
    library_voice_match: Option<LibraryVoiceMatchResponseModel>,
}

impl AddVoiceIvcResponseModelBuilder {
    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    pub fn requires_verification(mut self, value: bool) -> Self {
        self.requires_verification = Some(value);
        self
    }

    pub fn library_voice_match(mut self, value: LibraryVoiceMatchResponseModel) -> Self {
        self.library_voice_match = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddVoiceIvcResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`voice_id`](AddVoiceIvcResponseModelBuilder::voice_id)
    /// - [`requires_verification`](AddVoiceIvcResponseModelBuilder::requires_verification)
    pub fn build(self) -> Result<AddVoiceIvcResponseModel, BuildError> {
        Ok(AddVoiceIvcResponseModel {
            voice_id: self.voice_id.ok_or_else(|| BuildError::missing_field("voice_id"))?,
            requires_verification: self.requires_verification.ok_or_else(|| BuildError::missing_field("requires_verification"))?,
            library_voice_match: self.library_voice_match,
        })
    }
}
