pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UserEvaluationAnalysisItem {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// LLM to use for this item. If unset, the attaching agent's analysis_llm default is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm: Option<Llm>,
    /// The prompt used to evaluate the conversation.
    #[serde(default)]
    pub conversation_goal_prompt: String,
    /// Whether the attaching agent's knowledge base is used when evaluating this criterion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_knowledge_base: Option<bool>,
    /// How this criterion is scored (binary or numeric_uniform).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scoring: Option<UserEvaluationAnalysisItemScoring>,
}

impl UserEvaluationAnalysisItem {
    pub fn builder() -> UserEvaluationAnalysisItemBuilder {
        <UserEvaluationAnalysisItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserEvaluationAnalysisItemBuilder {
    name: Option<String>,
    source: Option<String>,
    llm: Option<Llm>,
    conversation_goal_prompt: Option<String>,
    use_knowledge_base: Option<bool>,
    scoring: Option<UserEvaluationAnalysisItemScoring>,
}

impl UserEvaluationAnalysisItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn llm(mut self, value: Llm) -> Self {
        self.llm = Some(value);
        self
    }

    pub fn conversation_goal_prompt(mut self, value: impl Into<String>) -> Self {
        self.conversation_goal_prompt = Some(value.into());
        self
    }

    pub fn use_knowledge_base(mut self, value: bool) -> Self {
        self.use_knowledge_base = Some(value);
        self
    }

    pub fn scoring(mut self, value: UserEvaluationAnalysisItemScoring) -> Self {
        self.scoring = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UserEvaluationAnalysisItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](UserEvaluationAnalysisItemBuilder::name)
    /// - [`conversation_goal_prompt`](UserEvaluationAnalysisItemBuilder::conversation_goal_prompt)
    pub fn build(self) -> Result<UserEvaluationAnalysisItem, BuildError> {
        Ok(UserEvaluationAnalysisItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            source: self.source,
            llm: self.llm,
            conversation_goal_prompt: self.conversation_goal_prompt.ok_or_else(|| BuildError::missing_field("conversation_goal_prompt"))?,
            use_knowledge_base: self.use_knowledge_base,
            scoring: self.scoring,
        })
    }
}
