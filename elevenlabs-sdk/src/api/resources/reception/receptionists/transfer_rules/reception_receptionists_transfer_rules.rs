use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct TransferRulesClient {
    pub http_client: HttpClient,
}

impl TransferRulesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Add a phone forwarding rule to the named receptionist.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - ID of the receptionist. List them with `GET /v1/reception/customer-facing-agents`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use elevenlabs_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ElevenlabsClient::new(config).expect("Failed to build client");
    ///     client
    ///         .reception
    ///         .receptionists
    ///         .transfer_rules
    ///         .create(
    ///             &"agent_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             &ReceptionCreateTransferRuleRequest {
    ///                 label: StrippedStr("label".to_string()),
    ///                 phone_number: PhoneNumber("phone_number".to_string()),
    ///                 condition: StrippedStr("condition".to_string()),
    ///                 post_dial_digits: None,
    ///             },
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        agent_id: &str,
        request: &ReceptionCreateTransferRuleRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionTransferRule, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/reception/customer-facing-agents/{}/transfer-rules",
                    agent_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Replace an existing transfer rule on the named receptionist. Send every field: an omitted `post_dial_digits` clears it.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - ID of the receptionist. List them with `GET /v1/reception/customer-facing-agents`.
    /// * `rule_id` - ID of the transfer rule. Returned when you add one, and listed in the receptionist's `config.transfer_rules`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use elevenlabs_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ElevenlabsClient::new(config).expect("Failed to build client");
    ///     client
    ///         .reception
    ///         .receptionists
    ///         .transfer_rules
    ///         .update(
    ///             &"agent_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             &"transferrule_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             &ReceptionUpdateTransferRuleRequest {
    ///                 label: StrippedStr("label".to_string()),
    ///                 phone_number: PhoneNumber("phone_number".to_string()),
    ///                 condition: StrippedStr("condition".to_string()),
    ///                 post_dial_digits: None,
    ///             },
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        agent_id: &str,
        rule_id: &str,
        request: &ReceptionUpdateTransferRuleRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionTransferRule, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!(
                    "v1/reception/customer-facing-agents/{}/transfer-rules/{}",
                    agent_id, rule_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a transfer rule from the named receptionist.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - ID of the receptionist. List them with `GET /v1/reception/customer-facing-agents`.
    /// * `rule_id` - ID of the transfer rule. Returned when you add one, and listed in the receptionist's `config.transfer_rules`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use elevenlabs_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ElevenlabsClient::new(config).expect("Failed to build client");
    ///     client
    ///         .reception
    ///         .receptionists
    ///         .transfer_rules
    ///         .delete(
    ///             &"agent_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             &"transferrule_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        agent_id: &str,
        rule_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, String>, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "v1/reception/customer-facing-agents/{}/transfer-rules/{}",
                    agent_id, rule_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
