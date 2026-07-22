pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DubbingTargetSegmentUpdateRequest {
    /// New translated text, or null to mark the segment for re-translation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<String>,
}

impl DubbingTargetSegmentUpdateRequest {
    pub fn builder() -> DubbingTargetSegmentUpdateRequestBuilder {
        <DubbingTargetSegmentUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingTargetSegmentUpdateRequestBuilder {
    translation: Option<String>,
}

impl DubbingTargetSegmentUpdateRequestBuilder {
    pub fn translation(mut self, value: impl Into<String>) -> Self {
        self.translation = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DubbingTargetSegmentUpdateRequest`].
    pub fn build(self) -> Result<DubbingTargetSegmentUpdateRequest, BuildError> {
        Ok(DubbingTargetSegmentUpdateRequest {
            translation: self.translation,
        })
    }
}

