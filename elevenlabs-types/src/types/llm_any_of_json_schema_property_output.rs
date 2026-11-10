pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LlmAnyOfJsonSchemaPropertyOutput(pub HashMap<String, serde_json::Value>);