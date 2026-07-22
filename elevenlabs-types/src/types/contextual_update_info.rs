pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ContextualUpdateInfo {
    /// Client-supplied identifier grouping related contextual updates.
    #[serde(default)]
    pub context_id: String,
    /// True when this contextual update has been replaced by a newer update with the same context_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_superseded: Option<bool>,
}

impl ContextualUpdateInfo {
    pub fn builder() -> ContextualUpdateInfoBuilder {
        <ContextualUpdateInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContextualUpdateInfoBuilder {
    context_id: Option<String>,
    is_superseded: Option<bool>,
}

impl ContextualUpdateInfoBuilder {
    pub fn context_id(mut self, value: impl Into<String>) -> Self {
        self.context_id = Some(value.into());
        self
    }

    pub fn is_superseded(mut self, value: bool) -> Self {
        self.is_superseded = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ContextualUpdateInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`context_id`](ContextualUpdateInfoBuilder::context_id)
    pub fn build(self) -> Result<ContextualUpdateInfo, BuildError> {
        Ok(ContextualUpdateInfo {
            context_id: self.context_id.ok_or_else(|| BuildError::missing_field("context_id"))?,
            is_superseded: self.is_superseded,
        })
    }
}
