pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GenerationSourceContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(default)]
    pub generation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(default)]
    pub model_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_iteration_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_parameters: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extend_video: Option<ReferenceVideo>,
}

impl GenerationSourceContext {
    pub fn builder() -> GenerationSourceContextBuilder {
        <GenerationSourceContextBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GenerationSourceContextBuilder {
    source_type: Option<String>,
    generation_id: Option<String>,
    prompt: Option<String>,
    model_id: Option<String>,
    model_provider: Option<String>,
    generation_session_id: Option<String>,
    session_iteration_id: Option<String>,
    model_parameters: Option<HashMap<String, serde_json::Value>>,
    extend_video: Option<ReferenceVideo>,
}

impl GenerationSourceContextBuilder {
    pub fn source_type(mut self, value: impl Into<String>) -> Self {
        self.source_type = Some(value.into());
        self
    }

    pub fn generation_id(mut self, value: impl Into<String>) -> Self {
        self.generation_id = Some(value.into());
        self
    }

    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    pub fn model_id(mut self, value: impl Into<String>) -> Self {
        self.model_id = Some(value.into());
        self
    }

    pub fn model_provider(mut self, value: impl Into<String>) -> Self {
        self.model_provider = Some(value.into());
        self
    }

    pub fn generation_session_id(mut self, value: impl Into<String>) -> Self {
        self.generation_session_id = Some(value.into());
        self
    }

    pub fn session_iteration_id(mut self, value: impl Into<String>) -> Self {
        self.session_iteration_id = Some(value.into());
        self
    }

    pub fn model_parameters(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.model_parameters = Some(value);
        self
    }

    pub fn extend_video(mut self, value: ReferenceVideo) -> Self {
        self.extend_video = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GenerationSourceContext`].
    /// This method will fail if any of the following fields are not set:
    /// - [`generation_id`](GenerationSourceContextBuilder::generation_id)
    /// - [`model_id`](GenerationSourceContextBuilder::model_id)
    pub fn build(self) -> Result<GenerationSourceContext, BuildError> {
        Ok(GenerationSourceContext {
            source_type: self.source_type,
            generation_id: self.generation_id.ok_or_else(|| BuildError::missing_field("generation_id"))?,
            prompt: self.prompt,
            model_id: self.model_id.ok_or_else(|| BuildError::missing_field("model_id"))?,
            model_provider: self.model_provider,
            generation_session_id: self.generation_session_id,
            session_iteration_id: self.session_iteration_id,
            model_parameters: self.model_parameters,
            extend_video: self.extend_video,
        })
    }
}
