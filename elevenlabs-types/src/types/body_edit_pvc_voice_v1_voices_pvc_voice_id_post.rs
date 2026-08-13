pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BodyEditPvcVoiceV1VoicesPvcVoiceIdPost {
    /// The name that identifies this voice. This will be displayed in the dropdown of the website.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Language used in the samples.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Description to use for the created voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Labels for the voice. Keys can be language, accent, gender, or age.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, Option<String>>>,
}

impl BodyEditPvcVoiceV1VoicesPvcVoiceIdPost {
    pub fn builder() -> BodyEditPvcVoiceV1VoicesPvcVoiceIdPostBuilder {
        <BodyEditPvcVoiceV1VoicesPvcVoiceIdPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyEditPvcVoiceV1VoicesPvcVoiceIdPostBuilder {
    name: Option<String>,
    language: Option<String>,
    description: Option<String>,
    labels: Option<HashMap<String, Option<String>>>,
}

impl BodyEditPvcVoiceV1VoicesPvcVoiceIdPostBuilder {
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

    /// Consumes the builder and constructs a [`BodyEditPvcVoiceV1VoicesPvcVoiceIdPost`].
    pub fn build(self) -> Result<BodyEditPvcVoiceV1VoicesPvcVoiceIdPost, BuildError> {
        Ok(BodyEditPvcVoiceV1VoicesPvcVoiceIdPost {
            name: self.name,
            language: self.language,
            description: self.description,
            labels: self.labels,
        })
    }
}

