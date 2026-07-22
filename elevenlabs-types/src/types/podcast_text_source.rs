pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PodcastTextSource {
    /// The type of source to create.
    pub r#type: String,
    /// The text to create the podcast from.
    #[serde(default)]
    pub text: String,
}

impl PodcastTextSource {
    pub fn builder() -> PodcastTextSourceBuilder {
        <PodcastTextSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PodcastTextSourceBuilder {
    r#type: Option<String>,
    text: Option<String>,
}

impl PodcastTextSourceBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PodcastTextSource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](PodcastTextSourceBuilder::r#type)
    /// - [`text`](PodcastTextSourceBuilder::text)
    pub fn build(self) -> Result<PodcastTextSource, BuildError> {
        Ok(PodcastTextSource {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
