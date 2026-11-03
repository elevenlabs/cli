pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserDataCollectionAnalysisItemInput {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// LLM to use for this item. If unset, the attaching agent's analysis_llm default is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm: Option<Llm>,
    /// Type/description/enum describing the value to extract.
    pub value_schema: DataCollectionValueSchema,
}

impl UserDataCollectionAnalysisItemInput {
    pub fn builder() -> UserDataCollectionAnalysisItemInputBuilder {
        <UserDataCollectionAnalysisItemInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserDataCollectionAnalysisItemInputBuilder {
    name: Option<String>,
    source: Option<String>,
    llm: Option<Llm>,
    value_schema: Option<DataCollectionValueSchema>,
}

impl UserDataCollectionAnalysisItemInputBuilder {
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

    pub fn value_schema(mut self, value: DataCollectionValueSchema) -> Self {
        self.value_schema = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UserDataCollectionAnalysisItemInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](UserDataCollectionAnalysisItemInputBuilder::name)
    /// - [`value_schema`](UserDataCollectionAnalysisItemInputBuilder::value_schema)
    pub fn build(self) -> Result<UserDataCollectionAnalysisItemInput, BuildError> {
        Ok(UserDataCollectionAnalysisItemInput {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            source: self.source,
            llm: self.llm,
            value_schema: self.value_schema.ok_or_else(|| BuildError::missing_field("value_schema"))?,
        })
    }
}
