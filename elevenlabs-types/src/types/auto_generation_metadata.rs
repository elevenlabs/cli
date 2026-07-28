pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Metadata for an automatically generated simulation test.
/// 
/// Tracks the origin and context of tests that were auto-generated from
/// conversations, including the source conversation, topic, and evaluation
/// criteria the test covers.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutoGenerationMetadata {
    /// Agent whose simulation library this test belongs to.
    #[serde(default)]
    pub agent_id: String,
    /// Stable topic id (from topic discovery) this test covers.
    #[serde(default)]
    pub stable_topic_id: String,
    /// Evaluation criterion id this test targets.
    #[serde(default)]
    pub criteria_id: String,
    /// Human-readable topic label at generation time.
    #[serde(default)]
    pub topic_label: String,
    /// Conversation the test was generated from.
    #[serde(default)]
    pub source_conversation_id: String,
    /// Unix timestamp (seconds) when the test was generated.
    #[serde(default)]
    pub generated_at_unix: i64,
}

impl AutoGenerationMetadata {
    pub fn builder() -> AutoGenerationMetadataBuilder {
        <AutoGenerationMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutoGenerationMetadataBuilder {
    agent_id: Option<String>,
    stable_topic_id: Option<String>,
    criteria_id: Option<String>,
    topic_label: Option<String>,
    source_conversation_id: Option<String>,
    generated_at_unix: Option<i64>,
}

impl AutoGenerationMetadataBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn stable_topic_id(mut self, value: impl Into<String>) -> Self {
        self.stable_topic_id = Some(value.into());
        self
    }

    pub fn criteria_id(mut self, value: impl Into<String>) -> Self {
        self.criteria_id = Some(value.into());
        self
    }

    pub fn topic_label(mut self, value: impl Into<String>) -> Self {
        self.topic_label = Some(value.into());
        self
    }

    pub fn source_conversation_id(mut self, value: impl Into<String>) -> Self {
        self.source_conversation_id = Some(value.into());
        self
    }

    pub fn generated_at_unix(mut self, value: i64) -> Self {
        self.generated_at_unix = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutoGenerationMetadata`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](AutoGenerationMetadataBuilder::agent_id)
    /// - [`stable_topic_id`](AutoGenerationMetadataBuilder::stable_topic_id)
    /// - [`criteria_id`](AutoGenerationMetadataBuilder::criteria_id)
    /// - [`topic_label`](AutoGenerationMetadataBuilder::topic_label)
    /// - [`source_conversation_id`](AutoGenerationMetadataBuilder::source_conversation_id)
    /// - [`generated_at_unix`](AutoGenerationMetadataBuilder::generated_at_unix)
    pub fn build(self) -> Result<AutoGenerationMetadata, BuildError> {
        Ok(AutoGenerationMetadata {
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            stable_topic_id: self.stable_topic_id.ok_or_else(|| BuildError::missing_field("stable_topic_id"))?,
            criteria_id: self.criteria_id.ok_or_else(|| BuildError::missing_field("criteria_id"))?,
            topic_label: self.topic_label.ok_or_else(|| BuildError::missing_field("topic_label"))?,
            source_conversation_id: self.source_conversation_id.ok_or_else(|| BuildError::missing_field("source_conversation_id"))?,
            generated_at_unix: self.generated_at_unix.ok_or_else(|| BuildError::missing_field("generated_at_unix"))?,
        })
    }
}
