pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetResponseUnitTestResponseModel {
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
    /// A prompt that evaluates whether the agent's response is successful. Should return True or False.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_condition: Option<String>,
    /// Non-empty list of example responses that should be considered successful
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_examples: Option<Vec<AgentSuccessfulResponseExample>>,
    /// Non-empty list of example responses that should be considered failures
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_examples: Option<Vec<AgentFailureResponseExample>>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

impl GetResponseUnitTestResponseModel {
    pub fn builder() -> GetResponseUnitTestResponseModelBuilder {
        <GetResponseUnitTestResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetResponseUnitTestResponseModelBuilder {
    from_conversation_metadata: Option<TestFromConversationMetadataOutput>,
    dynamic_variables: Option<HashMap<String, serde_json::Value>>,
    chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>,
    conversation_initiation_source: Option<ConversationInitiationSource>,
    success_condition: Option<String>,
    success_examples: Option<Vec<AgentSuccessfulResponseExample>>,
    failure_examples: Option<Vec<AgentFailureResponseExample>>,
    id: Option<String>,
    name: Option<String>,
}

impl GetResponseUnitTestResponseModelBuilder {
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

    pub fn success_condition(mut self, value: impl Into<String>) -> Self {
        self.success_condition = Some(value.into());
        self
    }

    pub fn success_examples(mut self, value: Vec<AgentSuccessfulResponseExample>) -> Self {
        self.success_examples = Some(value);
        self
    }

    pub fn failure_examples(mut self, value: Vec<AgentFailureResponseExample>) -> Self {
        self.failure_examples = Some(value);
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

    /// Consumes the builder and constructs a [`GetResponseUnitTestResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetResponseUnitTestResponseModelBuilder::id)
    /// - [`name`](GetResponseUnitTestResponseModelBuilder::name)
    pub fn build(self) -> Result<GetResponseUnitTestResponseModel, BuildError> {
        Ok(GetResponseUnitTestResponseModel {
            from_conversation_metadata: self.from_conversation_metadata,
            dynamic_variables: self.dynamic_variables,
            chat_history: self.chat_history,
            conversation_initiation_source: self.conversation_initiation_source,
            success_condition: self.success_condition,
            success_examples: self.success_examples,
            failure_examples: self.failure_examples,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
