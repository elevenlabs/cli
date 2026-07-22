pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CustomLlm {
    /// The URL of the Chat Completions compatible endpoint
    #[serde(default)]
    pub url: String,
    /// The model ID to be used if URL serves multiple models
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    /// The API key for authentication. Either a workspace secret reference {'secret_id': '...'} or an environment variable reference {'env_var_label': '...'}.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<CustomLlmApiKey>,
    /// Optional workspace auth connection for authentication. Only auth connections that produce an Authorization Bearer token are supported; Basic auth, mTLS, custom header, and URL secret auth connections are not supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_connection: Option<CustomLlmAuthConnection>,
    /// Headers that should be included in the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_headers: Option<HashMap<String, CustomLlmRequestHeadersValue>>,
    /// The API version to use for the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// The API type to use (chat_completions or responses)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_type: Option<CustomLlmapiType>,
}

impl CustomLlm {
    pub fn builder() -> CustomLlmBuilder {
        <CustomLlmBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomLlmBuilder {
    url: Option<String>,
    model_id: Option<String>,
    api_key: Option<CustomLlmApiKey>,
    auth_connection: Option<CustomLlmAuthConnection>,
    request_headers: Option<HashMap<String, CustomLlmRequestHeadersValue>>,
    api_version: Option<String>,
    api_type: Option<CustomLlmapiType>,
}

impl CustomLlmBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn model_id(mut self, value: impl Into<String>) -> Self {
        self.model_id = Some(value.into());
        self
    }

    pub fn api_key(mut self, value: CustomLlmApiKey) -> Self {
        self.api_key = Some(value);
        self
    }

    pub fn auth_connection(mut self, value: CustomLlmAuthConnection) -> Self {
        self.auth_connection = Some(value);
        self
    }

    pub fn request_headers(mut self, value: HashMap<String, CustomLlmRequestHeadersValue>) -> Self {
        self.request_headers = Some(value);
        self
    }

    pub fn api_version(mut self, value: impl Into<String>) -> Self {
        self.api_version = Some(value.into());
        self
    }

    pub fn api_type(mut self, value: CustomLlmapiType) -> Self {
        self.api_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CustomLlm`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](CustomLlmBuilder::url)
    pub fn build(self) -> Result<CustomLlm, BuildError> {
        Ok(CustomLlm {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            model_id: self.model_id,
            api_key: self.api_key,
            auth_connection: self.auth_connection,
            request_headers: self.request_headers,
            api_version: self.api_version,
            api_type: self.api_type,
        })
    }
}
