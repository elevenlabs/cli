pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UsageCharactersResponseModel {
    /// The time axis with unix timestamps for each day.
    #[serde(default)]
    pub time: Vec<i64>,
    /// The usage of each breakdown type along the time axis.
    #[serde(default)]
    pub usage: HashMap<String, Vec<f64>>,
}

impl UsageCharactersResponseModel {
    pub fn builder() -> UsageCharactersResponseModelBuilder {
        <UsageCharactersResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsageCharactersResponseModelBuilder {
    time: Option<Vec<i64>>,
    usage: Option<HashMap<String, Vec<f64>>>,
}

impl UsageCharactersResponseModelBuilder {
    pub fn time(mut self, value: Vec<i64>) -> Self {
        self.time = Some(value);
        self
    }

    pub fn usage(mut self, value: HashMap<String, Vec<f64>>) -> Self {
        self.usage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsageCharactersResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`time`](UsageCharactersResponseModelBuilder::time)
    /// - [`usage`](UsageCharactersResponseModelBuilder::usage)
    pub fn build(self) -> Result<UsageCharactersResponseModel, BuildError> {
        Ok(UsageCharactersResponseModel {
            time: self.time.ok_or_else(|| BuildError::missing_field("time"))?,
            usage: self.usage.ok_or_else(|| BuildError::missing_field("usage"))?,
        })
    }
}
