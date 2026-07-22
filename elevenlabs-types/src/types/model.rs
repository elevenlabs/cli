pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Model {
    /// The unique identifier of the model.
    #[serde(default)]
    pub model_id: String,
    /// The name of the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether the model can be finetuned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_be_finetuned: Option<bool>,
    /// Whether the model can do text-to-speech.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_do_text_to_speech: Option<bool>,
    /// Whether the model can do voice conversion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_do_voice_conversion: Option<bool>,
    /// Whether the model can use style.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_use_style: Option<bool>,
    /// Whether the model can use speaker boost.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_use_speaker_boost: Option<bool>,
    /// Whether the model serves pro voices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serves_pro_voices: Option<bool>,
    /// The cost factor for the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub token_cost_factor: Option<f64>,
    /// The description of the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the model requires alpha access.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_alpha_access: Option<bool>,
    /// The maximum number of characters that can be requested by a free user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_characters_request_free_user: Option<i64>,
    /// The maximum number of characters that can be requested by a subscribed user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_characters_request_subscribed_user: Option<i64>,
    /// The maximum length of text that can be requested for this model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_text_length_per_request: Option<i64>,
    /// The languages supported by the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<LanguageResponse>>,
    /// The rates for the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_rates: Option<ModelRatesResponseModel>,
    /// The concurrency group for the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency_group: Option<String>,
}

impl Model {
    pub fn builder() -> ModelBuilder {
        <ModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ModelBuilder {
    model_id: Option<String>,
    name: Option<String>,
    can_be_finetuned: Option<bool>,
    can_do_text_to_speech: Option<bool>,
    can_do_voice_conversion: Option<bool>,
    can_use_style: Option<bool>,
    can_use_speaker_boost: Option<bool>,
    serves_pro_voices: Option<bool>,
    token_cost_factor: Option<f64>,
    description: Option<String>,
    requires_alpha_access: Option<bool>,
    max_characters_request_free_user: Option<i64>,
    max_characters_request_subscribed_user: Option<i64>,
    maximum_text_length_per_request: Option<i64>,
    languages: Option<Vec<LanguageResponse>>,
    model_rates: Option<ModelRatesResponseModel>,
    concurrency_group: Option<String>,
}

impl ModelBuilder {
    pub fn model_id(mut self, value: impl Into<String>) -> Self {
        self.model_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn can_be_finetuned(mut self, value: bool) -> Self {
        self.can_be_finetuned = Some(value);
        self
    }

    pub fn can_do_text_to_speech(mut self, value: bool) -> Self {
        self.can_do_text_to_speech = Some(value);
        self
    }

    pub fn can_do_voice_conversion(mut self, value: bool) -> Self {
        self.can_do_voice_conversion = Some(value);
        self
    }

    pub fn can_use_style(mut self, value: bool) -> Self {
        self.can_use_style = Some(value);
        self
    }

    pub fn can_use_speaker_boost(mut self, value: bool) -> Self {
        self.can_use_speaker_boost = Some(value);
        self
    }

    pub fn serves_pro_voices(mut self, value: bool) -> Self {
        self.serves_pro_voices = Some(value);
        self
    }

    pub fn token_cost_factor(mut self, value: f64) -> Self {
        self.token_cost_factor = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn requires_alpha_access(mut self, value: bool) -> Self {
        self.requires_alpha_access = Some(value);
        self
    }

    pub fn max_characters_request_free_user(mut self, value: i64) -> Self {
        self.max_characters_request_free_user = Some(value);
        self
    }

    pub fn max_characters_request_subscribed_user(mut self, value: i64) -> Self {
        self.max_characters_request_subscribed_user = Some(value);
        self
    }

    pub fn maximum_text_length_per_request(mut self, value: i64) -> Self {
        self.maximum_text_length_per_request = Some(value);
        self
    }

    pub fn languages(mut self, value: Vec<LanguageResponse>) -> Self {
        self.languages = Some(value);
        self
    }

    pub fn model_rates(mut self, value: ModelRatesResponseModel) -> Self {
        self.model_rates = Some(value);
        self
    }

    pub fn concurrency_group(mut self, value: impl Into<String>) -> Self {
        self.concurrency_group = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Model`].
    /// This method will fail if any of the following fields are not set:
    /// - [`model_id`](ModelBuilder::model_id)
    pub fn build(self) -> Result<Model, BuildError> {
        Ok(Model {
            model_id: self.model_id.ok_or_else(|| BuildError::missing_field("model_id"))?,
            name: self.name,
            can_be_finetuned: self.can_be_finetuned,
            can_do_text_to_speech: self.can_do_text_to_speech,
            can_do_voice_conversion: self.can_do_voice_conversion,
            can_use_style: self.can_use_style,
            can_use_speaker_boost: self.can_use_speaker_boost,
            serves_pro_voices: self.serves_pro_voices,
            token_cost_factor: self.token_cost_factor,
            description: self.description,
            requires_alpha_access: self.requires_alpha_access,
            max_characters_request_free_user: self.max_characters_request_free_user,
            max_characters_request_subscribed_user: self.max_characters_request_subscribed_user,
            maximum_text_length_per_request: self.maximum_text_length_per_request,
            languages: self.languages,
            model_rates: self.model_rates,
            concurrency_group: self.concurrency_group,
        })
    }
}
