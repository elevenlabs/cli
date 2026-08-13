pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for delete
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationalAiToolsDeleteQueryRequest {
    /// If set to true, the tool will be deleted regardless of whether it is used by any agents and it will be removed from the dependent agents and branches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

impl ConversationalAiToolsDeleteQueryRequest {
    pub fn builder() -> ConversationalAiToolsDeleteQueryRequestBuilder {
        <ConversationalAiToolsDeleteQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationalAiToolsDeleteQueryRequestBuilder {
    force: Option<bool>,
}

impl ConversationalAiToolsDeleteQueryRequestBuilder {
    pub fn force(mut self, value: bool) -> Self {
        self.force = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationalAiToolsDeleteQueryRequest`].
    pub fn build(self) -> Result<ConversationalAiToolsDeleteQueryRequest, BuildError> {
        Ok(ConversationalAiToolsDeleteQueryRequest {
            force: self.force,
        })
    }
}

