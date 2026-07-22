pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SpeechEngineSummaryResponse {
    /// The speech engine resource ID
    #[serde(default)]
    pub speech_engine_id: String,
    /// Human-readable name for the speech engine
    #[serde(default)]
    pub name: String,
    /// Creation time in Unix seconds
    #[serde(default)]
    pub created_at_unix_secs: i64,
    /// Arbitrary tags for categorization and filtering
    #[serde(default)]
    pub tags: Vec<String>,
    /// The access information of the speech engine for the user
    pub access_info: ResourceAccessInfo,
}

impl SpeechEngineSummaryResponse {
    pub fn builder() -> SpeechEngineSummaryResponseBuilder {
        <SpeechEngineSummaryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpeechEngineSummaryResponseBuilder {
    speech_engine_id: Option<String>,
    name: Option<String>,
    created_at_unix_secs: Option<i64>,
    tags: Option<Vec<String>>,
    access_info: Option<ResourceAccessInfo>,
}

impl SpeechEngineSummaryResponseBuilder {
    pub fn speech_engine_id(mut self, value: impl Into<String>) -> Self {
        self.speech_engine_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn created_at_unix_secs(mut self, value: i64) -> Self {
        self.created_at_unix_secs = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn access_info(mut self, value: ResourceAccessInfo) -> Self {
        self.access_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SpeechEngineSummaryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`speech_engine_id`](SpeechEngineSummaryResponseBuilder::speech_engine_id)
    /// - [`name`](SpeechEngineSummaryResponseBuilder::name)
    /// - [`created_at_unix_secs`](SpeechEngineSummaryResponseBuilder::created_at_unix_secs)
    /// - [`tags`](SpeechEngineSummaryResponseBuilder::tags)
    /// - [`access_info`](SpeechEngineSummaryResponseBuilder::access_info)
    pub fn build(self) -> Result<SpeechEngineSummaryResponse, BuildError> {
        Ok(SpeechEngineSummaryResponse {
            speech_engine_id: self.speech_engine_id.ok_or_else(|| BuildError::missing_field("speech_engine_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            created_at_unix_secs: self.created_at_unix_secs.ok_or_else(|| BuildError::missing_field("created_at_unix_secs"))?,
            tags: self.tags.ok_or_else(|| BuildError::missing_field("tags"))?,
            access_info: self.access_info.ok_or_else(|| BuildError::missing_field("access_info"))?,
        })
    }
}
