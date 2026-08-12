pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An entity detected within transcribed text.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DetectedEntity {
    /// The text that was identified as an entity.
    #[serde(default)]
    pub text: String,
    /// The type of entity detected (e.g., 'credit_card', 'email_address', 'person_name').
    #[serde(default)]
    pub entity_type: String,
    /// Start character position in the transcript text.
    #[serde(default)]
    pub start_char: i64,
    /// End character position in the transcript text.
    #[serde(default)]
    pub end_char: i64,
}

impl DetectedEntity {
    pub fn builder() -> DetectedEntityBuilder {
        <DetectedEntityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DetectedEntityBuilder {
    text: Option<String>,
    entity_type: Option<String>,
    start_char: Option<i64>,
    end_char: Option<i64>,
}

impl DetectedEntityBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn entity_type(mut self, value: impl Into<String>) -> Self {
        self.entity_type = Some(value.into());
        self
    }

    pub fn start_char(mut self, value: i64) -> Self {
        self.start_char = Some(value);
        self
    }

    pub fn end_char(mut self, value: i64) -> Self {
        self.end_char = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DetectedEntity`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](DetectedEntityBuilder::text)
    /// - [`entity_type`](DetectedEntityBuilder::entity_type)
    /// - [`start_char`](DetectedEntityBuilder::start_char)
    /// - [`end_char`](DetectedEntityBuilder::end_char)
    pub fn build(self) -> Result<DetectedEntity, BuildError> {
        Ok(DetectedEntity {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            entity_type: self.entity_type.ok_or_else(|| BuildError::missing_field("entity_type"))?,
            start_char: self.start_char.ok_or_else(|| BuildError::missing_field("start_char"))?,
            end_char: self.end_char.ok_or_else(|| BuildError::missing_field("end_char"))?,
        })
    }
}
