pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "system_tool_type")]
#[non_exhaustive]
pub enum SystemToolConfigOutputParams {
        #[serde(rename = "end_call")]
        #[non_exhaustive]
        EndCall {
            #[serde(flatten)]
            data: EndCallToolConfig,
        },

        #[serde(rename = "end_procedure")]
        #[non_exhaustive]
        EndProcedure {
            #[serde(flatten)]
            data: EndProcedureToolConfig,
        },

        #[serde(rename = "knowledge_base")]
        #[non_exhaustive]
        KnowledgeBase {
            #[serde(flatten)]
            data: KnowledgeBaseToolConfig,
        },

        #[serde(rename = "knowledge_base_rag")]
        #[non_exhaustive]
        KnowledgeBaseRag {
            #[serde(flatten)]
            data: KnowledgeBaseRagToolConfig,
        },

        #[serde(rename = "language_detection")]
        #[non_exhaustive]
        LanguageDetection {
            #[serde(flatten)]
            data: LanguageDetectionToolConfig,
        },

        #[serde(rename = "play_keypad_touch_tone")]
        #[non_exhaustive]
        PlayKeypadTouchTone {
            #[serde(flatten)]
            data: PlayDtmfToolConfig,
        },

        #[serde(rename = "run_subagent")]
        #[non_exhaustive]
        RunSubagent {
            #[serde(default)]
            agents: Vec<SubAgentOutput>,
        },

        #[serde(rename = "skip_turn")]
        #[non_exhaustive]
        SkipTurn {
            #[serde(flatten)]
            data: SkipTurnToolConfig,
        },

        #[serde(rename = "start_procedure")]
        #[non_exhaustive]
        StartProcedure {
            #[serde(flatten)]
            data: StartProcedureToolConfig,
        },

        #[serde(rename = "transfer_to_agent")]
        #[non_exhaustive]
        TransferToAgent {
            #[serde(default)]
            transfers: Vec<AgentTransferOutput>,
        },

        #[serde(rename = "transfer_to_number")]
        #[non_exhaustive]
        TransferToNumber {
            #[serde(default)]
            transfers: Vec<PhoneNumberTransfer>,
            #[serde(skip_serializing_if = "Option::is_none")]
            enable_client_message: Option<bool>,
        },

        #[serde(rename = "voicemail_detection")]
        #[non_exhaustive]
        VoicemailDetection {
            #[serde(flatten)]
            data: VoicemailDetectionToolConfig,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl SystemToolConfigOutputParams {
    pub fn end_call(data: EndCallToolConfig) -> Self {
        Self::EndCall { data }
    }

    pub fn end_procedure(data: EndProcedureToolConfig) -> Self {
        Self::EndProcedure { data }
    }

    pub fn knowledge_base(data: KnowledgeBaseToolConfig) -> Self {
        Self::KnowledgeBase { data }
    }

    pub fn knowledge_base_rag(data: KnowledgeBaseRagToolConfig) -> Self {
        Self::KnowledgeBaseRag { data }
    }

    pub fn language_detection(data: LanguageDetectionToolConfig) -> Self {
        Self::LanguageDetection { data }
    }

    pub fn play_keypad_touch_tone(data: PlayDtmfToolConfig) -> Self {
        Self::PlayKeypadTouchTone { data }
    }

    pub fn run_subagent(agents: Vec<SubAgentOutput>) -> Self {
        Self::RunSubagent { agents }
    }

    pub fn skip_turn(data: SkipTurnToolConfig) -> Self {
        Self::SkipTurn { data }
    }

    pub fn start_procedure(data: StartProcedureToolConfig) -> Self {
        Self::StartProcedure { data }
    }

    pub fn transfer_to_agent(transfers: Vec<AgentTransferOutput>) -> Self {
        Self::TransferToAgent { transfers }
    }

    pub fn transfer_to_number(transfers: Vec<PhoneNumberTransfer>) -> Self {
        Self::TransferToNumber { transfers, enable_client_message: None }
    }

    pub fn voicemail_detection(data: VoicemailDetectionToolConfig) -> Self {
        Self::VoicemailDetection { data }
    }

    pub fn transfer_to_number_with_enable_client_message(transfers: Vec<PhoneNumberTransfer>, enable_client_message: bool) -> Self {
        Self::TransferToNumber { transfers, enable_client_message: Some(enable_client_message) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
