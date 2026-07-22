pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Definition for a tool the client can call.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Tool {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "inputSchema")]
    #[serde(default)]
    pub input_schema: HashMap<String, serde_json::Value>,
    #[serde(rename = "outputSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<Icon>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<ToolAnnotations>,
    #[serde(rename = "_meta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution: Option<ToolExecution>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Tool {
    pub fn builder() -> ToolBuilder {
        <ToolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolBuilder {
    name: Option<String>,
    title: Option<String>,
    description: Option<String>,
    input_schema: Option<HashMap<String, serde_json::Value>>,
    output_schema: Option<HashMap<String, serde_json::Value>>,
    icons: Option<Vec<Icon>>,
    annotations: Option<ToolAnnotations>,
    meta: Option<HashMap<String, serde_json::Value>>,
    execution: Option<ToolExecution>,
}

impl ToolBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn input_schema(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.input_schema = Some(value);
        self
    }

    pub fn output_schema(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.output_schema = Some(value);
        self
    }

    pub fn icons(mut self, value: Vec<Icon>) -> Self {
        self.icons = Some(value);
        self
    }

    pub fn annotations(mut self, value: ToolAnnotations) -> Self {
        self.annotations = Some(value);
        self
    }

    pub fn meta(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn execution(mut self, value: ToolExecution) -> Self {
        self.execution = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Tool`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ToolBuilder::name)
    /// - [`input_schema`](ToolBuilder::input_schema)
    pub fn build(self) -> Result<Tool, BuildError> {
        Ok(Tool {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            title: self.title,
            description: self.description,
            input_schema: self.input_schema.ok_or_else(|| BuildError::missing_field("input_schema"))?,
            output_schema: self.output_schema,
            icons: self.icons,
            annotations: self.annotations,
            meta: self.meta,
            execution: self.execution,
            extra: Default::default(),
        })
    }
}
