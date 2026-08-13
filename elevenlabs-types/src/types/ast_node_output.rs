pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum AstNodeOutput {
        #[serde(rename = "add_operator")]
        #[non_exhaustive]
        AddOperator {
            left: Box<AstNodeOutput>,
            right: Box<AstNodeOutput>,
        },

        #[serde(rename = "and_operator")]
        #[non_exhaustive]
        AndOperator {
            #[serde(default)]
            children: Vec<Box<AstNodeOutput>>,
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
            condition: Box<AstNodeOutput>,
            #[serde(rename = "trueExpression")]
            true_expression: Box<AstNodeOutput>,
            #[serde(rename = "falseExpression")]
            false_expression: Box<AstNodeOutput>,
        },

        #[serde(rename = "div_operator")]
        #[non_exhaustive]
        DivOperator {
            left: Box<AstNodeOutput>,
            right: Box<AstNodeOutput>,
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
            left: Box<AstNodeOutput>,
            right: Box<AstNodeOutput>,
        },

        #[serde(rename = "gt_operator")]
        #[non_exhaustive]
        GtOperator {
            left: Box<AstNodeOutput>,
            right: Box<AstNodeOutput>,
        },

        #[serde(rename = "gte_operator")]
        #[non_exhaustive]
        GteOperator {
            left: Box<AstNodeOutput>,
            right: Box<AstNodeOutput>,
        },

        #[serde(rename = "llm")]
        #[non_exhaustive]
        Llm {
            value_schema: LlmLiteralJsonSchemaProperty,
            #[serde(default)]
            prompt: String,
        },

        #[serde(rename = "lt_operator")]
        #[non_exhaustive]
        LtOperator {
            left: Box<AstNodeOutput>,
            right: Box<AstNodeOutput>,
        },

        #[serde(rename = "lte_operator")]
        #[non_exhaustive]
        LteOperator {
            left: Box<AstNodeOutput>,
            right: Box<AstNodeOutput>,
        },

        #[serde(rename = "mul_operator")]
        #[non_exhaustive]
        MulOperator {
            left: Box<AstNodeOutput>,
            right: Box<AstNodeOutput>,
        },

        #[serde(rename = "neq_operator")]
        #[non_exhaustive]
        NeqOperator {
            left: Box<AstNodeOutput>,
            right: Box<AstNodeOutput>,
        },

        #[serde(rename = "null_literal")]
        #[non_exhaustive]
        NullLiteral {},

        #[serde(rename = "number_literal")]
        #[non_exhaustive]
        NumberLiteral {
            value: AstNumberNodeOutputValue,
        },

        #[serde(rename = "or_operator")]
        #[non_exhaustive]
        OrOperator {
            #[serde(default)]
            children: Vec<Box<AstNodeOutput>>,
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
            left: Box<AstNodeOutput>,
            right: Box<AstNodeOutput>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl AstNodeOutput {
    pub fn add_operator(left: Box<AstNodeOutput>, right: Box<AstNodeOutput>) -> Self {
        Self::AddOperator { left, right }
    }

    pub fn and_operator(children: Vec<Box<AstNodeOutput>>) -> Self {
        Self::AndOperator { children }
    }

    pub fn boolean_literal(value: bool) -> Self {
        Self::BooleanLiteral { value }
    }

    pub fn conditional_operator(condition: Box<AstNodeOutput>, true_expression: Box<AstNodeOutput>, false_expression: Box<AstNodeOutput>) -> Self {
        Self::ConditionalOperator { condition, true_expression, false_expression }
    }

    pub fn div_operator(left: Box<AstNodeOutput>, right: Box<AstNodeOutput>) -> Self {
        Self::DivOperator { left, right }
    }

    pub fn dynamic_variable(name: String) -> Self {
        Self::DynamicVariable { name }
    }

    pub fn eq_operator(left: Box<AstNodeOutput>, right: Box<AstNodeOutput>) -> Self {
        Self::EqOperator { left, right }
    }

    pub fn gt_operator(left: Box<AstNodeOutput>, right: Box<AstNodeOutput>) -> Self {
        Self::GtOperator { left, right }
    }

    pub fn gte_operator(left: Box<AstNodeOutput>, right: Box<AstNodeOutput>) -> Self {
        Self::GteOperator { left, right }
    }

    pub fn llm(value_schema: LlmLiteralJsonSchemaProperty, prompt: String) -> Self {
        Self::Llm { value_schema, prompt }
    }

    pub fn lt_operator(left: Box<AstNodeOutput>, right: Box<AstNodeOutput>) -> Self {
        Self::LtOperator { left, right }
    }

    pub fn lte_operator(left: Box<AstNodeOutput>, right: Box<AstNodeOutput>) -> Self {
        Self::LteOperator { left, right }
    }

    pub fn mul_operator(left: Box<AstNodeOutput>, right: Box<AstNodeOutput>) -> Self {
        Self::MulOperator { left, right }
    }

    pub fn neq_operator(left: Box<AstNodeOutput>, right: Box<AstNodeOutput>) -> Self {
        Self::NeqOperator { left, right }
    }

    pub fn null_literal() -> Self {
        Self::NullLiteral {}
    }

    pub fn number_literal(value: AstNumberNodeOutputValue) -> Self {
        Self::NumberLiteral { value }
    }

    pub fn or_operator(children: Vec<Box<AstNodeOutput>>) -> Self {
        Self::OrOperator { children }
    }

    pub fn string_literal(value: String) -> Self {
        Self::StringLiteral { value }
    }

    pub fn sub_operator(left: Box<AstNodeOutput>, right: Box<AstNodeOutput>) -> Self {
        Self::SubOperator { left, right }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
