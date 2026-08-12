pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SendText {
    /// The text to be sent to the API for audio generation. Should always end with a single space string.
    #[serde(default)]
    pub text: String,
    /// This is an advanced setting that most users shouldn't need to use. It relates to our generation schedule.
    ///
    /// Use this to attempt to immediately trigger the generation of audio, overriding the `chunk_length_schedule`.
    /// Unlike flush, `try_trigger_generation` will only generate audio if our
    /// buffer contains more than a minimum
    /// threshold of characters, this is to ensure a higher quality response from our model.
    ///
    /// Note that overriding the chunk schedule to generate small amounts of
    /// text may result in lower quality audio, therefore, only use this parameter if you
    /// really need text to be processed immediately. We generally recommend keeping the default value of
    /// `false` and adjusting the `chunk_length_schedule` in the `generation_config` instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub try_trigger_generation: Option<bool>,
    /// The voice settings field can be provided in the first `InitializeConnection` message and then must either be not provided or not changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<RealtimeVoiceSettings>,
    /// The generator config field can be provided in the first `InitializeConnection` message and then must either be not provided or not changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generator_config: Option<GenerationConfig>,
    /// Flush forces the generation of audio. Set this value to true when you have finished sending text, but want to keep the websocket connection open.
    ///
    /// This is useful when you want to ensure that the last chunk of audio is generated even when the length of text sent is smaller than the value set in chunk_length_schedule (e.g. 120 or 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flush: Option<bool>,
}

impl SendText {
    pub fn builder() -> SendTextBuilder {
        <SendTextBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendTextBuilder {
    text: Option<String>,
    try_trigger_generation: Option<bool>,
    voice_settings: Option<RealtimeVoiceSettings>,
    generator_config: Option<GenerationConfig>,
    flush: Option<bool>,
}

impl SendTextBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn try_trigger_generation(mut self, value: bool) -> Self {
        self.try_trigger_generation = Some(value);
        self
    }

    pub fn voice_settings(mut self, value: RealtimeVoiceSettings) -> Self {
        self.voice_settings = Some(value);
        self
    }

    pub fn generator_config(mut self, value: GenerationConfig) -> Self {
        self.generator_config = Some(value);
        self
    }

    pub fn flush(mut self, value: bool) -> Self {
        self.flush = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SendText`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](SendTextBuilder::text)
    pub fn build(self) -> Result<SendText, BuildError> {
        Ok(SendText {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            try_trigger_generation: self.try_trigger_generation,
            voice_settings: self.voice_settings,
            generator_config: self.generator_config,
            flush: self.flush,
        })
    }
}
