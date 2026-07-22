use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct SipTrunkClient {
    pub http_client: HttpClient,
}

impl SipTrunkClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Handle an outbound call via SIP trunk
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn outbound_call(
        &self,
        request: &BodyHandleAnOutboundCallViaSipTrunkV1ConvaiSipTrunkOutboundCallPost,
        options: Option<RequestOptions>,
    ) -> Result<SipTrunkOutboundCallResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/convai/sip-trunk/outbound-call",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
