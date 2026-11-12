pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A Voice Library voice the uploaded audio matched, offered in place of the clone.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LibraryVoiceMatchResponseModel {
    /// The ID of the matched Voice Library voice.
    #[serde(default)]
    pub library_voice_id: String,
    /// The public owner ID of the Voice Library voice. Pass it with library_voice_id to POST /v1/voices/add/{public_user_id}/{voice_id} to add the voice.
    #[serde(default)]
    pub public_owner_id: String,
}

impl LibraryVoiceMatchResponseModel {
    pub fn builder() -> LibraryVoiceMatchResponseModelBuilder {
        <LibraryVoiceMatchResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LibraryVoiceMatchResponseModelBuilder {
    library_voice_id: Option<String>,
    public_owner_id: Option<String>,
}

impl LibraryVoiceMatchResponseModelBuilder {
    pub fn library_voice_id(mut self, value: impl Into<String>) -> Self {
        self.library_voice_id = Some(value.into());
        self
    }

    pub fn public_owner_id(mut self, value: impl Into<String>) -> Self {
        self.public_owner_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LibraryVoiceMatchResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`library_voice_id`](LibraryVoiceMatchResponseModelBuilder::library_voice_id)
    /// - [`public_owner_id`](LibraryVoiceMatchResponseModelBuilder::public_owner_id)
    pub fn build(self) -> Result<LibraryVoiceMatchResponseModel, BuildError> {
        Ok(LibraryVoiceMatchResponseModel {
            library_voice_id: self.library_voice_id.ok_or_else(|| BuildError::missing_field("library_voice_id"))?,
            public_owner_id: self.public_owner_id.ok_or_else(|| BuildError::missing_field("public_owner_id"))?,
        })
    }
}
