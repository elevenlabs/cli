pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum AgentWorkflowResponseModelNodesValue {
        #[serde(rename = "end")]
        #[non_exhaustive]
        End {
            #[serde(default)]
            position: PositionOutput,
            #[serde(default)]
            edge_order: Vec<String>,
            outcome: WorkflowEndNodeOutcome,
        },

        #[serde(rename = "override_agent")]
        #[non_exhaustive]
        OverrideAgent {
            #[serde(default)]
            position: PositionOutput,
            #[serde(default)]
            edge_order: Vec<String>,
            #[serde(default)]
            conversation_config: ConversationalConfigApiModelWorkflowOverrideOutput,
            #[serde(default)]
            additional_prompt: String,
            #[serde(default)]
            additional_knowledge_base: Vec<KnowledgeBaseLocator>,
            #[serde(default)]
            additional_tool_ids: Vec<String>,
            #[serde(default)]
            label: String,
            entry_behavior: EntryBehavior,
        },

        #[serde(rename = "phone_number")]
        #[non_exhaustive]
        PhoneNumber {
            #[serde(default)]
            custom_sip_headers: Vec<WorkflowPhoneNumberNodeModelOutputCustomSipHeadersItem>,
            transfer_destination: WorkflowPhoneNumberNodeModelOutputTransferDestination,
            transfer_type: TransferTypeEnum,
            #[serde(default)]
            sip_refer_play_dialtone: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            uui: Option<UuiTransferConfig>,
            #[serde(skip_serializing_if = "Option::is_none")]
            post_dial_digits: Option<WorkflowPhoneNumberNodeModelOutputPostDialDigits>,
            #[serde(default)]
            position: PositionOutput,
            #[serde(default)]
            edge_order: Vec<String>,
        },

        #[serde(rename = "standalone_agent")]
        #[non_exhaustive]
        StandaloneAgent {
            #[serde(default)]
            position: PositionOutput,
            #[serde(default)]
            edge_order: Vec<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            agent_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            branch_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            node_id: Option<String>,
            #[serde(default)]
            delay_ms: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            transfer_message: Option<String>,
            #[serde(default)]
            enable_transferred_agent_first_message: bool,
            #[serde(default)]
            preserve_client_tts_overrides: bool,
            #[serde(default)]
            preserve_voice_settings: bool,
        },

        #[serde(rename = "start")]
        #[non_exhaustive]
        Start {
            #[serde(default)]
            position: PositionOutput,
            #[serde(default)]
            edge_order: Vec<String>,
        },

        #[serde(rename = "tool")]
        #[non_exhaustive]
        Tool {
            #[serde(default)]
            position: PositionOutput,
            #[serde(default)]
            edge_order: Vec<String>,
            #[serde(default)]
            tools: Vec<WorkflowToolLocator>,
        },

        #[serde(rename = "say")]
        #[non_exhaustive]
        Say {
            #[serde(default)]
            position: PositionOutput,
            #[serde(default)]
            edge_order: Vec<String>,
            #[serde(default)]
            conversation_config: ConversationalConfigApiModelWorkflowOverrideOutput,
            #[serde(default)]
            additional_prompt: String,
            #[serde(default)]
            additional_knowledge_base: Vec<KnowledgeBaseLocator>,
            #[serde(default)]
            additional_tool_ids: Vec<String>,
            #[serde(default)]
            label: String,
            message: WorkflowSayNodeModelOutputMessage,
            #[serde(default)]
            wait_for_speech_completion: bool,
            #[serde(default)]
            wait_for_user_response: bool,
        },

        #[serde(rename = "update_state")]
        #[non_exhaustive]
        UpdateState {
            #[serde(default)]
            position: PositionOutput,
            #[serde(default)]
            edge_order: Vec<String>,
            #[serde(default)]
            label: String,
            #[serde(default)]
            updates: Vec<WorkflowStateUpdateOutput>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl AgentWorkflowResponseModelNodesValue {
    pub fn end(position: PositionOutput, edge_order: Vec<String>, outcome: WorkflowEndNodeOutcome) -> Self {
        Self::End { position, edge_order, outcome }
    }

    pub fn override_agent(position: PositionOutput, edge_order: Vec<String>, conversation_config: ConversationalConfigApiModelWorkflowOverrideOutput, additional_prompt: String, additional_knowledge_base: Vec<KnowledgeBaseLocator>, additional_tool_ids: Vec<String>, label: String, entry_behavior: EntryBehavior) -> Self {
        Self::OverrideAgent { position, edge_order, conversation_config, additional_prompt, additional_knowledge_base, additional_tool_ids, label, entry_behavior }
    }

    pub fn phone_number(custom_sip_headers: Vec<WorkflowPhoneNumberNodeModelOutputCustomSipHeadersItem>, transfer_destination: WorkflowPhoneNumberNodeModelOutputTransferDestination, transfer_type: TransferTypeEnum, sip_refer_play_dialtone: bool, position: PositionOutput, edge_order: Vec<String>) -> Self {
        Self::PhoneNumber { custom_sip_headers, transfer_destination, transfer_type, sip_refer_play_dialtone, uui: None, post_dial_digits: None, position, edge_order }
    }

    pub fn standalone_agent(position: PositionOutput, edge_order: Vec<String>, delay_ms: i64, enable_transferred_agent_first_message: bool, preserve_client_tts_overrides: bool, preserve_voice_settings: bool) -> Self {
        Self::StandaloneAgent { position, edge_order, agent_id: None, branch_id: None, node_id: None, delay_ms, transfer_message: None, enable_transferred_agent_first_message, preserve_client_tts_overrides, preserve_voice_settings }
    }

    pub fn start(position: PositionOutput, edge_order: Vec<String>) -> Self {
        Self::Start { position, edge_order }
    }

    pub fn tool(position: PositionOutput, edge_order: Vec<String>, tools: Vec<WorkflowToolLocator>) -> Self {
        Self::Tool { position, edge_order, tools }
    }

    pub fn say(position: PositionOutput, edge_order: Vec<String>, conversation_config: ConversationalConfigApiModelWorkflowOverrideOutput, additional_prompt: String, additional_knowledge_base: Vec<KnowledgeBaseLocator>, additional_tool_ids: Vec<String>, label: String, message: WorkflowSayNodeModelOutputMessage, wait_for_speech_completion: bool, wait_for_user_response: bool) -> Self {
        Self::Say { position, edge_order, conversation_config, additional_prompt, additional_knowledge_base, additional_tool_ids, label, message, wait_for_speech_completion, wait_for_user_response }
    }

    pub fn update_state(position: PositionOutput, edge_order: Vec<String>, label: String, updates: Vec<WorkflowStateUpdateOutput>) -> Self {
        Self::UpdateState { position, edge_order, label, updates }
    }

    pub fn phone_number_with_uui(custom_sip_headers: Vec<WorkflowPhoneNumberNodeModelOutputCustomSipHeadersItem>, transfer_destination: WorkflowPhoneNumberNodeModelOutputTransferDestination, transfer_type: TransferTypeEnum, sip_refer_play_dialtone: bool, uui: UuiTransferConfig, post_dial_digits: Option<WorkflowPhoneNumberNodeModelOutputPostDialDigits>, position: PositionOutput, edge_order: Vec<String>) -> Self {
        Self::PhoneNumber { custom_sip_headers, transfer_destination, transfer_type, sip_refer_play_dialtone, uui: Some(uui), post_dial_digits, position, edge_order }
    }

    pub fn phone_number_with_post_dial_digits(custom_sip_headers: Vec<WorkflowPhoneNumberNodeModelOutputCustomSipHeadersItem>, transfer_destination: WorkflowPhoneNumberNodeModelOutputTransferDestination, transfer_type: TransferTypeEnum, sip_refer_play_dialtone: bool, uui: Option<UuiTransferConfig>, post_dial_digits: WorkflowPhoneNumberNodeModelOutputPostDialDigits, position: PositionOutput, edge_order: Vec<String>) -> Self {
        Self::PhoneNumber { custom_sip_headers, transfer_destination, transfer_type, sip_refer_play_dialtone, uui, post_dial_digits: Some(post_dial_digits), position, edge_order }
    }

    pub fn standalone_agent_with_agent_id(position: PositionOutput, edge_order: Vec<String>, agent_id: String, branch_id: Option<String>, node_id: Option<String>, delay_ms: i64, transfer_message: Option<String>, enable_transferred_agent_first_message: bool, preserve_client_tts_overrides: bool, preserve_voice_settings: bool) -> Self {
        Self::StandaloneAgent { position, edge_order, agent_id: Some(agent_id), branch_id, node_id, delay_ms, transfer_message, enable_transferred_agent_first_message, preserve_client_tts_overrides, preserve_voice_settings }
    }

    pub fn standalone_agent_with_branch_id(position: PositionOutput, edge_order: Vec<String>, agent_id: Option<String>, branch_id: String, node_id: Option<String>, delay_ms: i64, transfer_message: Option<String>, enable_transferred_agent_first_message: bool, preserve_client_tts_overrides: bool, preserve_voice_settings: bool) -> Self {
        Self::StandaloneAgent { position, edge_order, agent_id, branch_id: Some(branch_id), node_id, delay_ms, transfer_message, enable_transferred_agent_first_message, preserve_client_tts_overrides, preserve_voice_settings }
    }

    pub fn standalone_agent_with_node_id(position: PositionOutput, edge_order: Vec<String>, agent_id: Option<String>, branch_id: Option<String>, node_id: String, delay_ms: i64, transfer_message: Option<String>, enable_transferred_agent_first_message: bool, preserve_client_tts_overrides: bool, preserve_voice_settings: bool) -> Self {
        Self::StandaloneAgent { position, edge_order, agent_id, branch_id, node_id: Some(node_id), delay_ms, transfer_message, enable_transferred_agent_first_message, preserve_client_tts_overrides, preserve_voice_settings }
    }

    pub fn standalone_agent_with_transfer_message(position: PositionOutput, edge_order: Vec<String>, agent_id: Option<String>, branch_id: Option<String>, node_id: Option<String>, delay_ms: i64, transfer_message: String, enable_transferred_agent_first_message: bool, preserve_client_tts_overrides: bool, preserve_voice_settings: bool) -> Self {
        Self::StandaloneAgent { position, edge_order, agent_id, branch_id, node_id, delay_ms, transfer_message: Some(transfer_message), enable_transferred_agent_first_message, preserve_client_tts_overrides, preserve_voice_settings }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
