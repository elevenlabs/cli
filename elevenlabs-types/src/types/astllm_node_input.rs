pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum AstllmNodeInput {
        AstllmNodeInputValueSchema(AstllmNodeInputValueSchema),

        AstllmNodeInputPrompt(AstllmNodeInputPrompt),
}

impl AstllmNodeInput {
    pub fn is_astllm_node_input_value_schema(&self) -> bool {
        matches!(self, Self::AstllmNodeInputValueSchema(_))
    }

    pub fn is_astllm_node_input_prompt(&self) -> bool {
        matches!(self, Self::AstllmNodeInputPrompt(_))
    }


    pub fn as_astllm_node_input_value_schema(&self) -> Option<&AstllmNodeInputValueSchema> {
        match self {
                    Self::AstllmNodeInputValueSchema(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_astllm_node_input_value_schema(self) -> Option<AstllmNodeInputValueSchema> {
        match self {
                    Self::AstllmNodeInputValueSchema(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_astllm_node_input_prompt(&self) -> Option<&AstllmNodeInputPrompt> {
        match self {
                    Self::AstllmNodeInputPrompt(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_astllm_node_input_prompt(self) -> Option<AstllmNodeInputPrompt> {
        match self {
                    Self::AstllmNodeInputPrompt(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for AstllmNodeInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AstllmNodeInputValueSchema(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::AstllmNodeInputPrompt(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
