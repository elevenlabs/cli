pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An evaluation using the transcript and a prompt for a yes/no achieved answer
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PromptEvaluationCriteria {
    /// The unique identifier for the evaluation criteria
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    /// The type of evaluation criteria
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The prompt that the agent should use to evaluate the conversation
    #[serde(default)]
    pub conversation_goal_prompt: String,
    /// When evaluating the prompt, should the agent's knowledge base be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_knowledge_base: Option<bool>,
    /// The scope of transcript context used when evaluating this criterion. 'conversation' uses the full transcript; 'agent' uses only the portion where the defining agent was active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<AnalysisScope>,
    /// LLM model to use for this evaluation criteria. If not set, uses agent's analysis_llm default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm: Option<Llm>,
    /// How this criterion is scored. 'binary' resolves to success/failure/unknown. 'numeric_uniform' returns a number on the [0, max_score] scale which is normalized into the aggregate conversation success percentage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scoring_mode: Option<CriteriaScoringMode>,
    /// Maximum value of the numeric score scale (minimum is always 0). Only used when scoring_mode is 'numeric_uniform'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_score: Option<i64>,
    /// Optional free-text instructions describing how to assign values on the numeric scale. Only used when scoring_mode is 'numeric_uniform'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_instructions: Option<String>,
}

impl PromptEvaluationCriteria {
    pub fn builder() -> PromptEvaluationCriteriaBuilder {
        <PromptEvaluationCriteriaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PromptEvaluationCriteriaBuilder {
    id: Option<String>,
    name: Option<String>,
    r#type: Option<String>,
    conversation_goal_prompt: Option<String>,
    use_knowledge_base: Option<bool>,
    scope: Option<AnalysisScope>,
    llm: Option<Llm>,
    scoring_mode: Option<CriteriaScoringMode>,
    max_score: Option<i64>,
    score_instructions: Option<String>,
}

impl PromptEvaluationCriteriaBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
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

    pub fn scope(mut self, value: AnalysisScope) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn llm(mut self, value: Llm) -> Self {
        self.llm = Some(value);
        self
    }

    pub fn scoring_mode(mut self, value: CriteriaScoringMode) -> Self {
        self.scoring_mode = Some(value);
        self
    }

    pub fn max_score(mut self, value: i64) -> Self {
        self.max_score = Some(value);
        self
    }

    pub fn score_instructions(mut self, value: impl Into<String>) -> Self {
        self.score_instructions = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PromptEvaluationCriteria`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PromptEvaluationCriteriaBuilder::id)
    /// - [`name`](PromptEvaluationCriteriaBuilder::name)
    /// - [`conversation_goal_prompt`](PromptEvaluationCriteriaBuilder::conversation_goal_prompt)
    pub fn build(self) -> Result<PromptEvaluationCriteria, BuildError> {
        Ok(PromptEvaluationCriteria {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type,
            conversation_goal_prompt: self.conversation_goal_prompt.ok_or_else(|| BuildError::missing_field("conversation_goal_prompt"))?,
            use_knowledge_base: self.use_knowledge_base,
            scope: self.scope,
            llm: self.llm,
            scoring_mode: self.scoring_mode,
            max_score: self.max_score,
            score_instructions: self.score_instructions,
        })
    }
}
