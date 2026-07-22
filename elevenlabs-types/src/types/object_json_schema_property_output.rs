pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ObjectJsonSchemaPropertyOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// When set, the entire parameter is populated from this dynamic variable at runtime. Mutually exclusive with description (LLM-provided value), constant_value, and is_omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_variable: Option<String>,
    /// When set, the entire object uses this constant JSON value at runtime. Mutually exclusive with description (LLM-provided object), dynamic_variable, and is_omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_value: Option<HashMap<String, serde_json::Value>>,
    /// If true, this parameter will be completely omitted from the request. Only valid for optional parameters. Mutually exclusive with description, dynamic_variable, and constant_value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_omitted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Box<ObjectJsonSchemaPropertyOutputPropertiesValue>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_constraints: Option<RequiredConstraints>,
}

impl ObjectJsonSchemaPropertyOutput {
    pub fn builder() -> ObjectJsonSchemaPropertyOutputBuilder {
        <ObjectJsonSchemaPropertyOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ObjectJsonSchemaPropertyOutputBuilder {
    description: Option<String>,
    dynamic_variable: Option<String>,
    constant_value: Option<HashMap<String, serde_json::Value>>,
    is_omitted: Option<bool>,
    r#type: Option<String>,
    required: Option<Vec<String>>,
    properties: Option<HashMap<String, Box<ObjectJsonSchemaPropertyOutputPropertiesValue>>>,
    required_constraints: Option<RequiredConstraints>,
}

impl ObjectJsonSchemaPropertyOutputBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn dynamic_variable(mut self, value: impl Into<String>) -> Self {
        self.dynamic_variable = Some(value.into());
        self
    }

    pub fn constant_value(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.constant_value = Some(value);
        self
    }

    pub fn is_omitted(mut self, value: bool) -> Self {
        self.is_omitted = Some(value);
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn required(mut self, value: Vec<String>) -> Self {
        self.required = Some(value);
        self
    }

    pub fn properties(mut self, value: HashMap<String, Box<ObjectJsonSchemaPropertyOutputPropertiesValue>>) -> Self {
        self.properties = Some(value);
        self
    }

    pub fn required_constraints(mut self, value: RequiredConstraints) -> Self {
        self.required_constraints = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ObjectJsonSchemaPropertyOutput`].
    pub fn build(self) -> Result<ObjectJsonSchemaPropertyOutput, BuildError> {
        Ok(ObjectJsonSchemaPropertyOutput {
            description: self.description,
            dynamic_variable: self.dynamic_variable,
            constant_value: self.constant_value,
            is_omitted: self.is_omitted,
            r#type: self.r#type,
            required: self.required,
            properties: self.properties,
            required_constraints: self.required_constraints,
        })
    }
}
