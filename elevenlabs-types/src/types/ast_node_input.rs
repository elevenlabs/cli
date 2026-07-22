pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum AstNodeInput {
        #[serde(rename = "add_operator")]
        #[non_exhaustive]
        AddOperator {
            left: Box<AstNodeInput>,
            right: Box<AstNodeInput>,
        },

        #[serde(rename = "and_operator")]
        #[non_exhaustive]
        AndOperator {
            #[serde(default)]
            children: Vec<Box<AstNodeInput>>,
        },

        #[serde(rename = "boolean_literal")]
        #[non_exhaustive]
        BooleanLiteral {
            #[serde(default)]
            value: bool,
        },

        #[serde(rename = "conditional_operator")]
        #[non_exhaustive]
        ConditionalOperator {
            condition: Box<AstNodeInput>,
            #[serde(rename = "trueExpression")]
            true_expression: Box<AstNodeInput>,
            #[serde(rename = "falseExpression")]
            false_expression: Box<AstNodeInput>,
        },

        #[serde(rename = "div_operator")]
        #[non_exhaustive]
        DivOperator {
            left: Box<AstNodeInput>,
            right: Box<AstNodeInput>,
        },

        #[serde(rename = "dynamic_variable")]
        #[non_exhaustive]
        DynamicVariable {
            #[serde(default)]
            name: String,
        },

        #[serde(rename = "eq_operator")]
        #[non_exhaustive]
        EqOperator {
            left: Box<AstNodeInput>,
            right: Box<AstNodeInput>,
        },

        #[serde(rename = "gt_operator")]
        #[non_exhaustive]
        GtOperator {
            left: Box<AstNodeInput>,
            right: Box<AstNodeInput>,
        },

        #[serde(rename = "gte_operator")]
        #[non_exhaustive]
        GteOperator {
            left: Box<AstNodeInput>,
            right: Box<AstNodeInput>,
        },

        #[serde(rename = "llm")]
        #[non_exhaustive]
        Llm {
            #[serde(skip_serializing_if = "Option::is_none")]
            value_schema: Option<LlmLiteralJsonSchemaProperty>,
            #[serde(skip_serializing_if = "Option::is_none")]
            prompt: Option<String>,
        },

        #[serde(rename = "lt_operator")]
        #[non_exhaustive]
        LtOperator {
            left: Box<AstNodeInput>,
            right: Box<AstNodeInput>,
        },

        #[serde(rename = "lte_operator")]
        #[non_exhaustive]
        LteOperator {
            left: Box<AstNodeInput>,
            right: Box<AstNodeInput>,
        },

        #[serde(rename = "mul_operator")]
        #[non_exhaustive]
        MulOperator {
            left: Box<AstNodeInput>,
            right: Box<AstNodeInput>,
        },

        #[serde(rename = "neq_operator")]
        #[non_exhaustive]
        NeqOperator {
            left: Box<AstNodeInput>,
            right: Box<AstNodeInput>,
        },

        #[serde(rename = "null_literal")]
        #[non_exhaustive]
        NullLiteral {},

        #[serde(rename = "number_literal")]
        #[non_exhaustive]
        NumberLiteral {
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers")]
            value: f64,
        },

        #[serde(rename = "or_operator")]
        #[non_exhaustive]
        OrOperator {
            #[serde(default)]
            children: Vec<Box<AstNodeInput>>,
        },

        #[serde(rename = "string_literal")]
        #[non_exhaustive]
        StringLiteral {
            #[serde(default)]
            value: String,
        },

        #[serde(rename = "sub_operator")]
        #[non_exhaustive]
        SubOperator {
            left: Box<AstNodeInput>,
            right: Box<AstNodeInput>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl AstNodeInput {
    pub fn add_operator(left: Box<AstNodeInput>, right: Box<AstNodeInput>) -> Self {
        Self::AddOperator { left, right }
    }

    pub fn and_operator(children: Vec<Box<AstNodeInput>>) -> Self {
        Self::AndOperator { children }
    }

    pub fn boolean_literal(value: bool) -> Self {
        Self::BooleanLiteral { value }
    }

    pub fn conditional_operator(condition: Box<AstNodeInput>, true_expression: Box<AstNodeInput>, false_expression: Box<AstNodeInput>) -> Self {
        Self::ConditionalOperator { condition, true_expression, false_expression }
    }

    pub fn div_operator(left: Box<AstNodeInput>, right: Box<AstNodeInput>) -> Self {
        Self::DivOperator { left, right }
    }

    pub fn dynamic_variable(name: String) -> Self {
        Self::DynamicVariable { name }
    }

    pub fn eq_operator(left: Box<AstNodeInput>, right: Box<AstNodeInput>) -> Self {
        Self::EqOperator { left, right }
    }

    pub fn gt_operator(left: Box<AstNodeInput>, right: Box<AstNodeInput>) -> Self {
        Self::GtOperator { left, right }
    }

    pub fn gte_operator(left: Box<AstNodeInput>, right: Box<AstNodeInput>) -> Self {
        Self::GteOperator { left, right }
    }

    pub fn llm() -> Self {
        Self::Llm { value_schema: None, prompt: None }
    }

    pub fn lt_operator(left: Box<AstNodeInput>, right: Box<AstNodeInput>) -> Self {
        Self::LtOperator { left, right }
    }

    pub fn lte_operator(left: Box<AstNodeInput>, right: Box<AstNodeInput>) -> Self {
        Self::LteOperator { left, right }
    }

    pub fn mul_operator(left: Box<AstNodeInput>, right: Box<AstNodeInput>) -> Self {
        Self::MulOperator { left, right }
    }

    pub fn neq_operator(left: Box<AstNodeInput>, right: Box<AstNodeInput>) -> Self {
        Self::NeqOperator { left, right }
    }

    pub fn null_literal() -> Self {
        Self::NullLiteral {}
    }

    pub fn number_literal(value: f64) -> Self {
        Self::NumberLiteral { value }
    }

    pub fn or_operator(children: Vec<Box<AstNodeInput>>) -> Self {
        Self::OrOperator { children }
    }

    pub fn string_literal(value: String) -> Self {
        Self::StringLiteral { value }
    }

    pub fn sub_operator(left: Box<AstNodeInput>, right: Box<AstNodeInput>) -> Self {
        Self::SubOperator { left, right }
    }

    pub fn llm_with_value_schema(value_schema: LlmLiteralJsonSchemaProperty, prompt: Option<String>) -> Self {
        Self::Llm { value_schema: Some(value_schema), prompt }
    }

    pub fn llm_with_prompt(value_schema: Option<LlmLiteralJsonSchemaProperty>, prompt: String) -> Self {
        Self::Llm { value_schema, prompt: Some(prompt) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
