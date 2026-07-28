pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Alignment information for the generated audio given the input text sequence.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Alignment {
    /// A list of starting times (in milliseconds) for each character in the text as it
    /// corresponds to the audio. For instance, the character 'H' starts at time 0 ms in the audio.
    /// Note these times are relative to the returned chunk from the model, and not the
    /// full audio response.
    #[serde(rename = "charStartTimesMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub char_start_times_ms: Option<Vec<i64>>,
    /// A list of durations (in milliseconds) for each character in the text as it
    /// corresponds to the audio. For instance, the character 'H' lasts for 3 ms in the audio.
    /// Note these times are relative to the returned chunk from the model, and not the
    /// full audio response.
    #[serde(rename = "charDurationsMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub char_durations_ms: Option<Vec<i64>>,
    /// A list of characters in the text sequence. For instance, the first character is 'H'.
    /// Note that this list may contain spaces, punctuation, and other special characters.
    /// The length of this list should be the same as the lengths of `charStartTimesMs` and `charDurationsMs`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chars: Option<Vec<String>>,
}

impl Alignment {
    pub fn builder() -> AlignmentBuilder {
        <AlignmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AlignmentBuilder {
    char_start_times_ms: Option<Vec<i64>>,
    char_durations_ms: Option<Vec<i64>>,
    chars: Option<Vec<String>>,
}

impl AlignmentBuilder {
    pub fn char_start_times_ms(mut self, value: Vec<i64>) -> Self {
        self.char_start_times_ms = Some(value);
        self
    }

    pub fn char_durations_ms(mut self, value: Vec<i64>) -> Self {
        self.char_durations_ms = Some(value);
        self
    }

    pub fn chars(mut self, value: Vec<String>) -> Self {
        self.chars = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Alignment`].
    pub fn build(self) -> Result<Alignment, BuildError> {
        Ok(Alignment {
            char_start_times_ms: self.char_start_times_ms,
            char_durations_ms: self.char_durations_ms,
            chars: self.chars,
        })
    }
}
