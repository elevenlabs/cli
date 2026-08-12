pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudioNativeEditContentResponseModel {
    /// The ID of the project.
    #[serde(default)]
    pub project_id: String,
    /// Whether the project is currently being converted.
    #[serde(default)]
    pub converting: bool,
    /// Whether the project is currently being published.
    #[serde(default)]
    pub publishing: bool,
    /// The HTML snippet to embed the Audio Native player.
    #[serde(default)]
    pub html_snippet: String,
}

impl AudioNativeEditContentResponseModel {
    pub fn builder() -> AudioNativeEditContentResponseModelBuilder {
        <AudioNativeEditContentResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioNativeEditContentResponseModelBuilder {
    project_id: Option<String>,
    converting: Option<bool>,
    publishing: Option<bool>,
    html_snippet: Option<String>,
}

impl AudioNativeEditContentResponseModelBuilder {
    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn converting(mut self, value: bool) -> Self {
        self.converting = Some(value);
        self
    }

    pub fn publishing(mut self, value: bool) -> Self {
        self.publishing = Some(value);
        self
    }

    pub fn html_snippet(mut self, value: impl Into<String>) -> Self {
        self.html_snippet = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AudioNativeEditContentResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`project_id`](AudioNativeEditContentResponseModelBuilder::project_id)
    /// - [`converting`](AudioNativeEditContentResponseModelBuilder::converting)
    /// - [`publishing`](AudioNativeEditContentResponseModelBuilder::publishing)
    /// - [`html_snippet`](AudioNativeEditContentResponseModelBuilder::html_snippet)
    pub fn build(self) -> Result<AudioNativeEditContentResponseModel, BuildError> {
        Ok(AudioNativeEditContentResponseModel {
            project_id: self.project_id.ok_or_else(|| BuildError::missing_field("project_id"))?,
            converting: self.converting.ok_or_else(|| BuildError::missing_field("converting"))?,
            publishing: self.publishing.ok_or_else(|| BuildError::missing_field("publishing"))?,
            html_snippet: self.html_snippet.ok_or_else(|| BuildError::missing_field("html_snippet"))?,
        })
    }
}
