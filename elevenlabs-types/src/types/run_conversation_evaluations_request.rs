pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RunConversationEvaluationsRequest {
    /// ID of the single evaluation criterion to rerun.
    #[serde(default)]
    pub evaluation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<AnalysisScope>,
}

impl RunConversationEvaluationsRequest {
    pub fn builder() -> RunConversationEvaluationsRequestBuilder {
        <RunConversationEvaluationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunConversationEvaluationsRequestBuilder {
    evaluation_id: Option<String>,
    scope: Option<AnalysisScope>,
}

impl RunConversationEvaluationsRequestBuilder {
    pub fn evaluation_id(mut self, value: impl Into<String>) -> Self {
        self.evaluation_id = Some(value.into());
        self
    }

    pub fn scope(mut self, value: AnalysisScope) -> Self {
        self.scope = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RunConversationEvaluationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`evaluation_id`](RunConversationEvaluationsRequestBuilder::evaluation_id)
    pub fn build(self) -> Result<RunConversationEvaluationsRequest, BuildError> {
        Ok(RunConversationEvaluationsRequest {
            evaluation_id: self.evaluation_id.ok_or_else(|| BuildError::missing_field("evaluation_id"))?,
            scope: self.scope,
        })
    }
}

