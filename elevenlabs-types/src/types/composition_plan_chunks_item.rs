pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum CompositionPlanChunksItem {
        GenerationChunkInput(GenerationChunkInput),

        AudioRefChunk(AudioRefChunk),
}

impl CompositionPlanChunksItem {
    pub fn is_generation_chunk_input(&self) -> bool {
        matches!(self, Self::GenerationChunkInput(_))
    }

    pub fn is_audio_ref_chunk(&self) -> bool {
        matches!(self, Self::AudioRefChunk(_))
    }


    pub fn as_generation_chunk_input(&self) -> Option<&GenerationChunkInput> {
        match self {
                    Self::GenerationChunkInput(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_generation_chunk_input(self) -> Option<GenerationChunkInput> {
        match self {
                    Self::GenerationChunkInput(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_audio_ref_chunk(&self) -> Option<&AudioRefChunk> {
        match self {
                    Self::AudioRefChunk(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_audio_ref_chunk(self) -> Option<AudioRefChunk> {
        match self {
                    Self::AudioRefChunk(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for CompositionPlanChunksItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GenerationChunkInput(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::AudioRefChunk(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
