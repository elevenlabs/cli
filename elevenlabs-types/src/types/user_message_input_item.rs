pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UserMessageInputItem {
    /// The parts of the message: at most one text part and up to 5 file parts.
    #[serde(default)]
    pub content: Vec<UserMessageInputItemContentItem>,
}

impl UserMessageInputItem {
    pub fn builder() -> UserMessageInputItemBuilder {
        <UserMessageInputItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserMessageInputItemBuilder {
    content: Option<Vec<UserMessageInputItemContentItem>>,
}

impl UserMessageInputItemBuilder {
    pub fn content(mut self, value: Vec<UserMessageInputItemContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UserMessageInputItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](UserMessageInputItemBuilder::content)
    pub fn build(self) -> Result<UserMessageInputItem, BuildError> {
        Ok(UserMessageInputItem {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
        })
    }
}
