pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SuggestedAudioTag {
    /// Audio tag to use (for best performance, 1-2 words, e.g., 'happy', 'excited')
    #[serde(default)]
    pub tag: String,
    /// Optional description of when to use this tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl SuggestedAudioTag {
    pub fn builder() -> SuggestedAudioTagBuilder {
        <SuggestedAudioTagBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SuggestedAudioTagBuilder {
    tag: Option<String>,
    description: Option<String>,
}

impl SuggestedAudioTagBuilder {
    pub fn tag(mut self, value: impl Into<String>) -> Self {
        self.tag = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SuggestedAudioTag`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tag`](SuggestedAudioTagBuilder::tag)
    pub fn build(self) -> Result<SuggestedAudioTag, BuildError> {
        Ok(SuggestedAudioTag {
            tag: self.tag.ok_or_else(|| BuildError::missing_field("tag"))?,
            description: self.description,
        })
    }
}
