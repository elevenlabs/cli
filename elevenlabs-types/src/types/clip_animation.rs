pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ClipAnimation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enter_effect: Option<ClipAnimationEnterEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enter_duration_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_effect: Option<ClipAnimationExitEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_duration_ms: Option<i64>,
}

impl ClipAnimation {
    pub fn builder() -> ClipAnimationBuilder {
        <ClipAnimationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClipAnimationBuilder {
    enter_effect: Option<ClipAnimationEnterEffect>,
    enter_duration_ms: Option<i64>,
    exit_effect: Option<ClipAnimationExitEffect>,
    exit_duration_ms: Option<i64>,
}

impl ClipAnimationBuilder {
    pub fn enter_effect(mut self, value: ClipAnimationEnterEffect) -> Self {
        self.enter_effect = Some(value);
        self
    }

    pub fn enter_duration_ms(mut self, value: i64) -> Self {
        self.enter_duration_ms = Some(value);
        self
    }

    pub fn exit_effect(mut self, value: ClipAnimationExitEffect) -> Self {
        self.exit_effect = Some(value);
        self
    }

    pub fn exit_duration_ms(mut self, value: i64) -> Self {
        self.exit_duration_ms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClipAnimation`].
    pub fn build(self) -> Result<ClipAnimation, BuildError> {
        Ok(ClipAnimation {
            enter_effect: self.enter_effect,
            enter_duration_ms: self.enter_duration_ms,
            exit_effect: self.exit_effect,
            exit_duration_ms: self.exit_duration_ms,
        })
    }
}
