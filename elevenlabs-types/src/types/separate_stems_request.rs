pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SeparateStemsRequest {
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes")]
    pub file: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stem_variation_id: Option<MusicSeparateStemsRequestStemVariationId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_with_c2pa: Option<bool>,
    #[serde(skip)]
    pub output_format: Option<AllowedOutputFormats>,
}
impl SeparateStemsRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
    let mut form = reqwest::multipart::Form::new();

    form = form.part(
        "file",
        reqwest::multipart::Part::bytes(self.file.clone())
            .file_name("file")
            .mime_str("application/octet-stream").unwrap()
    );

    if let Some(ref value) = self.stem_variation_id {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("stem_variation_id", json_str);
        }
    }

    if let Some(ref value) = self.sign_with_c2pa {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("sign_with_c2pa", json_str);
        }
    }

    form
}
}

impl SeparateStemsRequest {
    pub fn builder() -> SeparateStemsRequestBuilder {
        <SeparateStemsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SeparateStemsRequestBuilder {
    file: Option<Vec<u8>>,
    stem_variation_id: Option<MusicSeparateStemsRequestStemVariationId>,
    sign_with_c2pa: Option<bool>,
    output_format: Option<AllowedOutputFormats>,
}

impl SeparateStemsRequestBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn stem_variation_id(mut self, value: MusicSeparateStemsRequestStemVariationId) -> Self {
        self.stem_variation_id = Some(value);
        self
    }

    pub fn sign_with_c2pa(mut self, value: bool) -> Self {
        self.sign_with_c2pa = Some(value);
        self
    }

    pub fn output_format(mut self, value: AllowedOutputFormats) -> Self {
        self.output_format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SeparateStemsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file`](SeparateStemsRequestBuilder::file)
    pub fn build(self) -> Result<SeparateStemsRequest, BuildError> {
        Ok(SeparateStemsRequest {
            file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
            stem_variation_id: self.stem_variation_id,
            sign_with_c2pa: self.sign_with_c2pa,
            output_format: self.output_format,
        })
    }
}
