pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ContextualUpdateInputItem {
    /// Context for the agent that the user did not write, such as the client's current state.
    #[serde(default)]
    pub text: String,
    /// What the update describes. The agent sees only the latest update for each `context_id`.
    #[serde(default)]
    pub context_id: String,
    /// `in_place`: the update stays where it was sent, like an event. `end`: the update is current state, sent after the newest message or tool result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<ContextualUpdatePlacement>,
}

impl ContextualUpdateInputItem {
    pub fn builder() -> ContextualUpdateInputItemBuilder {
        <ContextualUpdateInputItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContextualUpdateInputItemBuilder {
    text: Option<String>,
    context_id: Option<String>,
    placement: Option<ContextualUpdatePlacement>,
}

impl ContextualUpdateInputItemBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn context_id(mut self, value: impl Into<String>) -> Self {
        self.context_id = Some(value.into());
        self
    }

    pub fn placement(mut self, value: ContextualUpdatePlacement) -> Self {
        self.placement = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ContextualUpdateInputItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](ContextualUpdateInputItemBuilder::text)
    /// - [`context_id`](ContextualUpdateInputItemBuilder::context_id)
    pub fn build(self) -> Result<ContextualUpdateInputItem, BuildError> {
        Ok(ContextualUpdateInputItem {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            context_id: self.context_id.ok_or_else(|| BuildError::missing_field("context_id"))?,
            placement: self.placement,
        })
    }
}
