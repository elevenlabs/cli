use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct DraftsClient {
    pub http_client: HttpClient,
}

impl DraftsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Create a new draft for an agent
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `branch_id` - The ID of the agent branch to use
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
    ///         .agents
    ///         .drafts
    ///         .create(
    ///             &"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///             &BodyCreateAgentDraftV1ConvaiAgentsAgentIDDraftsPost {
    ///                 branch_id: "agtbrch_8901k4t9z5defmb8vh3e9361y7nj".to_string(),
    ///                 conversation_config: HashMap::from([(
    ///                     "key".to_string(),
    ///                     serde_json::json!("value"),
    ///                 )]),
    ///                 platform_settings: HashMap::from([("key".to_string(), serde_json::json!("value"))]),
    ///                 workflow: AgentWorkflowRequestModel {
    ///                     edges: Some(HashMap::from([
    ///                         (
    ///                             "entry_to_tool_a".to_string(),
    ///                             WorkflowEdgeModelInput {
    ///                                 source: "entry_node".to_string(),
    ///                                 target: "tool_node_a".to_string(),
    ///                                 forward_condition: Some(
    ///                                     WorkflowEdgeModelInputForwardCondition::Expression {
    ///                                         data: WorkflowExpressionConditionModelInput {
    ///                                             label: None,
    ///                                             expression: AstNodeInput::AndOperator {
    ///                                                 data: AstAndOperatorNodeInput {
    ///                                                     children: vec![],
    ///                                                     ..Default::default()
    ///                                                 },
    ///                                             },
    ///                                         },
    ///                                     },
    ///                                 ),
    ///                                 ..Default::default()
    ///                             },
    ///                         ),
    ///                         (
    ///                             "start_to_entry".to_string(),
    ///                             WorkflowEdgeModelInput {
    ///                                 source: "start_node".to_string(),
    ///                                 target: "entry_node".to_string(),
    ///                                 forward_condition: Some(
    ///                                     WorkflowEdgeModelInputForwardCondition::Expression {
    ///                                         data: WorkflowExpressionConditionModelInput {
    ///                                             label: None,
    ///                                             expression: AstNodeInput::AndOperator {
    ///                                                 data: AstAndOperatorNodeInput {
    ///                                                     children: vec![],
    ///                                                     ..Default::default()
    ///                                                 },
    ///                                             },
    ///                                         },
    ///                                     },
    ///                                 ),
    ///                                 ..Default::default()
    ///                             },
    ///                         ),
    ///                         (
    ///                             "tool_a_to_failure".to_string(),
    ///                             WorkflowEdgeModelInput {
    ///                                 source: "tool_node_a".to_string(),
    ///                                 target: "failure_node".to_string(),
    ///                                 forward_condition: Some(
    ///                                     WorkflowEdgeModelInputForwardCondition::Expression {
    ///                                         data: WorkflowExpressionConditionModelInput {
    ///                                             label: None,
    ///                                             expression: AstNodeInput::AndOperator {
    ///                                                 data: AstAndOperatorNodeInput {
    ///                                                     children: vec![],
    ///                                                     ..Default::default()
    ///                                                 },
    ///                                             },
    ///                                         },
    ///                                     },
    ///                                 ),
    ///                                 ..Default::default()
    ///                             },
    ///                         ),
    ///                         (
    ///                             "tool_a_to_tool_b".to_string(),
    ///                             WorkflowEdgeModelInput {
    ///                                 source: "tool_node_a".to_string(),
    ///                                 target: "tool_node_b".to_string(),
    ///                                 forward_condition: Some(
    ///                                     WorkflowEdgeModelInputForwardCondition::Expression {
    ///                                         data: WorkflowExpressionConditionModelInput {
    ///                                             label: None,
    ///                                             expression: AstNodeInput::AndOperator {
    ///                                                 data: AstAndOperatorNodeInput {
    ///                                                     children: vec![],
    ///                                                     ..Default::default()
    ///                                                 },
    ///                                             },
    ///                                         },
    ///                                     },
    ///                                 ),
    ///                                 ..Default::default()
    ///                             },
    ///                         ),
    ///                         (
    ///                             "tool_b_to_agent_transfer".to_string(),
    ///                             WorkflowEdgeModelInput {
    ///                                 source: "tool_node_b".to_string(),
    ///                                 target: "success_transfer".to_string(),
    ///                                 forward_condition: Some(
    ///                                     WorkflowEdgeModelInputForwardCondition::Expression {
    ///                                         data: WorkflowExpressionConditionModelInput {
    ///                                             label: None,
    ///                                             expression: AstNodeInput::AndOperator {
    ///                                                 data: AstAndOperatorNodeInput {
    ///                                                     children: vec![],
    ///                                                     ..Default::default()
    ///                                                 },
    ///                                             },
    ///                                         },
    ///                                     },
    ///                                 ),
    ///                                 ..Default::default()
    ///                             },
    ///                         ),
    ///                         (
    ///                             "tool_b_to_conversation".to_string(),
    ///                             WorkflowEdgeModelInput {
    ///                                 source: "tool_node_b".to_string(),
    ///                                 target: "success_conversation".to_string(),
    ///                                 forward_condition: Some(
    ///                                     WorkflowEdgeModelInputForwardCondition::Expression {
    ///                                         data: WorkflowExpressionConditionModelInput {
    ///                                             label: None,
    ///                                             expression: AstNodeInput::AndOperator {
    ///                                                 data: AstAndOperatorNodeInput {
    ///                                                     children: vec![],
    ///                                                     ..Default::default()
    ///                                                 },
    ///                                             },
    ///                                         },
    ///                                     },
    ///                                 ),
    ///                                 ..Default::default()
    ///                             },
    ///                         ),
    ///                         (
    ///                             "tool_b_to_end".to_string(),
    ///                             WorkflowEdgeModelInput {
    ///                                 source: "tool_node_b".to_string(),
    ///                                 target: "success_end".to_string(),
    ///                                 forward_condition: Some(
    ///                                     WorkflowEdgeModelInputForwardCondition::Expression {
    ///                                         data: WorkflowExpressionConditionModelInput {
    ///                                             label: None,
    ///                                             expression: AstNodeInput::AndOperator {
    ///                                                 data: AstAndOperatorNodeInput {
    ///                                                     children: vec![],
    ///                                                     ..Default::default()
    ///                                                 },
    ///                                             },
    ///                                         },
    ///                                     },
    ///                                 ),
    ///                                 ..Default::default()
    ///                             },
    ///                         ),
    ///                         (
    ///                             "tool_b_to_phone".to_string(),
    ///                             WorkflowEdgeModelInput {
    ///                                 source: "tool_node_b".to_string(),
    ///                                 target: "success_phone".to_string(),
    ///                                 forward_condition: Some(
    ///                                     WorkflowEdgeModelInputForwardCondition::Expression {
    ///                                         data: WorkflowExpressionConditionModelInput {
    ///                                             label: None,
    ///                                             expression: AstNodeInput::AndOperator {
    ///                                                 data: AstAndOperatorNodeInput {
    ///                                                     children: vec![],
    ///                                                     ..Default::default()
    ///                                                 },
    ///                                             },
    ///                                         },
    ///                                     },
    ///                                 ),
    ///                                 ..Default::default()
    ///                             },
    ///                         ),
    ///                     ])),
    ///                     nodes: Some(HashMap::from([
    ///                         (
    ///                             "entry_node".to_string(),
    ///                             AgentWorkflowRequestModelNodesValue::End {
    ///                                 data: WorkflowEndNodeModelInput {
    ///                                     ..Default::default()
    ///                                 },
    ///                             },
    ///                         ),
    ///                         (
    ///                             "failure_node".to_string(),
    ///                             AgentWorkflowRequestModelNodesValue::End {
    ///                                 data: WorkflowEndNodeModelInput {
    ///                                     ..Default::default()
    ///                                 },
    ///                             },
    ///                         ),
    ///                         (
    ///                             "start_node".to_string(),
    ///                             AgentWorkflowRequestModelNodesValue::End {
    ///                                 data: WorkflowEndNodeModelInput {
    ///                                     ..Default::default()
    ///                                 },
    ///                             },
    ///                         ),
    ///                         (
    ///                             "success_conversation".to_string(),
    ///                             AgentWorkflowRequestModelNodesValue::End {
    ///                                 data: WorkflowEndNodeModelInput {
    ///                                     ..Default::default()
    ///                                 },
    ///                             },
    ///                         ),
    ///                         (
    ///                             "success_end".to_string(),
    ///                             AgentWorkflowRequestModelNodesValue::End {
    ///                                 data: WorkflowEndNodeModelInput {
    ///                                     ..Default::default()
    ///                                 },
    ///                             },
    ///                         ),
    ///                         (
    ///                             "success_phone".to_string(),
    ///                             AgentWorkflowRequestModelNodesValue::End {
    ///                                 data: WorkflowEndNodeModelInput {
    ///                                     ..Default::default()
    ///                                 },
    ///                             },
    ///                         ),
    ///                         (
    ///                             "success_transfer".to_string(),
    ///                             AgentWorkflowRequestModelNodesValue::End {
    ///                                 data: WorkflowEndNodeModelInput {
    ///                                     ..Default::default()
    ///                                 },
    ///                             },
    ///                         ),
    ///                         (
    ///                             "tool_node_a".to_string(),
    ///                             AgentWorkflowRequestModelNodesValue::End {
    ///                                 data: WorkflowEndNodeModelInput {
    ///                                     ..Default::default()
    ///                                 },
    ///                             },
    ///                         ),
    ///                         (
    ///                             "tool_node_b".to_string(),
    ///                             AgentWorkflowRequestModelNodesValue::End {
    ///                                 data: WorkflowEndNodeModelInput {
    ///                                     ..Default::default()
    ///                                 },
    ///                             },
    ///                         ),
    ///                     ])),
    ///                     ..Default::default()
    ///                 },
    ///                 name: "name".to_string(),
    ///                 tags: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        agent_id: &str,
        request: &BodyCreateAgentDraftV1ConvaiAgentsAgentIdDraftsPost,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/convai/agents/{}/drafts", agent_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("branch_id", request.branch_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete a draft for an agent
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `branch_id` - The ID of the agent branch to use
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
    ///         .agents
    ///         .drafts
    ///         .delete(
    ///             &"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///             &AgentsDraftsDeleteQueryRequest {
    ///                 branch_id: "agtbrch_8901k4t9z5defmb8vh3e9361y7nj".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        agent_id: &str,
        request: &AgentsDraftsDeleteQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/convai/agents/{}/drafts", agent_id),
                None,
                QueryBuilder::new()
                    .string("branch_id", request.branch_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
