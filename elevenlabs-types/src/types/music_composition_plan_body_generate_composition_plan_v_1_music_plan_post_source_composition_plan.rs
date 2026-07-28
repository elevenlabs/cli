pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum BodyGenerateCompositionPlanV1MusicPlanPostSourceCompositionPlan {
        MusicPrompt(MusicPrompt),

        CompositionPlan(CompositionPlan),
}

impl BodyGenerateCompositionPlanV1MusicPlanPostSourceCompositionPlan {
    pub fn is_music_prompt(&self) -> bool {
        matches!(self, Self::MusicPrompt(_))
    }

    pub fn is_composition_plan(&self) -> bool {
        matches!(self, Self::CompositionPlan(_))
    }


    pub fn as_music_prompt(&self) -> Option<&MusicPrompt> {
        match self {
                    Self::MusicPrompt(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_music_prompt(self) -> Option<MusicPrompt> {
        match self {
                    Self::MusicPrompt(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_composition_plan(&self) -> Option<&CompositionPlan> {
        match self {
                    Self::CompositionPlan(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_composition_plan(self) -> Option<CompositionPlan> {
        match self {
                    Self::CompositionPlan(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for BodyGenerateCompositionPlanV1MusicPlanPostSourceCompositionPlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MusicPrompt(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::CompositionPlan(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
