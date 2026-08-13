pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RegisterMediaResponse {
    /// The ID of the uploaded media file.
    #[serde(default)]
    pub media_id: MediaId,
}

impl RegisterMediaResponse {
    pub fn builder() -> RegisterMediaResponseBuilder {
        <RegisterMediaResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RegisterMediaResponseBuilder {
    media_id: Option<MediaId>,
}

impl RegisterMediaResponseBuilder {
    pub fn media_id(mut self, value: MediaId) -> Self {
        self.media_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RegisterMediaResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`media_id`](RegisterMediaResponseBuilder::media_id)
    pub fn build(self) -> Result<RegisterMediaResponse, BuildError> {
        Ok(RegisterMediaResponse {
            media_id: self.media_id.ok_or_else(|| BuildError::missing_field("media_id"))?,
        })
    }
}
