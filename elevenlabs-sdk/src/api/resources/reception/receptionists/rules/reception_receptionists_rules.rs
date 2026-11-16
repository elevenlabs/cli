use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct RulesClient2 {
    pub http_client: HttpClient,
}

impl RulesClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Add a behavioral rule to the named receptionist.
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
    ///         .rules
    ///         .create(
    ///             &"agent_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             &ReceptionCreateAgentRuleRequest {
    ///                 title: StrippedStr("title".to_string()),
    ///                 description: StrippedStr("description".to_string()),
    ///                 priority: None,
    ///             },
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        agent_id: &str,
        request: &ReceptionCreateAgentRuleRequest,
        options: Option<RequestOptions>,
    ) -> Result<AgentRule, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/reception/customer-facing-agents/{}/rules", agent_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Replace an existing rule on the named receptionist. Send every field: an omitted `priority` resets to `strict`.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - ID of the receptionist. List them with `GET /v1/reception/customer-facing-agents`.
    /// * `rule_id` - ID of the rule. Returned when you add one, and listed in the receptionist's `config.agent_rules`.
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
    ///         .rules
    ///         .update(
    ///             &"agent_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             &"rule_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             &ReceptionUpdateAgentRuleRequest {
    ///                 title: StrippedStr("title".to_string()),
    ///                 description: StrippedStr("description".to_string()),
    ///                 priority: None,
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
        request: &ReceptionUpdateAgentRuleRequest,
        options: Option<RequestOptions>,
    ) -> Result<AgentRule, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!(
                    "v1/reception/customer-facing-agents/{}/rules/{}",
                    agent_id, rule_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a rule from the named receptionist.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - ID of the receptionist. List them with `GET /v1/reception/customer-facing-agents`.
    /// * `rule_id` - ID of the rule. Returned when you add one, and listed in the receptionist's `config.agent_rules`.
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
    ///         .rules
    ///         .delete(
    ///             &"agent_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             &"rule_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
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
                    "v1/reception/customer-facing-agents/{}/rules/{}",
                    agent_id, rule_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
