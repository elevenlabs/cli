pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A whitelist of fields that can be overridden by users when
/// configuring an API Integration Webhook Tool.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ApiIntegrationWebhookOverrides {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_overrides: Option<HashMap<String, Option<ApiIntegrationWebhookOverridesSchemaOverridesValue>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_headers: Option<HashMap<String, Option<ApiIntegrationWebhookOverridesRequestHeadersValue>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_filter_mode: Option<ResponseFilterMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_filters: Option<Vec<String>>,
}

impl ApiIntegrationWebhookOverrides {
    pub fn builder() -> ApiIntegrationWebhookOverridesBuilder {
        <ApiIntegrationWebhookOverridesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiIntegrationWebhookOverridesBuilder {
    schema_overrides: Option<HashMap<String, Option<ApiIntegrationWebhookOverridesSchemaOverridesValue>>>,
    request_headers: Option<HashMap<String, Option<ApiIntegrationWebhookOverridesRequestHeadersValue>>>,
    response_filter_mode: Option<ResponseFilterMode>,
    response_filters: Option<Vec<String>>,
}

impl ApiIntegrationWebhookOverridesBuilder {
    pub fn schema_overrides(mut self, value: HashMap<String, Option<ApiIntegrationWebhookOverridesSchemaOverridesValue>>) -> Self {
        self.schema_overrides = Some(value);
        self
    }

    pub fn request_headers(mut self, value: HashMap<String, Option<ApiIntegrationWebhookOverridesRequestHeadersValue>>) -> Self {
        self.request_headers = Some(value);
        self
    }

    pub fn response_filter_mode(mut self, value: ResponseFilterMode) -> Self {
        self.response_filter_mode = Some(value);
        self
    }

    pub fn response_filters(mut self, value: Vec<String>) -> Self {
        self.response_filters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApiIntegrationWebhookOverrides`].
    pub fn build(self) -> Result<ApiIntegrationWebhookOverrides, BuildError> {
        Ok(ApiIntegrationWebhookOverrides {
            schema_overrides: self.schema_overrides,
            request_headers: self.request_headers,
            response_filter_mode: self.response_filter_mode,
            response_filters: self.response_filters,
        })
    }
}
