pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OpenAiSessionConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modalities: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<OpenAiAudioConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<OpenAiSessionConfigToolsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<OpenAiSessionConfigToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_response_output_tokens: Option<OpenAiSessionConfigMaxResponseOutputTokens>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_modalities: Option<Vec<String>>,
}

impl OpenAiSessionConfig {
    pub fn builder() -> OpenAiSessionConfigBuilder {
        <OpenAiSessionConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OpenAiSessionConfigBuilder {
    r#type: Option<String>,
    model: Option<String>,
    instructions: Option<String>,
    modalities: Option<Vec<String>>,
    audio: Option<OpenAiAudioConfig>,
    tools: Option<Vec<OpenAiSessionConfigToolsItem>>,
    tool_choice: Option<OpenAiSessionConfigToolChoice>,
    temperature: Option<f64>,
    max_response_output_tokens: Option<OpenAiSessionConfigMaxResponseOutputTokens>,
    output_modalities: Option<Vec<String>>,
}

impl OpenAiSessionConfigBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn instructions(mut self, value: impl Into<String>) -> Self {
        self.instructions = Some(value.into());
        self
    }

    pub fn modalities(mut self, value: Vec<String>) -> Self {
        self.modalities = Some(value);
        self
    }

    pub fn audio(mut self, value: OpenAiAudioConfig) -> Self {
        self.audio = Some(value);
        self
    }

    pub fn tools(mut self, value: Vec<OpenAiSessionConfigToolsItem>) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn tool_choice(mut self, value: OpenAiSessionConfigToolChoice) -> Self {
        self.tool_choice = Some(value);
        self
    }

    pub fn temperature(mut self, value: f64) -> Self {
        self.temperature = Some(value);
        self
    }

    pub fn max_response_output_tokens(mut self, value: OpenAiSessionConfigMaxResponseOutputTokens) -> Self {
        self.max_response_output_tokens = Some(value);
        self
    }

    pub fn output_modalities(mut self, value: Vec<String>) -> Self {
        self.output_modalities = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OpenAiSessionConfig`].
    pub fn build(self) -> Result<OpenAiSessionConfig, BuildError> {
        Ok(OpenAiSessionConfig {
            r#type: self.r#type,
            model: self.model,
            instructions: self.instructions,
            modalities: self.modalities,
            audio: self.audio,
            tools: self.tools,
            tool_choice: self.tool_choice,
            temperature: self.temperature,
            max_response_output_tokens: self.max_response_output_tokens,
            output_modalities: self.output_modalities,
        })
    }
}
