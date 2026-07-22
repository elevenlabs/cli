pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudioNativeCreateProjectResponseModel {
    /// The ID of the created Audio Native project.
    #[serde(default)]
    pub project_id: String,
    /// Whether the project is currently being converted.
    #[serde(default)]
    pub converting: bool,
    /// The HTML snippet to embed the Audio Native player.
    #[serde(default)]
    pub html_snippet: String,
}

impl AudioNativeCreateProjectResponseModel {
    pub fn builder() -> AudioNativeCreateProjectResponseModelBuilder {
        <AudioNativeCreateProjectResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioNativeCreateProjectResponseModelBuilder {
    project_id: Option<String>,
    converting: Option<bool>,
    html_snippet: Option<String>,
}

impl AudioNativeCreateProjectResponseModelBuilder {
    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn converting(mut self, value: bool) -> Self {
        self.converting = Some(value);
        self
    }

    pub fn html_snippet(mut self, value: impl Into<String>) -> Self {
        self.html_snippet = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AudioNativeCreateProjectResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`project_id`](AudioNativeCreateProjectResponseModelBuilder::project_id)
    /// - [`converting`](AudioNativeCreateProjectResponseModelBuilder::converting)
    /// - [`html_snippet`](AudioNativeCreateProjectResponseModelBuilder::html_snippet)
    pub fn build(self) -> Result<AudioNativeCreateProjectResponseModel, BuildError> {
        Ok(AudioNativeCreateProjectResponseModel {
            project_id: self.project_id.ok_or_else(|| BuildError::missing_field("project_id"))?,
            converting: self.converting.ok_or_else(|| BuildError::missing_field("converting"))?,
            html_snippet: self.html_snippet.ok_or_else(|| BuildError::missing_field("html_snippet"))?,
        })
    }
}
