pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
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

    /// Consumes the builder and constructs a [`FindSimilarVoicesRequest`].
    pub fn build(self) -> Result<FindSimilarVoicesRequest, BuildError> {
        Ok(FindSimilarVoicesRequest {
            audio_file: self.audio_file,
            similarity_threshold: self.similarity_threshold,
            top_k: self.top_k,
        })
    }
}
