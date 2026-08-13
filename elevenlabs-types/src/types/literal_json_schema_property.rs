pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Schema property for literal JSON types. IMPORTANT: Only ONE of the following fields can be set: description (LLM provides value), dynamic_variable (value from variable), is_system_provided (system provides value), constant_value (fixed value), or is_omitted (parameter is omitted). These are mutually exclusive.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LiteralJsonSchemaProperty {
    pub r#type: LiteralJsonSchemaPropertyType,
    /// The description of the property. When set, the LLM will provide the value based on this description. Mutually exclusive with dynamic_variable, is_system_provided, constant_value, and is_omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// List of allowed string values for string type parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<String>>,
    /// If true, the value will be populated by the system at runtime. Used by API Integration Webhook tools for templating. Mutually exclusive with description, dynamic_variable, constant_value, and is_omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_system_provided: Option<bool>,
    /// The name of the dynamic variable to use for this property's value. Mutually exclusive with description, is_system_provided, constant_value, and is_omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_variable: Option<String>,
    /// When set, the LLM provides the value but the runtime rejects any value not present in the list held by this dynamic variable. Use to let the LLM pick from a server-verified set (e.g. the IDs the current user is allowed to access). Requires description; mutually exclusive with dynamic_variable, is_system_provided, constant_value, and is_omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values_dynamic_variable: Option<String>,
    /// A constant value to use for this property. Mutually exclusive with description, dynamic_variable, is_system_provided, and is_omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_value: Option<LiteralJsonSchemaPropertyConstantValue>,
    /// If true, this parameter will be completely omitted from the request. Only valid for optional parameters. Mutually exclusive with description, dynamic_variable, is_system_provided, and constant_value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_omitted: Option<bool>,
}

impl LiteralJsonSchemaProperty {
    pub fn builder() -> LiteralJsonSchemaPropertyBuilder {
        <LiteralJsonSchemaPropertyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LiteralJsonSchemaPropertyBuilder {
    r#type: Option<LiteralJsonSchemaPropertyType>,
    description: Option<String>,
    r#enum: Option<Vec<String>>,
    is_system_provided: Option<bool>,
    dynamic_variable: Option<String>,
    allowed_values_dynamic_variable: Option<String>,
    constant_value: Option<LiteralJsonSchemaPropertyConstantValue>,
    is_omitted: Option<bool>,
}

impl LiteralJsonSchemaPropertyBuilder {
    pub fn r#type(mut self, value: LiteralJsonSchemaPropertyType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn r#enum(mut self, value: Vec<String>) -> Self {
        self.r#enum = Some(value);
        self
    }

    pub fn is_system_provided(mut self, value: bool) -> Self {
        self.is_system_provided = Some(value);
        self
    }

    pub fn dynamic_variable(mut self, value: impl Into<String>) -> Self {
        self.dynamic_variable = Some(value.into());
        self
    }

    pub fn allowed_values_dynamic_variable(mut self, value: impl Into<String>) -> Self {
        self.allowed_values_dynamic_variable = Some(value.into());
        self
    }

    pub fn constant_value(mut self, value: LiteralJsonSchemaPropertyConstantValue) -> Self {
        self.constant_value = Some(value);
        self
    }

    pub fn is_omitted(mut self, value: bool) -> Self {
        self.is_omitted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LiteralJsonSchemaProperty`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](LiteralJsonSchemaPropertyBuilder::r#type)
    pub fn build(self) -> Result<LiteralJsonSchemaProperty, BuildError> {
        Ok(LiteralJsonSchemaProperty {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            description: self.description,
            r#enum: self.r#enum,
            is_system_provided: self.is_system_provided,
            dynamic_variable: self.dynamic_variable,
            allowed_values_dynamic_variable: self.allowed_values_dynamic_variable,
            constant_value: self.constant_value,
            is_omitted: self.is_omitted,
        })
    }
}
