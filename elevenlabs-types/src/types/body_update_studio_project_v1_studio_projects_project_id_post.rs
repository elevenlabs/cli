pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyUpdateStudioProjectV1StudioProjectsProjectIdPost {
    /// The name of the Studio project, used for identification only.
    #[serde(default)]
    pub name: String,
    /// The voice_id that corresponds to the default voice used for new titles.
    #[serde(default)]
    pub default_title_voice_id: String,
    /// The voice_id that corresponds to the default voice used for new paragraphs.
    #[serde(default)]
    pub default_paragraph_voice_id: String,
    /// An optional name of the author of the Studio project, this will be added as metadata to the mp3 file on Studio project or chapter download.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// An optional name of the author of the Studio project, this will be added as metadata to the mp3 file on Studio project or chapter download.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// An optional ISBN number of the Studio project you want to create, this will be added as metadata to the mp3 file on Studio project or chapter download.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isbn_number: Option<String>,
    /// When the Studio project is downloaded, should the returned audio have postprocessing in order to make it compliant with audiobook normalized volume requirements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_normalization: Option<bool>,
}

impl BodyUpdateStudioProjectV1StudioProjectsProjectIdPost {
    pub fn builder() -> BodyUpdateStudioProjectV1StudioProjectsProjectIdPostBuilder {
        <BodyUpdateStudioProjectV1StudioProjectsProjectIdPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyUpdateStudioProjectV1StudioProjectsProjectIdPostBuilder {
    name: Option<String>,
    default_title_voice_id: Option<String>,
    default_paragraph_voice_id: Option<String>,
    title: Option<String>,
    author: Option<String>,
    isbn_number: Option<String>,
    volume_normalization: Option<bool>,
}

impl BodyUpdateStudioProjectV1StudioProjectsProjectIdPostBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn default_title_voice_id(mut self, value: impl Into<String>) -> Self {
        self.default_title_voice_id = Some(value.into());
        self
    }

    pub fn default_paragraph_voice_id(mut self, value: impl Into<String>) -> Self {
        self.default_paragraph_voice_id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn author(mut self, value: impl Into<String>) -> Self {
        self.author = Some(value.into());
        self
    }

    pub fn isbn_number(mut self, value: impl Into<String>) -> Self {
        self.isbn_number = Some(value.into());
        self
    }

    pub fn volume_normalization(mut self, value: bool) -> Self {
        self.volume_normalization = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyUpdateStudioProjectV1StudioProjectsProjectIdPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](BodyUpdateStudioProjectV1StudioProjectsProjectIdPostBuilder::name)
    /// - [`default_title_voice_id`](BodyUpdateStudioProjectV1StudioProjectsProjectIdPostBuilder::default_title_voice_id)
    /// - [`default_paragraph_voice_id`](BodyUpdateStudioProjectV1StudioProjectsProjectIdPostBuilder::default_paragraph_voice_id)
    pub fn build(self) -> Result<BodyUpdateStudioProjectV1StudioProjectsProjectIdPost, BuildError> {
        Ok(BodyUpdateStudioProjectV1StudioProjectsProjectIdPost {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            default_title_voice_id: self.default_title_voice_id.ok_or_else(|| BuildError::missing_field("default_title_voice_id"))?,
            default_paragraph_voice_id: self.default_paragraph_voice_id.ok_or_else(|| BuildError::missing_field("default_paragraph_voice_id"))?,
            title: self.title,
            author: self.author,
            isbn_number: self.isbn_number,
            volume_normalization: self.volume_normalization,
        })
    }
}

