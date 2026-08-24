pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "result_type")]
#[non_exhaustive]
pub enum ConversationHistoryTranscriptSystemToolResultCommonModelOutputResult {
        #[serde(rename = "dummy")]
        #[non_exhaustive]
        Dummy {
            #[serde(flatten)]
            data: DummyToolResultModel,
        },

        #[serde(rename = "end_call_success")]
        #[non_exhaustive]
        EndCallSuccess {
            #[serde(flatten)]
            data: EndCallToolResultModel,
        },

        #[serde(rename = "knowledge_base_rag_success")]
        #[non_exhaustive]
        KnowledgeBaseRagSuccess {
            #[serde(flatten)]
            data: KnowledgeBaseRagToolResultModel,
        },

        #[serde(rename = "knowledge_base_success")]
        #[non_exhaustive]
        KnowledgeBaseSuccess {
            #[serde(flatten)]
            data: KnowledgeBaseToolResultModel,
        },

        #[serde(rename = "language_detection_success")]
        #[non_exhaustive]
        LanguageDetectionSuccess {
            #[serde(flatten)]
            data: LanguageDetectionToolResultModel,
        },

        #[serde(rename = "play_dtmf_error")]
        #[non_exhaustive]
        PlayDtmfError {
            #[serde(flatten)]
            data: PlayDtmfResultErrorModel,
        },

        #[serde(rename = "play_dtmf_success")]
        #[non_exhaustive]
        PlayDtmfSuccess {
            #[serde(flatten)]
            data: PlayDtmfResultSuccessModel,
        },

        #[serde(rename = "skip_turn_success")]
        #[non_exhaustive]
        SkipTurnSuccess {
            #[serde(flatten)]
            data: SkipTurnToolResponseModel,
        },

        #[serde(rename = "testing_tool_result")]
        #[non_exhaustive]
        TestingToolResult {
            #[serde(flatten)]
            data: TestToolResultModel,
        },

        #[serde(rename = "transfer_to_agent_error")]
        #[non_exhaustive]
        TransferToAgentError {
            #[serde(flatten)]
            data: TransferToAgentToolResultErrorModel,
        },

        #[serde(rename = "transfer_to_agent_success")]
        #[non_exhaustive]
        TransferToAgentSuccess {
            #[serde(skip_serializing_if = "Option::is_none")]
            status: Option<String>,
            #[serde(default)]
            from_agent: String,
            #[serde(default)]
            to_agent: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            to_node: Option<String>,
            #[serde(default)]
            condition: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            delay_ms: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            transfer_message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            enable_transferred_agent_first_message: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            branch_info: Option<TransferToAgentToolResultSuccessModelOutputBranchInfo>,
            #[serde(skip_serializing_if = "Option::is_none")]
            preserve_client_tts_overrides: Option<bool>,
        },

        #[serde(rename = "transfer_to_number_error")]
        #[non_exhaustive]
        TransferToNumberError {
            #[serde(flatten)]
            data: TransferToNumberResultErrorModel,
        },

        #[serde(rename = "transfer_to_number_exotel_success")]
        #[non_exhaustive]
        TransferToNumberExotelSuccess {
            #[serde(flatten)]
            data: TransferToNumberResultExotelSuccessModel,
        },

        #[serde(rename = "transfer_to_number_sip_success")]
        #[non_exhaustive]
        TransferToNumberSipSuccess {
            #[serde(flatten)]
            data: TransferToNumberResultSipSuccessModel,
        },

        #[serde(rename = "transfer_to_number_twilio_success")]
        #[non_exhaustive]
        TransferToNumberTwilioSuccess {
            #[serde(flatten)]
            data: TransferToNumberResultTwilioSuccessModel,
        },

        #[serde(rename = "voicemail_detection_success")]
        #[non_exhaustive]
        VoicemailDetectionSuccess {
            #[serde(flatten)]
            data: VoiceMailDetectionResultSuccessModel,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl ConversationHistoryTranscriptSystemToolResultCommonModelOutputResult {
    pub fn dummy(data: DummyToolResultModel) -> Self {
        Self::Dummy { data }
    }

    pub fn end_call_success(data: EndCallToolResultModel) -> Self {
        Self::EndCallSuccess { data }
    }

    pub fn knowledge_base_rag_success(data: KnowledgeBaseRagToolResultModel) -> Self {
        Self::KnowledgeBaseRagSuccess { data }
    }

    pub fn knowledge_base_success(data: KnowledgeBaseToolResultModel) -> Self {
        Self::KnowledgeBaseSuccess { data }
    }

    pub fn language_detection_success(data: LanguageDetectionToolResultModel) -> Self {
        Self::LanguageDetectionSuccess { data }
    }

    pub fn play_dtmf_error(data: PlayDtmfResultErrorModel) -> Self {
        Self::PlayDtmfError { data }
    }

    pub fn play_dtmf_success(data: PlayDtmfResultSuccessModel) -> Self {
        Self::PlayDtmfSuccess { data }
    }

    pub fn skip_turn_success(data: SkipTurnToolResponseModel) -> Self {
        Self::SkipTurnSuccess { data }
    }

    pub fn testing_tool_result(data: TestToolResultModel) -> Self {
        Self::TestingToolResult { data }
    }

    pub fn transfer_to_agent_error(data: TransferToAgentToolResultErrorModel) -> Self {
        Self::TransferToAgentError { data }
    }

    pub fn transfer_to_agent_success(from_agent: String, to_agent: String, condition: String) -> Self {
        Self::TransferToAgentSuccess { status: None, from_agent, to_agent, to_node: None, condition, delay_ms: None, transfer_message: None, enable_transferred_agent_first_message: None, branch_info: None, preserve_client_tts_overrides: None }
    }

    pub fn transfer_to_number_error(data: TransferToNumberResultErrorModel) -> Self {
        Self::TransferToNumberError { data }
    }

    pub fn transfer_to_number_exotel_success(data: TransferToNumberResultExotelSuccessModel) -> Self {
        Self::TransferToNumberExotelSuccess { data }
    }

    pub fn transfer_to_number_sip_success(data: TransferToNumberResultSipSuccessModel) -> Self {
        Self::TransferToNumberSipSuccess { data }
    }

    pub fn transfer_to_number_twilio_success(data: TransferToNumberResultTwilioSuccessModel) -> Self {
        Self::TransferToNumberTwilioSuccess { data }
    }

    pub fn voicemail_detection_success(data: VoiceMailDetectionResultSuccessModel) -> Self {
        Self::VoicemailDetectionSuccess { data }
    }

    pub fn transfer_to_agent_success_with_status(status: String, from_agent: String, to_agent: String, to_node: Option<String>, condition: String, delay_ms: Option<i64>, transfer_message: Option<String>, enable_transferred_agent_first_message: Option<bool>, branch_info: Option<TransferToAgentToolResultSuccessModelOutputBranchInfo>, preserve_client_tts_overrides: Option<bool>) -> Self {
        Self::TransferToAgentSuccess { status: Some(status), from_agent, to_agent, to_node, condition, delay_ms, transfer_message, enable_transferred_agent_first_message, branch_info, preserve_client_tts_overrides }
    }

    pub fn transfer_to_agent_success_with_to_node(status: Option<String>, from_agent: String, to_agent: String, to_node: String, condition: String, delay_ms: Option<i64>, transfer_message: Option<String>, enable_transferred_agent_first_message: Option<bool>, branch_info: Option<TransferToAgentToolResultSuccessModelOutputBranchInfo>, preserve_client_tts_overrides: Option<bool>) -> Self {
        Self::TransferToAgentSuccess { status, from_agent, to_agent, to_node: Some(to_node), condition, delay_ms, transfer_message, enable_transferred_agent_first_message, branch_info, preserve_client_tts_overrides }
    }

    pub fn transfer_to_agent_success_with_delay_ms(status: Option<String>, from_agent: String, to_agent: String, to_node: Option<String>, condition: String, delay_ms: i64, transfer_message: Option<String>, enable_transferred_agent_first_message: Option<bool>, branch_info: Option<TransferToAgentToolResultSuccessModelOutputBranchInfo>, preserve_client_tts_overrides: Option<bool>) -> Self {
        Self::TransferToAgentSuccess { status, from_agent, to_agent, to_node, condition, delay_ms: Some(delay_ms), transfer_message, enable_transferred_agent_first_message, branch_info, preserve_client_tts_overrides }
    }

    pub fn transfer_to_agent_success_with_transfer_message(status: Option<String>, from_agent: String, to_agent: String, to_node: Option<String>, condition: String, delay_ms: Option<i64>, transfer_message: String, enable_transferred_agent_first_message: Option<bool>, branch_info: Option<TransferToAgentToolResultSuccessModelOutputBranchInfo>, preserve_client_tts_overrides: Option<bool>) -> Self {
        Self::TransferToAgentSuccess { status, from_agent, to_agent, to_node, condition, delay_ms, transfer_message: Some(transfer_message), enable_transferred_agent_first_message, branch_info, preserve_client_tts_overrides }
    }

    pub fn transfer_to_agent_success_with_enable_transferred_agent_first_message(status: Option<String>, from_agent: String, to_agent: String, to_node: Option<String>, condition: String, delay_ms: Option<i64>, transfer_message: Option<String>, enable_transferred_agent_first_message: bool, branch_info: Option<TransferToAgentToolResultSuccessModelOutputBranchInfo>, preserve_client_tts_overrides: Option<bool>) -> Self {
        Self::TransferToAgentSuccess { status, from_agent, to_agent, to_node, condition, delay_ms, transfer_message, enable_transferred_agent_first_message: Some(enable_transferred_agent_first_message), branch_info, preserve_client_tts_overrides }
    }

    pub fn transfer_to_agent_success_with_branch_info(status: Option<String>, from_agent: String, to_agent: String, to_node: Option<String>, condition: String, delay_ms: Option<i64>, transfer_message: Option<String>, enable_transferred_agent_first_message: Option<bool>, branch_info: TransferToAgentToolResultSuccessModelOutputBranchInfo, preserve_client_tts_overrides: Option<bool>) -> Self {
        Self::TransferToAgentSuccess { status, from_agent, to_agent, to_node, condition, delay_ms, transfer_message, enable_transferred_agent_first_message, branch_info: Some(branch_info), preserve_client_tts_overrides }
    }

    pub fn transfer_to_agent_success_with_preserve_client_tts_overrides(status: Option<String>, from_agent: String, to_agent: String, to_node: Option<String>, condition: String, delay_ms: Option<i64>, transfer_message: Option<String>, enable_transferred_agent_first_message: Option<bool>, branch_info: Option<TransferToAgentToolResultSuccessModelOutputBranchInfo>, preserve_client_tts_overrides: bool) -> Self {
        Self::TransferToAgentSuccess { status, from_agent, to_agent, to_node, condition, delay_ms, transfer_message, enable_transferred_agent_first_message, branch_info, preserve_client_tts_overrides: Some(preserve_client_tts_overrides) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
