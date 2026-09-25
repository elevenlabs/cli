pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TemplateRunInput {
        String(String),

        Boolean(bool),

        Integer(i64),

        Double(f64),

        TemplateInputReference(TemplateInputReference),

        TemplateRunInputList(Vec<TemplateRunInput>),
}

impl TemplateRunInput {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }

    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_template_input_reference(&self) -> bool {
        matches!(self, Self::TemplateInputReference(_))
    }

    pub fn is_template_run_input_list(&self) -> bool {
        matches!(self, Self::TemplateRunInputList(_))
    }


    pub fn as_string(&self) -> Option<&str> {
        match self {
                    Self::String(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
                    Self::String(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_boolean(&self) -> Option<&bool> {
        match self {
                    Self::Boolean(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_boolean(self) -> Option<bool> {
        match self {
                    Self::Boolean(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_integer(&self) -> Option<&i64> {
        match self {
                    Self::Integer(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_integer(self) -> Option<i64> {
        match self {
                    Self::Integer(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_double(&self) -> Option<&f64> {
        match self {
                    Self::Double(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_double(self) -> Option<f64> {
        match self {
                    Self::Double(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_template_input_reference(&self) -> Option<&TemplateInputReference> {
        match self {
                    Self::TemplateInputReference(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_template_input_reference(self) -> Option<TemplateInputReference> {
        match self {
                    Self::TemplateInputReference(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_template_run_input_list(&self) -> Option<&Vec<TemplateRunInput>> {
        match self {
                    Self::TemplateRunInputList(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_template_run_input_list(self) -> Option<Vec<TemplateRunInput>> {
        match self {
                    Self::TemplateRunInputList(value) => Some(value),
                    _ => None,
                }
    }
}
