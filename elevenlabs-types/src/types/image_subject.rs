pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ImageSubject {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
}

impl ImageSubject {
    pub fn builder() -> ImageSubjectBuilder {
        <ImageSubjectBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImageSubjectBuilder {
    name: Option<String>,
    description: Option<String>,
}

impl ImageSubjectBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ImageSubject`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ImageSubjectBuilder::name)
    /// - [`description`](ImageSubjectBuilder::description)
    pub fn build(self) -> Result<ImageSubject, BuildError> {
        Ok(ImageSubject {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
        })
    }
}
