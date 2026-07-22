pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetToolCallUnitTestResponseModel {
    /// Metadata of a conversation this test was created from (if applicable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_conversation_metadata: Option<TestFromConversationMetadataOutput>,
    /// Dynamic variables to replace in the agent config during testing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_variables: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>,
    /// Simulate the test as if the conversation originated from this channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_initiation_source: Option<ConversationInitiationSource>,
    /// How to evaluate the agent's tool call (if any). If empty, the tool call is not evaluated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_parameters: Option<UnitTestToolCallEvaluationModelOutput>,
    /// If set to True this test will pass if any tool call returned by the LLM matches the criteria. Otherwise it will fail if more than one tool is returned by the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_any_tool_matches: Option<bool>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

impl GetToolCallUnitTestResponseModel {
    pub fn builder() -> GetToolCallUnitTestResponseModelBuilder {
        <GetToolCallUnitTestResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetToolCallUnitTestResponseModelBuilder {
    from_conversation_metadata: Option<TestFromConversationMetadataOutput>,
    dynamic_variables: Option<HashMap<String, serde_json::Value>>,
    chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>,
    conversation_initiation_source: Option<ConversationInitiationSource>,
    tool_call_parameters: Option<UnitTestToolCallEvaluationModelOutput>,
    check_any_tool_matches: Option<bool>,
    id: Option<String>,
    name: Option<String>,
}

impl GetToolCallUnitTestResponseModelBuilder {
    pub fn from_conversation_metadata(mut self, value: TestFromConversationMetadataOutput) -> Self {
        self.from_conversation_metadata = Some(value);
        self
    }

    pub fn dynamic_variables(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.dynamic_variables = Some(value);
        self
    }

    pub fn chat_history(mut self, value: Vec<ConversationHistoryTranscriptCommonModelOutput>) -> Self {
        self.chat_history = Some(value);
        self
    }

    pub fn conversation_initiation_source(mut self, value: ConversationInitiationSource) -> Self {
        self.conversation_initiation_source = Some(value);
        self
    }

    pub fn tool_call_parameters(mut self, value: UnitTestToolCallEvaluationModelOutput) -> Self {
        self.tool_call_parameters = Some(value);
        self
    }

    pub fn check_any_tool_matches(mut self, value: bool) -> Self {
        self.check_any_tool_matches = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetToolCallUnitTestResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetToolCallUnitTestResponseModelBuilder::id)
    /// - [`name`](GetToolCallUnitTestResponseModelBuilder::name)
    pub fn build(self) -> Result<GetToolCallUnitTestResponseModel, BuildError> {
        Ok(GetToolCallUnitTestResponseModel {
            from_conversation_metadata: self.from_conversation_metadata,
            dynamic_variables: self.dynamic_variables,
            chat_history: self.chat_history,
            conversation_initiation_source: self.conversation_initiation_source,
            tool_call_parameters: self.tool_call_parameters,
            check_any_tool_matches: self.check_any_tool_matches,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
