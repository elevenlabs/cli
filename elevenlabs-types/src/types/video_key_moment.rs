pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VideoKeyMoment {
    #[serde(default)]
    pub timestamp_ms: i64,
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub description: String,
}

impl VideoKeyMoment {
    pub fn builder() -> VideoKeyMomentBuilder {
        <VideoKeyMomentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VideoKeyMomentBuilder {
    timestamp_ms: Option<i64>,
    r#type: Option<String>,
    description: Option<String>,
}

impl VideoKeyMomentBuilder {
    pub fn timestamp_ms(mut self, value: i64) -> Self {
        self.timestamp_ms = Some(value);
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VideoKeyMoment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`timestamp_ms`](VideoKeyMomentBuilder::timestamp_ms)
    /// - [`r#type`](VideoKeyMomentBuilder::r#type)
    /// - [`description`](VideoKeyMomentBuilder::description)
    pub fn build(self) -> Result<VideoKeyMoment, BuildError> {
        Ok(VideoKeyMoment {
            timestamp_ms: self.timestamp_ms.ok_or_else(|| BuildError::missing_field("timestamp_ms"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
        })
    }
}
