pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProjectVideoThumbnailSheetResponseModel {
    #[serde(default)]
    pub start_thumbnail_index: i64,
    #[serde(default)]
    pub thumbnail_count: i64,
    #[serde(default)]
    pub signed_cloud_url: String,
}

impl ProjectVideoThumbnailSheetResponseModel {
    pub fn builder() -> ProjectVideoThumbnailSheetResponseModelBuilder {
        <ProjectVideoThumbnailSheetResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProjectVideoThumbnailSheetResponseModelBuilder {
    start_thumbnail_index: Option<i64>,
    thumbnail_count: Option<i64>,
    signed_cloud_url: Option<String>,
}

impl ProjectVideoThumbnailSheetResponseModelBuilder {
    pub fn start_thumbnail_index(mut self, value: i64) -> Self {
        self.start_thumbnail_index = Some(value);
        self
    }

    pub fn thumbnail_count(mut self, value: i64) -> Self {
        self.thumbnail_count = Some(value);
        self
    }

    pub fn signed_cloud_url(mut self, value: impl Into<String>) -> Self {
        self.signed_cloud_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProjectVideoThumbnailSheetResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_thumbnail_index`](ProjectVideoThumbnailSheetResponseModelBuilder::start_thumbnail_index)
    /// - [`thumbnail_count`](ProjectVideoThumbnailSheetResponseModelBuilder::thumbnail_count)
    /// - [`signed_cloud_url`](ProjectVideoThumbnailSheetResponseModelBuilder::signed_cloud_url)
    pub fn build(self) -> Result<ProjectVideoThumbnailSheetResponseModel, BuildError> {
        Ok(ProjectVideoThumbnailSheetResponseModel {
            start_thumbnail_index: self.start_thumbnail_index.ok_or_else(|| BuildError::missing_field("start_thumbnail_index"))?,
            thumbnail_count: self.thumbnail_count.ok_or_else(|| BuildError::missing_field("thumbnail_count"))?,
            signed_cloud_url: self.signed_cloud_url.ok_or_else(|| BuildError::missing_field("signed_cloud_url"))?,
        })
    }
}
