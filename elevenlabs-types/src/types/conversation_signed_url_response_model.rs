pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationSignedUrlResponseModel {
    #[serde(default)]
    pub signed_url: String,
}

impl ConversationSignedUrlResponseModel {
    pub fn builder() -> ConversationSignedUrlResponseModelBuilder {
        <ConversationSignedUrlResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationSignedUrlResponseModelBuilder {
    signed_url: Option<String>,
}

impl ConversationSignedUrlResponseModelBuilder {
    pub fn signed_url(mut self, value: impl Into<String>) -> Self {
        self.signed_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationSignedUrlResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`signed_url`](ConversationSignedUrlResponseModelBuilder::signed_url)
    pub fn build(self) -> Result<ConversationSignedUrlResponseModel, BuildError> {
        Ok(ConversationSignedUrlResponseModel {
            signed_url: self.signed_url.ok_or_else(|| BuildError::missing_field("signed_url"))?,
        })
    }
}
