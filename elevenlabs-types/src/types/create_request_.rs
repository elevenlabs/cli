pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateRequest4 {
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes")]
    pub asset: Vec<u8>,
    #[serde(default)]
    pub name: String,
}
impl CreateRequest4 {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
    let mut form = reqwest::multipart::Form::new();

    form = form.part(
        "asset",
        reqwest::multipart::Part::bytes(self.asset.clone())
            .file_name("asset")
            .mime_str("application/octet-stream").unwrap()
    );

    if let Ok(json_str) = serde_json::to_string(&self.name) {
        form = form.text("name", json_str);
    }

    form
}
}

impl CreateRequest4 {
    pub fn builder() -> CreateRequest4Builder {
        <CreateRequest4Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateRequest4Builder {
    asset: Option<Vec<u8>>,
    name: Option<String>,
}

impl CreateRequest4Builder {
    pub fn asset(mut self, value: Vec<u8>) -> Self {
        self.asset = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateRequest4`].
    /// This method will fail if any of the following fields are not set:
    /// - [`asset`](CreateRequest4Builder::asset)
    /// - [`name`](CreateRequest4Builder::name)
    pub fn build(self) -> Result<CreateRequest4, BuildError> {
        Ok(CreateRequest4 {
            asset: self.asset.ok_or_else(|| BuildError::missing_field("asset"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
