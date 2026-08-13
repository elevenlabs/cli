pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WidgetEndFeedbackConfig {
    /// The type of feedback to collect at the end of the conversation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<WidgetEndFeedbackType>,
}

impl WidgetEndFeedbackConfig {
    pub fn builder() -> WidgetEndFeedbackConfigBuilder {
        <WidgetEndFeedbackConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WidgetEndFeedbackConfigBuilder {
    r#type: Option<WidgetEndFeedbackType>,
}

impl WidgetEndFeedbackConfigBuilder {
    pub fn r#type(mut self, value: WidgetEndFeedbackType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WidgetEndFeedbackConfig`].
    pub fn build(self) -> Result<WidgetEndFeedbackConfig, BuildError> {
        Ok(WidgetEndFeedbackConfig {
            r#type: self.r#type,
        })
    }
}
