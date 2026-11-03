pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AnalysisItemAttachmentResultData {
    #[serde(default)]
    pub agent_id: String,
    #[serde(default)]
    pub branch_id: String,
    pub action: AnalysisItemAttachmentAction,
    #[serde(default)]
    pub changed: bool,
}

impl AnalysisItemAttachmentResultData {
    pub fn builder() -> AnalysisItemAttachmentResultDataBuilder {
        <AnalysisItemAttachmentResultDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnalysisItemAttachmentResultDataBuilder {
    agent_id: Option<String>,
    branch_id: Option<String>,
    action: Option<AnalysisItemAttachmentAction>,
    changed: Option<bool>,
}

impl AnalysisItemAttachmentResultDataBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    pub fn action(mut self, value: AnalysisItemAttachmentAction) -> Self {
        self.action = Some(value);
        self
    }

    pub fn changed(mut self, value: bool) -> Self {
        self.changed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AnalysisItemAttachmentResultData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](AnalysisItemAttachmentResultDataBuilder::agent_id)
    /// - [`branch_id`](AnalysisItemAttachmentResultDataBuilder::branch_id)
    /// - [`action`](AnalysisItemAttachmentResultDataBuilder::action)
    /// - [`changed`](AnalysisItemAttachmentResultDataBuilder::changed)
    pub fn build(self) -> Result<AnalysisItemAttachmentResultData, BuildError> {
        Ok(AnalysisItemAttachmentResultData {
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            branch_id: self.branch_id.ok_or_else(|| BuildError::missing_field("branch_id"))?,
            action: self.action.ok_or_else(|| BuildError::missing_field("action"))?,
            changed: self.changed.ok_or_else(|| BuildError::missing_field("changed"))?,
        })
    }
}
