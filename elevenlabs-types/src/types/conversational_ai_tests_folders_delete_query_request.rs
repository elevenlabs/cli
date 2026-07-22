pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for delete
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationalAiTestsFoldersDeleteQueryRequest {
    /// Force delete. Required for deleting non-empty folders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

impl ConversationalAiTestsFoldersDeleteQueryRequest {
    pub fn builder() -> ConversationalAiTestsFoldersDeleteQueryRequestBuilder {
        <ConversationalAiTestsFoldersDeleteQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationalAiTestsFoldersDeleteQueryRequestBuilder {
    force: Option<bool>,
}

impl ConversationalAiTestsFoldersDeleteQueryRequestBuilder {
    pub fn force(mut self, value: bool) -> Self {
        self.force = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationalAiTestsFoldersDeleteQueryRequest`].
    pub fn build(self) -> Result<ConversationalAiTestsFoldersDeleteQueryRequest, BuildError> {
        Ok(ConversationalAiTestsFoldersDeleteQueryRequest {
            force: self.force,
        })
    }
}

