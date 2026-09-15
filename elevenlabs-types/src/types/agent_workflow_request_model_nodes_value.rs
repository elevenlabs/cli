pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum AgentWorkflowRequestModelNodesValue {
        #[serde(rename = "end")]
        #[non_exhaustive]
        End {
            #[serde(skip_serializing_if = "Option::is_none")]
            position: Option<PositionInput>,
            #[serde(skip_serializing_if = "Option::is_none")]
            edge_order: Option<Vec<String>>,
        },

        #[serde(rename = "override_agent")]
        #[non_exhaustive]
        OverrideAgent {
            #[serde(skip_serializing_if = "Option::is_none")]
            position: Option<PositionInput>,
            #[serde(skip_serializing_if = "Option::is_none")]
            edge_order: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            conversation_config: Option<ConversationalConfigApiModelWorkflowOverrideInput>,
            #[serde(skip_serializing_if = "Option::is_none")]
            additional_prompt: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            additional_knowledge_base: Option<Vec<KnowledgeBaseLocator>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            additional_tool_ids: Option<Vec<String>>,
            #[serde(default)]
            label: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            entry_behavior: Option<EntryBehavior>,
        },

        #[serde(rename = "phone_number")]
        #[non_exhaustive]
        PhoneNumber {
            #[serde(skip_serializing_if = "Option::is_none")]
            custom_sip_headers: Option<Vec<WorkflowPhoneNumberNodeModelInputCustomSipHeadersItem>>,
            transfer_destination: WorkflowPhoneNumberNodeModelInputTransferDestination,
            #[serde(skip_serializing_if = "Option::is_none")]
            transfer_type: Option<TransferTypeEnum>,
            #[serde(skip_serializing_if = "Option::is_none")]
            sip_refer_play_dialtone: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            uui: Option<UuiTransferConfig>,
            #[serde(skip_serializing_if = "Option::is_none")]
            post_dial_digits: Option<WorkflowPhoneNumberNodeModelInputPostDialDigits>,
            #[serde(skip_serializing_if = "Option::is_none")]
            position: Option<PositionInput>,
            #[serde(skip_serializing_if = "Option::is_none")]
            edge_order: Option<Vec<String>>,
        },

        #[serde(rename = "standalone_agent")]
        #[non_exhaustive]
        StandaloneAgent {
            #[serde(skip_serializing_if = "Option::is_none")]
            position: Option<PositionInput>,
            #[serde(skip_serializing_if = "Option::is_none")]
            edge_order: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            agent_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            node_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            delay_ms: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            transfer_message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            enable_transferred_agent_first_message: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            preserve_client_tts_overrides: Option<bool>,
        },

        #[serde(rename = "start")]
        #[non_exhaustive]
        Start {
            #[serde(skip_serializing_if = "Option::is_none")]
            position: Option<PositionInput>,
            #[serde(skip_serializing_if = "Option::is_none")]
            edge_order: Option<Vec<String>>,
        },

        #[serde(rename = "tool")]
        #[non_exhaustive]
        Tool {
            #[serde(skip_serializing_if = "Option::is_none")]
            position: Option<PositionInput>,
            #[serde(skip_serializing_if = "Option::is_none")]
            edge_order: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            tools: Option<Vec<WorkflowToolLocator>>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl AgentWorkflowRequestModelNodesValue {
    pub fn end() -> Self {
        Self::End { position: None, edge_order: None }
    }

    pub fn override_agent(label: String) -> Self {
        Self::OverrideAgent { position: None, edge_order: None, conversation_config: None, additional_prompt: None, additional_knowledge_base: None, additional_tool_ids: None, label, entry_behavior: None }
    }

    pub fn phone_number(transfer_destination: WorkflowPhoneNumberNodeModelInputTransferDestination) -> Self {
        Self::PhoneNumber { custom_sip_headers: None, transfer_destination, transfer_type: None, sip_refer_play_dialtone: None, uui: None, post_dial_digits: None, position: None, edge_order: None }
    }

    pub fn standalone_agent() -> Self {
        Self::StandaloneAgent { position: None, edge_order: None, agent_id: None, node_id: None, delay_ms: None, transfer_message: None, enable_transferred_agent_first_message: None, preserve_client_tts_overrides: None }
    }

    pub fn start() -> Self {
        Self::Start { position: None, edge_order: None }
    }

    pub fn tool() -> Self {
        Self::Tool { position: None, edge_order: None, tools: None }
    }

    pub fn end_with_position(position: PositionInput, edge_order: Option<Vec<String>>) -> Self {
        Self::End { position: Some(position), edge_order }
    }

    pub fn end_with_edge_order(position: Option<PositionInput>, edge_order: Vec<String>) -> Self {
        Self::End { position, edge_order: Some(edge_order) }
    }

    pub fn override_agent_with_position(position: PositionInput, edge_order: Option<Vec<String>>, conversation_config: Option<ConversationalConfigApiModelWorkflowOverrideInput>, additional_prompt: Option<String>, additional_knowledge_base: Option<Vec<KnowledgeBaseLocator>>, additional_tool_ids: Option<Vec<String>>, label: String, entry_behavior: Option<EntryBehavior>) -> Self {
        Self::OverrideAgent { position: Some(position), edge_order, conversation_config, additional_prompt, additional_knowledge_base, additional_tool_ids, label, entry_behavior }
    }

    pub fn override_agent_with_edge_order(position: Option<PositionInput>, edge_order: Vec<String>, conversation_config: Option<ConversationalConfigApiModelWorkflowOverrideInput>, additional_prompt: Option<String>, additional_knowledge_base: Option<Vec<KnowledgeBaseLocator>>, additional_tool_ids: Option<Vec<String>>, label: String, entry_behavior: Option<EntryBehavior>) -> Self {
        Self::OverrideAgent { position, edge_order: Some(edge_order), conversation_config, additional_prompt, additional_knowledge_base, additional_tool_ids, label, entry_behavior }
    }

    pub fn override_agent_with_conversation_config(position: Option<PositionInput>, edge_order: Option<Vec<String>>, conversation_config: ConversationalConfigApiModelWorkflowOverrideInput, additional_prompt: Option<String>, additional_knowledge_base: Option<Vec<KnowledgeBaseLocator>>, additional_tool_ids: Option<Vec<String>>, label: String, entry_behavior: Option<EntryBehavior>) -> Self {
        Self::OverrideAgent { position, edge_order, conversation_config: Some(conversation_config), additional_prompt, additional_knowledge_base, additional_tool_ids, label, entry_behavior }
    }

    pub fn override_agent_with_additional_prompt(position: Option<PositionInput>, edge_order: Option<Vec<String>>, conversation_config: Option<ConversationalConfigApiModelWorkflowOverrideInput>, additional_prompt: String, additional_knowledge_base: Option<Vec<KnowledgeBaseLocator>>, additional_tool_ids: Option<Vec<String>>, label: String, entry_behavior: Option<EntryBehavior>) -> Self {
        Self::OverrideAgent { position, edge_order, conversation_config, additional_prompt: Some(additional_prompt), additional_knowledge_base, additional_tool_ids, label, entry_behavior }
    }

    pub fn override_agent_with_additional_knowledge_base(position: Option<PositionInput>, edge_order: Option<Vec<String>>, conversation_config: Option<ConversationalConfigApiModelWorkflowOverrideInput>, additional_prompt: Option<String>, additional_knowledge_base: Vec<KnowledgeBaseLocator>, additional_tool_ids: Option<Vec<String>>, label: String, entry_behavior: Option<EntryBehavior>) -> Self {
        Self::OverrideAgent { position, edge_order, conversation_config, additional_prompt, additional_knowledge_base: Some(additional_knowledge_base), additional_tool_ids, label, entry_behavior }
    }

    pub fn override_agent_with_additional_tool_ids(position: Option<PositionInput>, edge_order: Option<Vec<String>>, conversation_config: Option<ConversationalConfigApiModelWorkflowOverrideInput>, additional_prompt: Option<String>, additional_knowledge_base: Option<Vec<KnowledgeBaseLocator>>, additional_tool_ids: Vec<String>, label: String, entry_behavior: Option<EntryBehavior>) -> Self {
        Self::OverrideAgent { position, edge_order, conversation_config, additional_prompt, additional_knowledge_base, additional_tool_ids: Some(additional_tool_ids), label, entry_behavior }
    }

    pub fn override_agent_with_entry_behavior(position: Option<PositionInput>, edge_order: Option<Vec<String>>, conversation_config: Option<ConversationalConfigApiModelWorkflowOverrideInput>, additional_prompt: Option<String>, additional_knowledge_base: Option<Vec<KnowledgeBaseLocator>>, additional_tool_ids: Option<Vec<String>>, label: String, entry_behavior: EntryBehavior) -> Self {
        Self::OverrideAgent { position, edge_order, conversation_config, additional_prompt, additional_knowledge_base, additional_tool_ids, label, entry_behavior: Some(entry_behavior) }
    }

    pub fn phone_number_with_custom_sip_headers(custom_sip_headers: Vec<WorkflowPhoneNumberNodeModelInputCustomSipHeadersItem>, transfer_destination: WorkflowPhoneNumberNodeModelInputTransferDestination, transfer_type: Option<TransferTypeEnum>, sip_refer_play_dialtone: Option<bool>, uui: Option<UuiTransferConfig>, post_dial_digits: Option<WorkflowPhoneNumberNodeModelInputPostDialDigits>, position: Option<PositionInput>, edge_order: Option<Vec<String>>) -> Self {
        Self::PhoneNumber { custom_sip_headers: Some(custom_sip_headers), transfer_destination, transfer_type, sip_refer_play_dialtone, uui, post_dial_digits, position, edge_order }
    }

    pub fn phone_number_with_transfer_type(custom_sip_headers: Option<Vec<WorkflowPhoneNumberNodeModelInputCustomSipHeadersItem>>, transfer_destination: WorkflowPhoneNumberNodeModelInputTransferDestination, transfer_type: TransferTypeEnum, sip_refer_play_dialtone: Option<bool>, uui: Option<UuiTransferConfig>, post_dial_digits: Option<WorkflowPhoneNumberNodeModelInputPostDialDigits>, position: Option<PositionInput>, edge_order: Option<Vec<String>>) -> Self {
        Self::PhoneNumber { custom_sip_headers, transfer_destination, transfer_type: Some(transfer_type), sip_refer_play_dialtone, uui, post_dial_digits, position, edge_order }
    }

    pub fn phone_number_with_sip_refer_play_dialtone(custom_sip_headers: Option<Vec<WorkflowPhoneNumberNodeModelInputCustomSipHeadersItem>>, transfer_destination: WorkflowPhoneNumberNodeModelInputTransferDestination, transfer_type: Option<TransferTypeEnum>, sip_refer_play_dialtone: bool, uui: Option<UuiTransferConfig>, post_dial_digits: Option<WorkflowPhoneNumberNodeModelInputPostDialDigits>, position: Option<PositionInput>, edge_order: Option<Vec<String>>) -> Self {
        Self::PhoneNumber { custom_sip_headers, transfer_destination, transfer_type, sip_refer_play_dialtone: Some(sip_refer_play_dialtone), uui, post_dial_digits, position, edge_order }
    }

    pub fn phone_number_with_uui(custom_sip_headers: Option<Vec<WorkflowPhoneNumberNodeModelInputCustomSipHeadersItem>>, transfer_destination: WorkflowPhoneNumberNodeModelInputTransferDestination, transfer_type: Option<TransferTypeEnum>, sip_refer_play_dialtone: Option<bool>, uui: UuiTransferConfig, post_dial_digits: Option<WorkflowPhoneNumberNodeModelInputPostDialDigits>, position: Option<PositionInput>, edge_order: Option<Vec<String>>) -> Self {
        Self::PhoneNumber { custom_sip_headers, transfer_destination, transfer_type, sip_refer_play_dialtone, uui: Some(uui), post_dial_digits, position, edge_order }
    }

    pub fn phone_number_with_post_dial_digits(custom_sip_headers: Option<Vec<WorkflowPhoneNumberNodeModelInputCustomSipHeadersItem>>, transfer_destination: WorkflowPhoneNumberNodeModelInputTransferDestination, transfer_type: Option<TransferTypeEnum>, sip_refer_play_dialtone: Option<bool>, uui: Option<UuiTransferConfig>, post_dial_digits: WorkflowPhoneNumberNodeModelInputPostDialDigits, position: Option<PositionInput>, edge_order: Option<Vec<String>>) -> Self {
        Self::PhoneNumber { custom_sip_headers, transfer_destination, transfer_type, sip_refer_play_dialtone, uui, post_dial_digits: Some(post_dial_digits), position, edge_order }
    }

    pub fn phone_number_with_position(custom_sip_headers: Option<Vec<WorkflowPhoneNumberNodeModelInputCustomSipHeadersItem>>, transfer_destination: WorkflowPhoneNumberNodeModelInputTransferDestination, transfer_type: Option<TransferTypeEnum>, sip_refer_play_dialtone: Option<bool>, uui: Option<UuiTransferConfig>, post_dial_digits: Option<WorkflowPhoneNumberNodeModelInputPostDialDigits>, position: PositionInput, edge_order: Option<Vec<String>>) -> Self {
        Self::PhoneNumber { custom_sip_headers, transfer_destination, transfer_type, sip_refer_play_dialtone, uui, post_dial_digits, position: Some(position), edge_order }
    }

    pub fn phone_number_with_edge_order(custom_sip_headers: Option<Vec<WorkflowPhoneNumberNodeModelInputCustomSipHeadersItem>>, transfer_destination: WorkflowPhoneNumberNodeModelInputTransferDestination, transfer_type: Option<TransferTypeEnum>, sip_refer_play_dialtone: Option<bool>, uui: Option<UuiTransferConfig>, post_dial_digits: Option<WorkflowPhoneNumberNodeModelInputPostDialDigits>, position: Option<PositionInput>, edge_order: Vec<String>) -> Self {
        Self::PhoneNumber { custom_sip_headers, transfer_destination, transfer_type, sip_refer_play_dialtone, uui, post_dial_digits, position, edge_order: Some(edge_order) }
    }

    pub fn standalone_agent_with_position(position: PositionInput, edge_order: Option<Vec<String>>, agent_id: Option<String>, node_id: Option<String>, delay_ms: Option<i64>, transfer_message: Option<String>, enable_transferred_agent_first_message: Option<bool>, preserve_client_tts_overrides: Option<bool>) -> Self {
        Self::StandaloneAgent { position: Some(position), edge_order, agent_id, node_id, delay_ms, transfer_message, enable_transferred_agent_first_message, preserve_client_tts_overrides }
    }

    pub fn standalone_agent_with_edge_order(position: Option<PositionInput>, edge_order: Vec<String>, agent_id: Option<String>, node_id: Option<String>, delay_ms: Option<i64>, transfer_message: Option<String>, enable_transferred_agent_first_message: Option<bool>, preserve_client_tts_overrides: Option<bool>) -> Self {
        Self::StandaloneAgent { position, edge_order: Some(edge_order), agent_id, node_id, delay_ms, transfer_message, enable_transferred_agent_first_message, preserve_client_tts_overrides }
    }

    pub fn standalone_agent_with_agent_id(position: Option<PositionInput>, edge_order: Option<Vec<String>>, agent_id: String, node_id: Option<String>, delay_ms: Option<i64>, transfer_message: Option<String>, enable_transferred_agent_first_message: Option<bool>, preserve_client_tts_overrides: Option<bool>) -> Self {
        Self::StandaloneAgent { position, edge_order, agent_id: Some(agent_id), node_id, delay_ms, transfer_message, enable_transferred_agent_first_message, preserve_client_tts_overrides }
    }

    pub fn standalone_agent_with_node_id(position: Option<PositionInput>, edge_order: Option<Vec<String>>, agent_id: Option<String>, node_id: String, delay_ms: Option<i64>, transfer_message: Option<String>, enable_transferred_agent_first_message: Option<bool>, preserve_client_tts_overrides: Option<bool>) -> Self {
        Self::StandaloneAgent { position, edge_order, agent_id, node_id: Some(node_id), delay_ms, transfer_message, enable_transferred_agent_first_message, preserve_client_tts_overrides }
    }

    pub fn standalone_agent_with_delay_ms(position: Option<PositionInput>, edge_order: Option<Vec<String>>, agent_id: Option<String>, node_id: Option<String>, delay_ms: i64, transfer_message: Option<String>, enable_transferred_agent_first_message: Option<bool>, preserve_client_tts_overrides: Option<bool>) -> Self {
        Self::StandaloneAgent { position, edge_order, agent_id, node_id, delay_ms: Some(delay_ms), transfer_message, enable_transferred_agent_first_message, preserve_client_tts_overrides }
    }

    pub fn standalone_agent_with_transfer_message(position: Option<PositionInput>, edge_order: Option<Vec<String>>, agent_id: Option<String>, node_id: Option<String>, delay_ms: Option<i64>, transfer_message: String, enable_transferred_agent_first_message: Option<bool>, preserve_client_tts_overrides: Option<bool>) -> Self {
        Self::StandaloneAgent { position, edge_order, agent_id, node_id, delay_ms, transfer_message: Some(transfer_message), enable_transferred_agent_first_message, preserve_client_tts_overrides }
    }

    pub fn standalone_agent_with_enable_transferred_agent_first_message(position: Option<PositionInput>, edge_order: Option<Vec<String>>, agent_id: Option<String>, node_id: Option<String>, delay_ms: Option<i64>, transfer_message: Option<String>, enable_transferred_agent_first_message: bool, preserve_client_tts_overrides: Option<bool>) -> Self {
        Self::StandaloneAgent { position, edge_order, agent_id, node_id, delay_ms, transfer_message, enable_transferred_agent_first_message: Some(enable_transferred_agent_first_message), preserve_client_tts_overrides }
    }

    pub fn standalone_agent_with_preserve_client_tts_overrides(position: Option<PositionInput>, edge_order: Option<Vec<String>>, agent_id: Option<String>, node_id: Option<String>, delay_ms: Option<i64>, transfer_message: Option<String>, enable_transferred_agent_first_message: Option<bool>, preserve_client_tts_overrides: bool) -> Self {
        Self::StandaloneAgent { position, edge_order, agent_id, node_id, delay_ms, transfer_message, enable_transferred_agent_first_message, preserve_client_tts_overrides: Some(preserve_client_tts_overrides) }
    }

    pub fn start_with_position(position: PositionInput, edge_order: Option<Vec<String>>) -> Self {
        Self::Start { position: Some(position), edge_order }
    }

    pub fn start_with_edge_order(position: Option<PositionInput>, edge_order: Vec<String>) -> Self {
        Self::Start { position, edge_order: Some(edge_order) }
    }

    pub fn tool_with_position(position: PositionInput, edge_order: Option<Vec<String>>, tools: Option<Vec<WorkflowToolLocator>>) -> Self {
        Self::Tool { position: Some(position), edge_order, tools }
    }

    pub fn tool_with_edge_order(position: Option<PositionInput>, edge_order: Vec<String>, tools: Option<Vec<WorkflowToolLocator>>) -> Self {
        Self::Tool { position, edge_order: Some(edge_order), tools }
    }

    pub fn tool_with_tools(position: Option<PositionInput>, edge_order: Option<Vec<String>>, tools: Vec<WorkflowToolLocator>) -> Self {
        Self::Tool { position, edge_order, tools: Some(tools) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
