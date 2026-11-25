pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FindSimilarVoicesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes::option")]
    pub audio_file: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub similarity_threshold: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i64>,
    #[serde(skip)]
    pub category: Option<VoicesFindSimilarVoicesRequestCategory>,
    #[serde(skip)]
    pub gender: Option<String>,
    #[serde(skip)]
    pub age: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub use_cases: Vec<Option<String>>,
    #[serde(skip)]
    pub min_notice_period_days: Option<i64>,
    #[serde(skip)]
    pub include_custom_rates: Option<bool>,
    #[serde(skip)]
    pub include_live_moderated: Option<bool>,
    #[serde(skip)]
    pub owner_id: Option<String>,
}
impl FindSimilarVoicesRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
    let mut form = reqwest::multipart::Form::new();

    if let Some(ref file_data) = self.audio_file {
        form = form.part(
            "audio_file",
            reqwest::multipart::Part::bytes(file_data.clone())
                .file_name("audio_file")
                .mime_str("application/octet-stream").unwrap()
        );
    }

    if let Some(ref value) = self.similarity_threshold {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("similarity_threshold", json_str);
        }
    }

    if let Some(ref value) = self.top_k {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("top_k", json_str);
        }
    }

    form
}
}

impl FindSimilarVoicesRequest {
    pub fn builder() -> FindSimilarVoicesRequestBuilder {
        <FindSimilarVoicesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FindSimilarVoicesRequestBuilder {
    audio_file: Option<Vec<u8>>,
    similarity_threshold: Option<f64>,
    top_k: Option<i64>,
    category: Option<VoicesFindSimilarVoicesRequestCategory>,
    gender: Option<String>,
    age: Option<String>,
    use_cases: Option<Vec<Option<String>>>,
    min_notice_period_days: Option<i64>,
    include_custom_rates: Option<bool>,
    include_live_moderated: Option<bool>,
    owner_id: Option<String>,
}

impl FindSimilarVoicesRequestBuilder {
    pub fn audio_file(mut self, value: Vec<u8>) -> Self {
        self.audio_file = Some(value);
        self
    }

    pub fn similarity_threshold(mut self, value: f64) -> Self {
        self.similarity_threshold = Some(value);
        self
    }

    pub fn top_k(mut self, value: i64) -> Self {
        self.top_k = Some(value);
        self
    }

    pub fn category(mut self, value: VoicesFindSimilarVoicesRequestCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn gender(mut self, value: impl Into<String>) -> Self {
        self.gender = Some(value.into());
        self
    }

    pub fn age(mut self, value: impl Into<String>) -> Self {
        self.age = Some(value.into());
        self
    }

    pub fn use_cases(mut self, value: Vec<Option<String>>) -> Self {
        self.use_cases = Some(value);
        self
    }

    pub fn min_notice_period_days(mut self, value: i64) -> Self {
        self.min_notice_period_days = Some(value);
        self
    }

    pub fn include_custom_rates(mut self, value: bool) -> Self {
        self.include_custom_rates = Some(value);
        self
    }

    pub fn include_live_moderated(mut self, value: bool) -> Self {
        self.include_live_moderated = Some(value);
        self
    }

    pub fn owner_id(mut self, value: impl Into<String>) -> Self {
        self.owner_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FindSimilarVoicesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`use_cases`](FindSimilarVoicesRequestBuilder::use_cases)
    pub fn build(self) -> Result<FindSimilarVoicesRequest, BuildError> {
        Ok(FindSimilarVoicesRequest {
            audio_file: self.audio_file,
            similarity_threshold: self.similarity_threshold,
            top_k: self.top_k,
            category: self.category,
            gender: self.gender,
            age: self.age,
            use_cases: self.use_cases.ok_or_else(|| BuildError::missing_field("use_cases"))?,
            min_notice_period_days: self.min_notice_period_days,
            include_custom_rates: self.include_custom_rates,
            include_live_moderated: self.include_live_moderated,
            owner_id: self.owner_id,
        })
    }
}
