pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Information about the source of conversation initiation
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationInitiationSourceInfo {
    /// Source of the conversation initiation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ConversationInitiationSource>,
    /// The SDK version number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl ConversationInitiationSourceInfo {
    pub fn builder() -> ConversationInitiationSourceInfoBuilder {
        <ConversationInitiationSourceInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationInitiationSourceInfoBuilder {
    source: Option<ConversationInitiationSource>,
    version: Option<String>,
}

impl ConversationInitiationSourceInfoBuilder {
    pub fn source(mut self, value: ConversationInitiationSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationInitiationSourceInfo`].
    pub fn build(self) -> Result<ConversationInitiationSourceInfo, BuildError> {
        Ok(ConversationInitiationSourceInfo {
            source: self.source,
            version: self.version,
        })
    }
}
