pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EndProcedureToolConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procedures: Option<HashMap<String, EndProcedureToolConfigProceduresValue>>,
}

impl EndProcedureToolConfig {
    pub fn builder() -> EndProcedureToolConfigBuilder {
        <EndProcedureToolConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EndProcedureToolConfigBuilder {
    procedures: Option<HashMap<String, EndProcedureToolConfigProceduresValue>>,
}

impl EndProcedureToolConfigBuilder {
    pub fn procedures(mut self, value: HashMap<String, EndProcedureToolConfigProceduresValue>) -> Self {
        self.procedures = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EndProcedureToolConfig`].
    pub fn build(self) -> Result<EndProcedureToolConfig, BuildError> {
        Ok(EndProcedureToolConfig {
            procedures: self.procedures,
        })
    }
}
