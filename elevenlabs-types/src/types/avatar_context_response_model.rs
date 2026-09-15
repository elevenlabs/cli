pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AvatarContextResponseModel {
    /// The ID of the avatar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_id: Option<String>,
    /// The ID of the avatar style.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_style_id: Option<String>,
    /// The name of the avatar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_name: Option<String>,
    /// The name of the avatar style.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_style_name: Option<String>,
}

impl AvatarContextResponseModel {
    pub fn builder() -> AvatarContextResponseModelBuilder {
        <AvatarContextResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AvatarContextResponseModelBuilder {
    avatar_id: Option<String>,
    avatar_style_id: Option<String>,
    avatar_name: Option<String>,
    avatar_style_name: Option<String>,
}

impl AvatarContextResponseModelBuilder {
    pub fn avatar_id(mut self, value: impl Into<String>) -> Self {
        self.avatar_id = Some(value.into());
        self
    }

    pub fn avatar_style_id(mut self, value: impl Into<String>) -> Self {
        self.avatar_style_id = Some(value.into());
        self
    }

    pub fn avatar_name(mut self, value: impl Into<String>) -> Self {
        self.avatar_name = Some(value.into());
        self
    }

    pub fn avatar_style_name(mut self, value: impl Into<String>) -> Self {
        self.avatar_style_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AvatarContextResponseModel`].
    pub fn build(self) -> Result<AvatarContextResponseModel, BuildError> {
        Ok(AvatarContextResponseModel {
            avatar_id: self.avatar_id,
            avatar_style_id: self.avatar_style_id,
            avatar_name: self.avatar_name,
            avatar_style_name: self.avatar_style_name,
        })
    }
}
