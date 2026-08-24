pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowFeaturesUsageCommonModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_node: Option<FeatureStatusCommonModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standalone_agent_node: Option<FeatureStatusCommonModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_node: Option<FeatureStatusCommonModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_node: Option<FeatureStatusCommonModel>,
}

impl WorkflowFeaturesUsageCommonModel {
    pub fn builder() -> WorkflowFeaturesUsageCommonModelBuilder {
        <WorkflowFeaturesUsageCommonModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowFeaturesUsageCommonModelBuilder {
    enabled: Option<bool>,
    tool_node: Option<FeatureStatusCommonModel>,
    standalone_agent_node: Option<FeatureStatusCommonModel>,
    phone_number_node: Option<FeatureStatusCommonModel>,
    end_node: Option<FeatureStatusCommonModel>,
}

impl WorkflowFeaturesUsageCommonModelBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn tool_node(mut self, value: FeatureStatusCommonModel) -> Self {
        self.tool_node = Some(value);
        self
    }

    pub fn standalone_agent_node(mut self, value: FeatureStatusCommonModel) -> Self {
        self.standalone_agent_node = Some(value);
        self
    }

    pub fn phone_number_node(mut self, value: FeatureStatusCommonModel) -> Self {
        self.phone_number_node = Some(value);
        self
    }

    pub fn end_node(mut self, value: FeatureStatusCommonModel) -> Self {
        self.end_node = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowFeaturesUsageCommonModel`].
    pub fn build(self) -> Result<WorkflowFeaturesUsageCommonModel, BuildError> {
        Ok(WorkflowFeaturesUsageCommonModel {
            enabled: self.enabled,
            tool_node: self.tool_node,
            standalone_agent_node: self.standalone_agent_node,
            phone_number_node: self.phone_number_node,
            end_node: self.end_node,
        })
    }
}
