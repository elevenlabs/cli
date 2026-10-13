pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UserVoiceCollectionResponseModel {
    /// The ID of the voice collection.
    #[serde(default)]
    pub collection_id: String,
    /// The title of the voice collection.
    #[serde(default)]
    pub title: String,
    /// The icon of the voice collection.
    #[serde(default)]
    pub icon: String,
    /// The caller's access role on the voice collection.
    pub permission_on_resource: UserVoiceCollectionResponseModelPermissionOnResource,
}

impl UserVoiceCollectionResponseModel {
    pub fn builder() -> UserVoiceCollectionResponseModelBuilder {
        <UserVoiceCollectionResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserVoiceCollectionResponseModelBuilder {
    collection_id: Option<String>,
    title: Option<String>,
    icon: Option<String>,
    permission_on_resource: Option<UserVoiceCollectionResponseModelPermissionOnResource>,
}

impl UserVoiceCollectionResponseModelBuilder {
    pub fn collection_id(mut self, value: impl Into<String>) -> Self {
        self.collection_id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn icon(mut self, value: impl Into<String>) -> Self {
        self.icon = Some(value.into());
        self
    }

    pub fn permission_on_resource(mut self, value: UserVoiceCollectionResponseModelPermissionOnResource) -> Self {
        self.permission_on_resource = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UserVoiceCollectionResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`collection_id`](UserVoiceCollectionResponseModelBuilder::collection_id)
    /// - [`title`](UserVoiceCollectionResponseModelBuilder::title)
    /// - [`icon`](UserVoiceCollectionResponseModelBuilder::icon)
    /// - [`permission_on_resource`](UserVoiceCollectionResponseModelBuilder::permission_on_resource)
    pub fn build(self) -> Result<UserVoiceCollectionResponseModel, BuildError> {
        Ok(UserVoiceCollectionResponseModel {
            collection_id: self.collection_id.ok_or_else(|| BuildError::missing_field("collection_id"))?,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            icon: self.icon.ok_or_else(|| BuildError::missing_field("icon"))?,
            permission_on_resource: self.permission_on_resource.ok_or_else(|| BuildError::missing_field("permission_on_resource"))?,
        })
    }
}
