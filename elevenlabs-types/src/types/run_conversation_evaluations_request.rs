pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RunConversationEvaluationsRequest {
    /// ID of the single evaluation criterion to rerun.
    #[serde(default)]
    pub evaluation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<AnalysisScope>,
    /// ID of the agent whose evaluation result to rerun, for conversations that transferred between agents. If omitted, a conversation-scoped evaluation reruns for the agent that started the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluated_agent_id: Option<String>,
    /// Branch of `evaluated_agent_id` whose evaluation result to rerun, for conversations that visited the agent on more than one branch. If omitted, the result reruns for every branch of that agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluated_branch_id: Option<String>,
}

impl RunConversationEvaluationsRequest {
    pub fn builder() -> RunConversationEvaluationsRequestBuilder {
        <RunConversationEvaluationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunConversationEvaluationsRequestBuilder {
    evaluation_id: Option<String>,
    scope: Option<AnalysisScope>,
    evaluated_agent_id: Option<String>,
    evaluated_branch_id: Option<String>,
}

impl RunConversationEvaluationsRequestBuilder {
    pub fn evaluation_id(mut self, value: impl Into<String>) -> Self {
        self.evaluation_id = Some(value.into());
        self
    }

    pub fn scope(mut self, value: AnalysisScope) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn evaluated_agent_id(mut self, value: impl Into<String>) -> Self {
        self.evaluated_agent_id = Some(value.into());
        self
    }

    pub fn evaluated_branch_id(mut self, value: impl Into<String>) -> Self {
        self.evaluated_branch_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RunConversationEvaluationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`evaluation_id`](RunConversationEvaluationsRequestBuilder::evaluation_id)
    pub fn build(self) -> Result<RunConversationEvaluationsRequest, BuildError> {
        Ok(RunConversationEvaluationsRequest {
            evaluation_id: self.evaluation_id.ok_or_else(|| BuildError::missing_field("evaluation_id"))?,
            scope: self.scope,
            evaluated_agent_id: self.evaluated_agent_id,
            evaluated_branch_id: self.evaluated_branch_id,
        })
    }
}

