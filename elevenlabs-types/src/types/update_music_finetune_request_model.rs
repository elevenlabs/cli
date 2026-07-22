pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateMusicFinetuneRequestModel {
    /// Updated name for the finetune.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Replacement set of tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Updated primary musical genre.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_genre: Option<String>,
    /// Finetune visibility. Only 'private' and 'workspace' can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<UpdateMusicFinetuneRequestModelVisibility>,
}

impl UpdateMusicFinetuneRequestModel {
    pub fn builder() -> UpdateMusicFinetuneRequestModelBuilder {
        <UpdateMusicFinetuneRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateMusicFinetuneRequestModelBuilder {
    name: Option<String>,
    tags: Option<Vec<String>>,
    primary_genre: Option<String>,
    visibility: Option<UpdateMusicFinetuneRequestModelVisibility>,
}

impl UpdateMusicFinetuneRequestModelBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn primary_genre(mut self, value: impl Into<String>) -> Self {
        self.primary_genre = Some(value.into());
        self
    }

    pub fn visibility(mut self, value: UpdateMusicFinetuneRequestModelVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateMusicFinetuneRequestModel`].
    pub fn build(self) -> Result<UpdateMusicFinetuneRequestModel, BuildError> {
        Ok(UpdateMusicFinetuneRequestModel {
            name: self.name,
            tags: self.tags,
            primary_genre: self.primary_genre,
            visibility: self.visibility,
        })
    }
}

