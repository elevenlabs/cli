pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DataCollectionResultCommonModel {
    #[serde(default)]
    pub data_collection_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_schema: Option<LiteralJsonSchemaProperty>,
    #[serde(default)]
    pub rationale: String,
}

impl DataCollectionResultCommonModel {
    pub fn builder() -> DataCollectionResultCommonModelBuilder {
        <DataCollectionResultCommonModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DataCollectionResultCommonModelBuilder {
    data_collection_id: Option<String>,
    value: Option<serde_json::Value>,
    json_schema: Option<LiteralJsonSchemaProperty>,
    rationale: Option<String>,
}

impl DataCollectionResultCommonModelBuilder {
    pub fn data_collection_id(mut self, value: impl Into<String>) -> Self {
        self.data_collection_id = Some(value.into());
        self
    }

    pub fn value(mut self, value: serde_json::Value) -> Self {
        self.value = Some(value);
        self
    }

    pub fn json_schema(mut self, value: LiteralJsonSchemaProperty) -> Self {
        self.json_schema = Some(value);
        self
    }

    pub fn rationale(mut self, value: impl Into<String>) -> Self {
        self.rationale = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DataCollectionResultCommonModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data_collection_id`](DataCollectionResultCommonModelBuilder::data_collection_id)
    /// - [`rationale`](DataCollectionResultCommonModelBuilder::rationale)
    pub fn build(self) -> Result<DataCollectionResultCommonModel, BuildError> {
        Ok(DataCollectionResultCommonModel {
            data_collection_id: self.data_collection_id.ok_or_else(|| BuildError::missing_field("data_collection_id"))?,
            value: self.value,
            json_schema: self.json_schema,
            rationale: self.rationale.ok_or_else(|| BuildError::missing_field("rationale"))?,
        })
    }
}
