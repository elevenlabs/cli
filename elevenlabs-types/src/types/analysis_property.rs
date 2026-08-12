pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Data collection property with optional per-item LLM override for post-call analysis.
/// 
/// TODO: migrate to composition (value_schema: LiteralJsonSchemaProperty + llm) instead of
/// inheritance, so this generalizes cleanly to object/array schemas in the future.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnalysisProperty {
    pub r#type: AnalysisPropertyType,
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
    pub constant_value: Option<AnalysisPropertyConstantValue>,
    /// If true, this parameter will be completely omitted from the request. Only valid for optional parameters. Mutually exclusive with description, dynamic_variable, is_system_provided, and constant_value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_omitted: Option<bool>,
    /// LLM model to use for this analysis item. If not set, uses agent's analysis_llm default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm: Option<Llm>,
}

impl AnalysisProperty {
    pub fn builder() -> AnalysisPropertyBuilder {
        <AnalysisPropertyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnalysisPropertyBuilder {
    r#type: Option<AnalysisPropertyType>,
    description: Option<String>,
    r#enum: Option<Vec<String>>,
    is_system_provided: Option<bool>,
    dynamic_variable: Option<String>,
    allowed_values_dynamic_variable: Option<String>,
    constant_value: Option<AnalysisPropertyConstantValue>,
    is_omitted: Option<bool>,
    llm: Option<Llm>,
}

impl AnalysisPropertyBuilder {
    pub fn r#type(mut self, value: AnalysisPropertyType) -> Self {
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

    pub fn constant_value(mut self, value: AnalysisPropertyConstantValue) -> Self {
        self.constant_value = Some(value);
        self
    }

    pub fn is_omitted(mut self, value: bool) -> Self {
        self.is_omitted = Some(value);
        self
    }

    pub fn llm(mut self, value: Llm) -> Self {
        self.llm = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AnalysisProperty`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](AnalysisPropertyBuilder::r#type)
    pub fn build(self) -> Result<AnalysisProperty, BuildError> {
        Ok(AnalysisProperty {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            description: self.description,
            r#enum: self.r#enum,
            is_system_provided: self.is_system_provided,
            dynamic_variable: self.dynamic_variable,
            allowed_values_dynamic_variable: self.allowed_values_dynamic_variable,
            constant_value: self.constant_value,
            is_omitted: self.is_omitted,
            llm: self.llm,
        })
    }
}
