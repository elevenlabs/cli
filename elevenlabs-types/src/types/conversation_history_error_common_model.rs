pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationHistoryErrorCommonModel {
    #[serde(default)]
    pub code: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl ConversationHistoryErrorCommonModel {
    pub fn builder() -> ConversationHistoryErrorCommonModelBuilder {
        <ConversationHistoryErrorCommonModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationHistoryErrorCommonModelBuilder {
    code: Option<i64>,
    reason: Option<String>,
}

impl ConversationHistoryErrorCommonModelBuilder {
    pub fn code(mut self, value: i64) -> Self {
        self.code = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationHistoryErrorCommonModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](ConversationHistoryErrorCommonModelBuilder::code)
    pub fn build(self) -> Result<ConversationHistoryErrorCommonModel, BuildError> {
        Ok(ConversationHistoryErrorCommonModel {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            reason: self.reason,
        })
    }
}
