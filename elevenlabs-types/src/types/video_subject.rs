pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VideoSubject {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
}

impl VideoSubject {
    pub fn builder() -> VideoSubjectBuilder {
        <VideoSubjectBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VideoSubjectBuilder {
    name: Option<String>,
    description: Option<String>,
}

impl VideoSubjectBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VideoSubject`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](VideoSubjectBuilder::name)
    /// - [`description`](VideoSubjectBuilder::description)
    pub fn build(self) -> Result<VideoSubject, BuildError> {
        Ok(VideoSubject {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
        })
    }
}
