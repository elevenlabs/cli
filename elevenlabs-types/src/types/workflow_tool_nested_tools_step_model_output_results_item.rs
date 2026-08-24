pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum WorkflowToolNestedToolsStepModelOutputResultsItem {
        ConversationHistoryTranscriptOtherToolsResultCommonModel(ConversationHistoryTranscriptOtherToolsResultCommonModel),

        ConversationHistoryTranscriptSystemToolResultCommonModelOutput(ConversationHistoryTranscriptSystemToolResultCommonModelOutput),

        ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutput(ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutput),

        ConversationHistoryTranscriptWorkflowToolsResultCommonModelOutput(Box<ConversationHistoryTranscriptWorkflowToolsResultCommonModelOutput>),
}

impl WorkflowToolNestedToolsStepModelOutputResultsItem {
    pub fn is_conversation_history_transcript_other_tools_result_common_model(&self) -> bool {
        matches!(self, Self::ConversationHistoryTranscriptOtherToolsResultCommonModel(_))
    }

    pub fn is_conversation_history_transcript_system_tool_result_common_model_output(&self) -> bool {
        matches!(self, Self::ConversationHistoryTranscriptSystemToolResultCommonModelOutput(_))
    }

    pub fn is_conversation_history_transcript_api_integration_webhook_tools_result_common_model_output(&self) -> bool {
        matches!(self, Self::ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutput(_))
    }

    pub fn is_conversation_history_transcript_workflow_tools_result_common_model_output(&self) -> bool {
        matches!(self, Self::ConversationHistoryTranscriptWorkflowToolsResultCommonModelOutput(_))
    }


    pub fn as_conversation_history_transcript_other_tools_result_common_model(&self) -> Option<&ConversationHistoryTranscriptOtherToolsResultCommonModel> {
        match self {
                    Self::ConversationHistoryTranscriptOtherToolsResultCommonModel(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_conversation_history_transcript_other_tools_result_common_model(self) -> Option<ConversationHistoryTranscriptOtherToolsResultCommonModel> {
        match self {
                    Self::ConversationHistoryTranscriptOtherToolsResultCommonModel(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_conversation_history_transcript_system_tool_result_common_model_output(&self) -> Option<&ConversationHistoryTranscriptSystemToolResultCommonModelOutput> {
        match self {
                    Self::ConversationHistoryTranscriptSystemToolResultCommonModelOutput(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_conversation_history_transcript_system_tool_result_common_model_output(self) -> Option<ConversationHistoryTranscriptSystemToolResultCommonModelOutput> {
        match self {
                    Self::ConversationHistoryTranscriptSystemToolResultCommonModelOutput(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_conversation_history_transcript_api_integration_webhook_tools_result_common_model_output(&self) -> Option<&ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutput> {
        match self {
                    Self::ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutput(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_conversation_history_transcript_api_integration_webhook_tools_result_common_model_output(self) -> Option<ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutput> {
        match self {
                    Self::ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutput(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_conversation_history_transcript_workflow_tools_result_common_model_output(&self) -> Option<&Box<ConversationHistoryTranscriptWorkflowToolsResultCommonModelOutput>> {
        match self {
                    Self::ConversationHistoryTranscriptWorkflowToolsResultCommonModelOutput(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_conversation_history_transcript_workflow_tools_result_common_model_output(self) -> Option<ConversationHistoryTranscriptWorkflowToolsResultCommonModelOutput> {
        match self {
                    Self::ConversationHistoryTranscriptWorkflowToolsResultCommonModelOutput(value) => Some(*value),
                    _ => None,
                }
    }
}

impl fmt::Display for WorkflowToolNestedToolsStepModelOutputResultsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConversationHistoryTranscriptOtherToolsResultCommonModel(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ConversationHistoryTranscriptSystemToolResultCommonModelOutput(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutput(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ConversationHistoryTranscriptWorkflowToolsResultCommonModelOutput(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
