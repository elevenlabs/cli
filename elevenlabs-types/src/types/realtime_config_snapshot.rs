pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RealtimeConfigSnapshot {
    #[serde(rename = "_id")]
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub workspace_id: String,
    #[serde(default)]
    pub conversation_id: String,
    #[serde(default)]
    pub seq_no: i64,
    #[serde(default)]
    pub time_in_call_secs: i64,
    #[serde(default)]
    pub time_committed_secs: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_event_id: Option<i64>,
    #[serde(default)]
    pub parents: RealtimeConfigSnapshotParents,
    #[serde(default)]
    pub session_config: OpenAiSessionConfig,
    #[serde(default)]
    pub safety: SafetyCommonModelOutput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changed_paths: Option<Vec<String>>,
}

impl RealtimeConfigSnapshot {
    pub fn builder() -> RealtimeConfigSnapshotBuilder {
        <RealtimeConfigSnapshotBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RealtimeConfigSnapshotBuilder {
    id: Option<String>,
    workspace_id: Option<String>,
    conversation_id: Option<String>,
    seq_no: Option<i64>,
    time_in_call_secs: Option<i64>,
    time_committed_secs: Option<i64>,
    source_event_id: Option<i64>,
    parents: Option<RealtimeConfigSnapshotParents>,
    session_config: Option<OpenAiSessionConfig>,
    safety: Option<SafetyCommonModelOutput>,
    changed_paths: Option<Vec<String>>,
}

impl RealtimeConfigSnapshotBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn seq_no(mut self, value: i64) -> Self {
        self.seq_no = Some(value);
        self
    }

    pub fn time_in_call_secs(mut self, value: i64) -> Self {
        self.time_in_call_secs = Some(value);
        self
    }

    pub fn time_committed_secs(mut self, value: i64) -> Self {
        self.time_committed_secs = Some(value);
        self
    }

    pub fn source_event_id(mut self, value: i64) -> Self {
        self.source_event_id = Some(value);
        self
    }

    pub fn parents(mut self, value: RealtimeConfigSnapshotParents) -> Self {
        self.parents = Some(value);
        self
    }

    pub fn session_config(mut self, value: OpenAiSessionConfig) -> Self {
        self.session_config = Some(value);
        self
    }

    pub fn safety(mut self, value: SafetyCommonModelOutput) -> Self {
        self.safety = Some(value);
        self
    }

    pub fn changed_paths(mut self, value: Vec<String>) -> Self {
        self.changed_paths = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RealtimeConfigSnapshot`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RealtimeConfigSnapshotBuilder::id)
    /// - [`workspace_id`](RealtimeConfigSnapshotBuilder::workspace_id)
    /// - [`conversation_id`](RealtimeConfigSnapshotBuilder::conversation_id)
    /// - [`seq_no`](RealtimeConfigSnapshotBuilder::seq_no)
    /// - [`time_in_call_secs`](RealtimeConfigSnapshotBuilder::time_in_call_secs)
    /// - [`time_committed_secs`](RealtimeConfigSnapshotBuilder::time_committed_secs)
    /// - [`parents`](RealtimeConfigSnapshotBuilder::parents)
    /// - [`session_config`](RealtimeConfigSnapshotBuilder::session_config)
    /// - [`safety`](RealtimeConfigSnapshotBuilder::safety)
    pub fn build(self) -> Result<RealtimeConfigSnapshot, BuildError> {
        Ok(RealtimeConfigSnapshot {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            workspace_id: self.workspace_id.ok_or_else(|| BuildError::missing_field("workspace_id"))?,
            conversation_id: self.conversation_id.ok_or_else(|| BuildError::missing_field("conversation_id"))?,
            seq_no: self.seq_no.ok_or_else(|| BuildError::missing_field("seq_no"))?,
            time_in_call_secs: self.time_in_call_secs.ok_or_else(|| BuildError::missing_field("time_in_call_secs"))?,
            time_committed_secs: self.time_committed_secs.ok_or_else(|| BuildError::missing_field("time_committed_secs"))?,
            source_event_id: self.source_event_id,
            parents: self.parents.ok_or_else(|| BuildError::missing_field("parents"))?,
            session_config: self.session_config.ok_or_else(|| BuildError::missing_field("session_config"))?,
            safety: self.safety.ok_or_else(|| BuildError::missing_field("safety"))?,
            changed_paths: self.changed_paths,
        })
    }
}
