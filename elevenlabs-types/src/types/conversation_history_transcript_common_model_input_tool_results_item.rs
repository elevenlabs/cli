pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ConversationHistoryTranscriptCommonModelInputToolResultsItem {
        ConversationHistoryTranscriptOtherToolsResultCommonModel(ConversationHistoryTranscriptOtherToolsResultCommonModel),

        ConversationHistoryTranscriptSystemToolResultCommonModelInput(ConversationHistoryTranscriptSystemToolResultCommonModelInput),

        ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelInput(ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelInput),

        ConversationHistoryTranscriptWorkflowToolsResultCommonModelInput(ConversationHistoryTranscriptWorkflowToolsResultCommonModelInput),
}

impl ConversationHistoryTranscriptCommonModelInputToolResultsItem {
    pub fn is_conversation_history_transcript_other_tools_result_common_model(&self) -> bool {
        matches!(self, Self::ConversationHistoryTranscriptOtherToolsResultCommonModel(_))
    }

    pub fn is_conversation_history_transcript_system_tool_result_common_model_input(&self) -> bool {
        matches!(self, Self::ConversationHistoryTranscriptSystemToolResultCommonModelInput(_))
    }

    pub fn is_conversation_history_transcript_api_integration_webhook_tools_result_common_model_input(&self) -> bool {
        matches!(self, Self::ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelInput(_))
    }

    pub fn is_conversation_history_transcript_workflow_tools_result_common_model_input(&self) -> bool {
        matches!(self, Self::ConversationHistoryTranscriptWorkflowToolsResultCommonModelInput(_))
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

    pub fn as_conversation_history_transcript_system_tool_result_common_model_input(&self) -> Option<&ConversationHistoryTranscriptSystemToolResultCommonModelInput> {
        match self {
                    Self::ConversationHistoryTranscriptSystemToolResultCommonModelInput(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_conversation_history_transcript_system_tool_result_common_model_input(self) -> Option<ConversationHistoryTranscriptSystemToolResultCommonModelInput> {
        match self {
                    Self::ConversationHistoryTranscriptSystemToolResultCommonModelInput(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_conversation_history_transcript_api_integration_webhook_tools_result_common_model_input(&self) -> Option<&ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelInput> {
        match self {
                    Self::ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelInput(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_conversation_history_transcript_api_integration_webhook_tools_result_common_model_input(self) -> Option<ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelInput> {
        match self {
                    Self::ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelInput(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_conversation_history_transcript_workflow_tools_result_common_model_input(&self) -> Option<&ConversationHistoryTranscriptWorkflowToolsResultCommonModelInput> {
        match self {
                    Self::ConversationHistoryTranscriptWorkflowToolsResultCommonModelInput(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_conversation_history_transcript_workflow_tools_result_common_model_input(self) -> Option<ConversationHistoryTranscriptWorkflowToolsResultCommonModelInput> {
        match self {
                    Self::ConversationHistoryTranscriptWorkflowToolsResultCommonModelInput(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for ConversationHistoryTranscriptCommonModelInputToolResultsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConversationHistoryTranscriptOtherToolsResultCommonModel(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ConversationHistoryTranscriptSystemToolResultCommonModelInput(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelInput(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ConversationHistoryTranscriptWorkflowToolsResultCommonModelInput(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
