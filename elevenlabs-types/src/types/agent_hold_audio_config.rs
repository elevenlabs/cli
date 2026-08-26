pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Custom hold audio played on loop to callers waiting in the queue.
/// 
/// Server-written only: set via the agent hold-audio upload route and stripped
/// from user PATCH payloads, since the runtime fetches audio_url at call time.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentHoldAudioConfig {
    /// Bucket-relative path of the uploaded clip, used for deletion
    #[serde(default)]
    pub audio_path: String,
    /// Public CDN URL of the uploaded clip
    #[serde(default)]
    pub audio_url: String,
    /// Filename of the uploaded clip as provided by the user
    #[serde(default)]
    pub original_filename: String,
    /// Duration of the uploaded clip in seconds
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub duration_secs: f64,
    /// Size of the uploaded clip in bytes
    #[serde(default)]
    pub size_bytes: i64,
}

impl AgentHoldAudioConfig {
    pub fn builder() -> AgentHoldAudioConfigBuilder {
        <AgentHoldAudioConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentHoldAudioConfigBuilder {
    audio_path: Option<String>,
    audio_url: Option<String>,
    original_filename: Option<String>,
    duration_secs: Option<f64>,
    size_bytes: Option<i64>,
}

impl AgentHoldAudioConfigBuilder {
    pub fn audio_path(mut self, value: impl Into<String>) -> Self {
        self.audio_path = Some(value.into());
        self
    }

    pub fn audio_url(mut self, value: impl Into<String>) -> Self {
        self.audio_url = Some(value.into());
        self
    }

    pub fn original_filename(mut self, value: impl Into<String>) -> Self {
        self.original_filename = Some(value.into());
        self
    }

    pub fn duration_secs(mut self, value: f64) -> Self {
        self.duration_secs = Some(value);
        self
    }

    pub fn size_bytes(mut self, value: i64) -> Self {
        self.size_bytes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentHoldAudioConfig`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audio_path`](AgentHoldAudioConfigBuilder::audio_path)
    /// - [`audio_url`](AgentHoldAudioConfigBuilder::audio_url)
    /// - [`original_filename`](AgentHoldAudioConfigBuilder::original_filename)
    /// - [`duration_secs`](AgentHoldAudioConfigBuilder::duration_secs)
    /// - [`size_bytes`](AgentHoldAudioConfigBuilder::size_bytes)
    pub fn build(self) -> Result<AgentHoldAudioConfig, BuildError> {
        Ok(AgentHoldAudioConfig {
            audio_path: self.audio_path.ok_or_else(|| BuildError::missing_field("audio_path"))?,
            audio_url: self.audio_url.ok_or_else(|| BuildError::missing_field("audio_url"))?,
            original_filename: self.original_filename.ok_or_else(|| BuildError::missing_field("original_filename"))?,
            duration_secs: self.duration_secs.ok_or_else(|| BuildError::missing_field("duration_secs"))?,
            size_bytes: self.size_bytes.ok_or_else(|| BuildError::missing_field("size_bytes"))?,
        })
    }
}
