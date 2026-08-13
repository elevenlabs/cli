pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyAddSharedVoiceV1VoicesAddPublicUserIdVoiceIdPost {
    /// The name that identifies this voice. This will be displayed in the dropdown of the website.
    #[serde(default)]
    pub new_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bookmarked: Option<bool>,
}

impl BodyAddSharedVoiceV1VoicesAddPublicUserIdVoiceIdPost {
    pub fn builder() -> BodyAddSharedVoiceV1VoicesAddPublicUserIdVoiceIdPostBuilder {
        <BodyAddSharedVoiceV1VoicesAddPublicUserIdVoiceIdPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyAddSharedVoiceV1VoicesAddPublicUserIdVoiceIdPostBuilder {
    new_name: Option<String>,
    bookmarked: Option<bool>,
}

impl BodyAddSharedVoiceV1VoicesAddPublicUserIdVoiceIdPostBuilder {
    pub fn new_name(mut self, value: impl Into<String>) -> Self {
        self.new_name = Some(value.into());
        self
    }

    pub fn bookmarked(mut self, value: bool) -> Self {
        self.bookmarked = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyAddSharedVoiceV1VoicesAddPublicUserIdVoiceIdPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`new_name`](BodyAddSharedVoiceV1VoicesAddPublicUserIdVoiceIdPostBuilder::new_name)
    pub fn build(self) -> Result<BodyAddSharedVoiceV1VoicesAddPublicUserIdVoiceIdPost, BuildError> {
        Ok(BodyAddSharedVoiceV1VoicesAddPublicUserIdVoiceIdPost {
            new_name: self.new_name.ok_or_else(|| BuildError::missing_field("new_name"))?,
            bookmarked: self.bookmarked,
        })
    }
}

