pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    pub conversation_id: String,
}

impl ConversationSource {
    pub fn builder() -> ConversationSourceBuilder {
        <ConversationSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationSourceBuilder {
    r#type: Option<String>,
    conversation_id: Option<String>,
}

impl ConversationSourceBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationSource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_id`](ConversationSourceBuilder::conversation_id)
    pub fn build(self) -> Result<ConversationSource, BuildError> {
        Ok(ConversationSource {
            r#type: self.r#type,
            conversation_id: self.conversation_id.ok_or_else(|| BuildError::missing_field("conversation_id"))?,
        })
    }
}
