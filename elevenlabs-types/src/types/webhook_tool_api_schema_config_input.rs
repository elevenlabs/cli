pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WebhookToolApiSchemaConfigInput {
    /// Headers that should be included in the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_headers: Option<HashMap<String, WebhookToolApiSchemaConfigInputRequestHeadersValue>>,
    /// The URL that the webhook will be sent to. May include path parameters, e.g. https://example.com/agents/{agent_id}
    #[serde(default)]
    pub url: String,
    /// The HTTP method to use for the webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<WebhookToolApiSchemaConfigInputMethod>,
    /// Schema for path parameters, if any. The keys should match the placeholders in the URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_params_schema: Option<HashMap<String, LiteralJsonSchemaProperty>>,
    /// Schema for any query params, if any. These will be added to end of the URL as query params. Note: properties in a query param must all be literal types
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_params_schema: Option<QueryParamsJsonSchema>,
    /// Schema for the body parameters, if any. Used for POST/PATCH/PUT requests. The schema should be an object which will be sent as the json body
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body_schema: Option<ObjectJsonSchemaPropertyInput>,
    /// Schema describing the expected response body structure. For documentation only; not surfaced to the LLM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_body_schema: Option<ObjectJsonSchemaPropertyInput>,
    /// Optional allow-list filter applied to the response before the LLM sees it, so large responses don't pollute the context. Defaults to the full response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_filter: Option<ResponseFilter>,
    /// Content type for the request body. Only applies to POST/PUT/PATCH requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<WebhookToolApiSchemaConfigInputContentType>,
    /// URL placeholders resolved from the auth connection (e.g. secrets injected via UrlSecretAuthConnection) rather than from path_params_schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_resolved_params: Option<Vec<String>>,
    /// Optional auth connection to use for authentication with this webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_connection: Option<WebhookToolApiSchemaConfigInputAuthConnection>,
}

impl WebhookToolApiSchemaConfigInput {
    pub fn builder() -> WebhookToolApiSchemaConfigInputBuilder {
        <WebhookToolApiSchemaConfigInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookToolApiSchemaConfigInputBuilder {
    request_headers: Option<HashMap<String, WebhookToolApiSchemaConfigInputRequestHeadersValue>>,
    url: Option<String>,
    method: Option<WebhookToolApiSchemaConfigInputMethod>,
    path_params_schema: Option<HashMap<String, LiteralJsonSchemaProperty>>,
    query_params_schema: Option<QueryParamsJsonSchema>,
    request_body_schema: Option<ObjectJsonSchemaPropertyInput>,
    response_body_schema: Option<ObjectJsonSchemaPropertyInput>,
    response_filter: Option<ResponseFilter>,
    content_type: Option<WebhookToolApiSchemaConfigInputContentType>,
    auth_resolved_params: Option<Vec<String>>,
    auth_connection: Option<WebhookToolApiSchemaConfigInputAuthConnection>,
}

impl WebhookToolApiSchemaConfigInputBuilder {
    pub fn request_headers(mut self, value: HashMap<String, WebhookToolApiSchemaConfigInputRequestHeadersValue>) -> Self {
        self.request_headers = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn method(mut self, value: WebhookToolApiSchemaConfigInputMethod) -> Self {
        self.method = Some(value);
        self
    }

    pub fn path_params_schema(mut self, value: HashMap<String, LiteralJsonSchemaProperty>) -> Self {
        self.path_params_schema = Some(value);
        self
    }

    pub fn query_params_schema(mut self, value: QueryParamsJsonSchema) -> Self {
        self.query_params_schema = Some(value);
        self
    }

    pub fn request_body_schema(mut self, value: ObjectJsonSchemaPropertyInput) -> Self {
        self.request_body_schema = Some(value);
        self
    }

    pub fn response_body_schema(mut self, value: ObjectJsonSchemaPropertyInput) -> Self {
        self.response_body_schema = Some(value);
        self
    }

    pub fn response_filter(mut self, value: ResponseFilter) -> Self {
        self.response_filter = Some(value);
        self
    }

    pub fn content_type(mut self, value: WebhookToolApiSchemaConfigInputContentType) -> Self {
        self.content_type = Some(value);
        self
    }

    pub fn auth_resolved_params(mut self, value: Vec<String>) -> Self {
        self.auth_resolved_params = Some(value);
        self
    }

    pub fn auth_connection(mut self, value: WebhookToolApiSchemaConfigInputAuthConnection) -> Self {
        self.auth_connection = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebhookToolApiSchemaConfigInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](WebhookToolApiSchemaConfigInputBuilder::url)
    pub fn build(self) -> Result<WebhookToolApiSchemaConfigInput, BuildError> {
        Ok(WebhookToolApiSchemaConfigInput {
            request_headers: self.request_headers,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            method: self.method,
            path_params_schema: self.path_params_schema,
            query_params_schema: self.query_params_schema,
            request_body_schema: self.request_body_schema,
            response_body_schema: self.response_body_schema,
            response_filter: self.response_filter,
            content_type: self.content_type,
            auth_resolved_params: self.auth_resolved_params,
            auth_connection: self.auth_connection,
        })
    }
}
