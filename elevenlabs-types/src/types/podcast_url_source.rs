pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PodcastUrlSource {
    /// The type of source to create.
    pub r#type: String,
    /// The URL to create the podcast from.
    #[serde(default)]
    pub url: String,
}

impl PodcastUrlSource {
    pub fn builder() -> PodcastUrlSourceBuilder {
        <PodcastUrlSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PodcastUrlSourceBuilder {
    r#type: Option<String>,
    url: Option<String>,
}

impl PodcastUrlSourceBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PodcastUrlSource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](PodcastUrlSourceBuilder::r#type)
    /// - [`url`](PodcastUrlSourceBuilder::url)
    pub fn build(self) -> Result<PodcastUrlSource, BuildError> {
        Ok(PodcastUrlSource {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
