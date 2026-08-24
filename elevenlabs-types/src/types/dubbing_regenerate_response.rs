pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The accepted re-dub: what it covers and what it cost.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DubbingRegenerateResponse {
    /// The segments this re-dub re-synthesizes: those with edits to apply.
    #[serde(default)]
    pub regenerated_segment_ids: Vec<String>,
    /// Seconds of audio this re-dub covers -- the edited regions only, never the whole target. `charged_seconds` is the part of it that was billed.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub regenerated_seconds: f64,
    /// Seconds actually billed, after the free-regeneration allowance. Zero when the re-dub cost nothing -- the allowance covered all of it, or the project's included generation did.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub charged_seconds: f64,
    /// Free-regeneration seconds left for this language target after this re-dub. The allowance is the source's own duration.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub free_regeneration_seconds_remaining: f64,
}

impl DubbingRegenerateResponse {
    pub fn builder() -> DubbingRegenerateResponseBuilder {
        <DubbingRegenerateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingRegenerateResponseBuilder {
    regenerated_segment_ids: Option<Vec<String>>,
    regenerated_seconds: Option<f64>,
    charged_seconds: Option<f64>,
    free_regeneration_seconds_remaining: Option<f64>,
}

impl DubbingRegenerateResponseBuilder {
    pub fn regenerated_segment_ids(mut self, value: Vec<String>) -> Self {
        self.regenerated_segment_ids = Some(value);
        self
    }

    pub fn regenerated_seconds(mut self, value: f64) -> Self {
        self.regenerated_seconds = Some(value);
        self
    }

    pub fn charged_seconds(mut self, value: f64) -> Self {
        self.charged_seconds = Some(value);
        self
    }

    pub fn free_regeneration_seconds_remaining(mut self, value: f64) -> Self {
        self.free_regeneration_seconds_remaining = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbingRegenerateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`regenerated_segment_ids`](DubbingRegenerateResponseBuilder::regenerated_segment_ids)
    /// - [`regenerated_seconds`](DubbingRegenerateResponseBuilder::regenerated_seconds)
    /// - [`charged_seconds`](DubbingRegenerateResponseBuilder::charged_seconds)
    /// - [`free_regeneration_seconds_remaining`](DubbingRegenerateResponseBuilder::free_regeneration_seconds_remaining)
    pub fn build(self) -> Result<DubbingRegenerateResponse, BuildError> {
        Ok(DubbingRegenerateResponse {
            regenerated_segment_ids: self.regenerated_segment_ids.ok_or_else(|| BuildError::missing_field("regenerated_segment_ids"))?,
            regenerated_seconds: self.regenerated_seconds.ok_or_else(|| BuildError::missing_field("regenerated_seconds"))?,
            charged_seconds: self.charged_seconds.ok_or_else(|| BuildError::missing_field("charged_seconds"))?,
            free_regeneration_seconds_remaining: self.free_regeneration_seconds_remaining.ok_or_else(|| BuildError::missing_field("free_regeneration_seconds_remaining"))?,
        })
    }
}
