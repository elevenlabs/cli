pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchHighlightSegment {
    #[serde(default)]
    pub value: String,
    #[serde(default)]
    pub is_hit: bool,
}

impl SearchHighlightSegment {
    pub fn builder() -> SearchHighlightSegmentBuilder {
        <SearchHighlightSegmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchHighlightSegmentBuilder {
    value: Option<String>,
    is_hit: Option<bool>,
}

impl SearchHighlightSegmentBuilder {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn is_hit(mut self, value: bool) -> Self {
        self.is_hit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchHighlightSegment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](SearchHighlightSegmentBuilder::value)
    /// - [`is_hit`](SearchHighlightSegmentBuilder::is_hit)
    pub fn build(self) -> Result<SearchHighlightSegment, BuildError> {
        Ok(SearchHighlightSegment {
            value: self.value.ok_or_else(|| BuildError::missing_field("value"))?,
            is_hit: self.is_hit.ok_or_else(|| BuildError::missing_field("is_hit"))?,
        })
    }
}
