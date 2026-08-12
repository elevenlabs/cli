pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CueOptionsRequest {
    /// Minimum duration of each cue in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_duration_ms: Option<i64>,
    /// Maximum duration of each cue in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_duration_ms: Option<i64>,
    /// Maximum number of lines per cue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_lines_per_cue: Option<i64>,
    /// Maximum number of characters per line in a cue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_chars_per_line: Option<i64>,
    /// Maximum characters per second reading speed. If not set, no reading speed limit is applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_chars_per_s: Option<i64>,
    /// Minimum gap between consecutive cues in frames. If not set, no minimum gap is enforced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_gap_between_cues_frames: Option<i64>,
}

impl CueOptionsRequest {
    pub fn builder() -> CueOptionsRequestBuilder {
        <CueOptionsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CueOptionsRequestBuilder {
    min_duration_ms: Option<i64>,
    max_duration_ms: Option<i64>,
    max_lines_per_cue: Option<i64>,
    max_chars_per_line: Option<i64>,
    max_chars_per_s: Option<i64>,
    min_gap_between_cues_frames: Option<i64>,
}

impl CueOptionsRequestBuilder {
    pub fn min_duration_ms(mut self, value: i64) -> Self {
        self.min_duration_ms = Some(value);
        self
    }

    pub fn max_duration_ms(mut self, value: i64) -> Self {
        self.max_duration_ms = Some(value);
        self
    }

    pub fn max_lines_per_cue(mut self, value: i64) -> Self {
        self.max_lines_per_cue = Some(value);
        self
    }

    pub fn max_chars_per_line(mut self, value: i64) -> Self {
        self.max_chars_per_line = Some(value);
        self
    }

    pub fn max_chars_per_s(mut self, value: i64) -> Self {
        self.max_chars_per_s = Some(value);
        self
    }

    pub fn min_gap_between_cues_frames(mut self, value: i64) -> Self {
        self.min_gap_between_cues_frames = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CueOptionsRequest`].
    pub fn build(self) -> Result<CueOptionsRequest, BuildError> {
        Ok(CueOptionsRequest {
            min_duration_ms: self.min_duration_ms,
            max_duration_ms: self.max_duration_ms,
            max_lines_per_cue: self.max_lines_per_cue,
            max_chars_per_line: self.max_chars_per_line,
            max_chars_per_s: self.max_chars_per_s,
            min_gap_between_cues_frames: self.min_gap_between_cues_frames,
        })
    }
}
