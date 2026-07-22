pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreatePvcVoiceRequest {
    /// The name that identifies this voice. This will be displayed in the dropdown of the website.
    #[serde(default)]
    pub name: String,
    /// Language used in the samples.
    #[serde(default)]
    pub language: String,
    /// Description to use for the created voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Labels for the voice. Keys can be language, accent, gender, or age.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, Option<String>>>,
}

impl CreatePvcVoiceRequest {
    pub fn builder() -> CreatePvcVoiceRequestBuilder {
        <CreatePvcVoiceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePvcVoiceRequestBuilder {
    name: Option<String>,
    language: Option<String>,
    description: Option<String>,
    labels: Option<HashMap<String, Option<String>>>,
}

impl CreatePvcVoiceRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn labels(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.labels = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePvcVoiceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreatePvcVoiceRequestBuilder::name)
    /// - [`language`](CreatePvcVoiceRequestBuilder::language)
    pub fn build(self) -> Result<CreatePvcVoiceRequest, BuildError> {
        Ok(CreatePvcVoiceRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            language: self.language.ok_or_else(|| BuildError::missing_field("language"))?,
            description: self.description,
            labels: self.labels,
        })
    }
}

