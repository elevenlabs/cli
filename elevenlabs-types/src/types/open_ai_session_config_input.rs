pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OpenAiSessionConfigInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<RealtimeModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_modalities: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<OpenAiAudioConfigInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<OpenAiSessionConfigInputToolsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<OpenAiSessionConfigInputToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_response_output_tokens: Option<OpenAiSessionConfigInputMaxResponseOutputTokens>,
    /// Maximum wall-clock duration of the session in seconds. The session is closed with code 1000 once it is exceeded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_duration_seconds: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_detection: Option<OpenAiSessionConfigInputTurnDetection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing: Option<OpenAiSessionConfigInputTracing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<OpenAiSessionConfigInputTruncation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<HashMap<String, serde_json::Value>>,
}

impl OpenAiSessionConfigInput {
    pub fn builder() -> OpenAiSessionConfigInputBuilder {
        <OpenAiSessionConfigInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OpenAiSessionConfigInputBuilder {
    r#type: Option<String>,
    model: Option<RealtimeModel>,
    instructions: Option<String>,
    output_modalities: Option<Vec<String>>,
    audio: Option<OpenAiAudioConfigInput>,
    tools: Option<Vec<OpenAiSessionConfigInputToolsItem>>,
    tool_choice: Option<OpenAiSessionConfigInputToolChoice>,
    temperature: Option<f64>,
    max_response_output_tokens: Option<OpenAiSessionConfigInputMaxResponseOutputTokens>,
    max_duration_seconds: Option<i64>,
    turn_detection: Option<OpenAiSessionConfigInputTurnDetection>,
    tracing: Option<OpenAiSessionConfigInputTracing>,
    include: Option<Vec<String>>,
    parallel_tool_calls: Option<bool>,
    reasoning: Option<HashMap<String, serde_json::Value>>,
    truncation: Option<OpenAiSessionConfigInputTruncation>,
    prompt: Option<HashMap<String, serde_json::Value>>,
}

impl OpenAiSessionConfigInputBuilder {
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

    pub fn audio(mut self, value: OpenAiAudioConfigInput) -> Self {
        self.audio = Some(value);
        self
    }

    pub fn tools(mut self, value: Vec<OpenAiSessionConfigInputToolsItem>) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn tool_choice(mut self, value: OpenAiSessionConfigInputToolChoice) -> Self {
        self.tool_choice = Some(value);
        self
    }

    pub fn temperature(mut self, value: f64) -> Self {
        self.temperature = Some(value);
        self
    }

    pub fn max_response_output_tokens(mut self, value: OpenAiSessionConfigInputMaxResponseOutputTokens) -> Self {
        self.max_response_output_tokens = Some(value);
        self
    }

    pub fn max_duration_seconds(mut self, value: i64) -> Self {
        self.max_duration_seconds = Some(value);
        self
    }

    pub fn turn_detection(mut self, value: OpenAiSessionConfigInputTurnDetection) -> Self {
        self.turn_detection = Some(value);
        self
    }

    pub fn tracing(mut self, value: OpenAiSessionConfigInputTracing) -> Self {
        self.tracing = Some(value);
        self
    }

    pub fn include(mut self, value: Vec<String>) -> Self {
        self.include = Some(value);
        self
    }

    pub fn parallel_tool_calls(mut self, value: bool) -> Self {
        self.parallel_tool_calls = Some(value);
        self
    }

    pub fn reasoning(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.reasoning = Some(value);
        self
    }

    pub fn truncation(mut self, value: OpenAiSessionConfigInputTruncation) -> Self {
        self.truncation = Some(value);
        self
    }

    pub fn prompt(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.prompt = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OpenAiSessionConfigInput`].
    pub fn build(self) -> Result<OpenAiSessionConfigInput, BuildError> {
        Ok(OpenAiSessionConfigInput {
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
            turn_detection: self.turn_detection,
            tracing: self.tracing,
            include: self.include,
            parallel_tool_calls: self.parallel_tool_calls,
            reasoning: self.reasoning,
            truncation: self.truncation,
            prompt: self.prompt,
        })
    }
}
