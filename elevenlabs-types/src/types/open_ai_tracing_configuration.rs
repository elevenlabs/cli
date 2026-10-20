pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Only ``group_id`` is acted on; ``workflow_name`` and ``metadata`` are kept so the echoed session matches the request.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OpenAiTracingConfiguration {
    /// Adds conversation to the group (case-insensitive, up to 64 chars).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl OpenAiTracingConfiguration {
    pub fn builder() -> OpenAiTracingConfigurationBuilder {
        <OpenAiTracingConfigurationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OpenAiTracingConfigurationBuilder {
    group_id: Option<String>,
    workflow_name: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
}

impl OpenAiTracingConfigurationBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn workflow_name(mut self, value: impl Into<String>) -> Self {
        self.workflow_name = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OpenAiTracingConfiguration`].
    pub fn build(self) -> Result<OpenAiTracingConfiguration, BuildError> {
        Ok(OpenAiTracingConfiguration {
            group_id: self.group_id,
            workflow_name: self.workflow_name,
            metadata: self.metadata,
        })
    }
}
