pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OpenAiSessionConfigOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<RealtimeModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_modalities: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<OpenAiAudioConfigOutput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<OpenAiSessionConfigOutputToolsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<OpenAiSessionConfigOutputToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_response_output_tokens: Option<OpenAiSessionConfigOutputMaxResponseOutputTokens>,
    /// Maximum wall-clock duration of the session in seconds. The session is closed with code 1000 once it is exceeded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_duration_seconds: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing: Option<OpenAiSessionConfigOutputTracing>,
}

impl OpenAiSessionConfigOutput {
    pub fn builder() -> OpenAiSessionConfigOutputBuilder {
        <OpenAiSessionConfigOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OpenAiSessionConfigOutputBuilder {
    r#type: Option<String>,
    model: Option<RealtimeModel>,
    instructions: Option<String>,
    output_modalities: Option<Vec<String>>,
    audio: Option<OpenAiAudioConfigOutput>,
    tools: Option<Vec<OpenAiSessionConfigOutputToolsItem>>,
    tool_choice: Option<OpenAiSessionConfigOutputToolChoice>,
    temperature: Option<f64>,
    max_response_output_tokens: Option<OpenAiSessionConfigOutputMaxResponseOutputTokens>,
    max_duration_seconds: Option<i64>,
    tracing: Option<OpenAiSessionConfigOutputTracing>,
}

impl OpenAiSessionConfigOutputBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn model(mut self, value: RealtimeModel) -> Self {
        self.model = Some(value);
        self
    }

    pub fn instructions(mut self, value: impl Into<String>) -> Self {
        self.instructions = Some(value.into());
        self
    }

    pub fn output_modalities(mut self, value: Vec<String>) -> Self {
        self.output_modalities = Some(value);
        self
    }

    pub fn audio(mut self, value: OpenAiAudioConfigOutput) -> Self {
        self.audio = Some(value);
        self
    }

    pub fn tools(mut self, value: Vec<OpenAiSessionConfigOutputToolsItem>) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn tool_choice(mut self, value: OpenAiSessionConfigOutputToolChoice) -> Self {
        self.tool_choice = Some(value);
        self
    }

    pub fn temperature(mut self, value: f64) -> Self {
        self.temperature = Some(value);
        self
    }

    pub fn max_response_output_tokens(mut self, value: OpenAiSessionConfigOutputMaxResponseOutputTokens) -> Self {
        self.max_response_output_tokens = Some(value);
        self
    }

    pub fn max_duration_seconds(mut self, value: i64) -> Self {
        self.max_duration_seconds = Some(value);
        self
    }

    pub fn tracing(mut self, value: OpenAiSessionConfigOutputTracing) -> Self {
        self.tracing = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OpenAiSessionConfigOutput`].
    pub fn build(self) -> Result<OpenAiSessionConfigOutput, BuildError> {
        Ok(OpenAiSessionConfigOutput {
            r#type: self.r#type,
            model: self.model,
            instructions: self.instructions,
            output_modalities: self.output_modalities,
            audio: self.audio,
            tools: self.tools,
            tool_choice: self.tool_choice,
            temperature: self.temperature,
            max_response_output_tokens: self.max_response_output_tokens,
            max_duration_seconds: self.max_duration_seconds,
            tracing: self.tracing,
        })
    }
}
