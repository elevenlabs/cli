pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentDeploymentPercentageStrategy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Traffic percentage to deploy
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub traffic_percentage: f64,
}

impl AgentDeploymentPercentageStrategy {
    pub fn builder() -> AgentDeploymentPercentageStrategyBuilder {
        <AgentDeploymentPercentageStrategyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentDeploymentPercentageStrategyBuilder {
    r#type: Option<String>,
    traffic_percentage: Option<f64>,
}

impl AgentDeploymentPercentageStrategyBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn traffic_percentage(mut self, value: f64) -> Self {
        self.traffic_percentage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentDeploymentPercentageStrategy`].
    /// This method will fail if any of the following fields are not set:
    /// - [`traffic_percentage`](AgentDeploymentPercentageStrategyBuilder::traffic_percentage)
    pub fn build(self) -> Result<AgentDeploymentPercentageStrategy, BuildError> {
        Ok(AgentDeploymentPercentageStrategy {
            r#type: self.r#type,
            traffic_percentage: self.traffic_percentage.ok_or_else(|| BuildError::missing_field("traffic_percentage"))?,
        })
    }
}
