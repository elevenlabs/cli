pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GuardrailType {
    Custom,
    PromptInjection,
    SelfHarmIntent,
    ViolenceGraphic,
    Sexual,
    Violence,
    Harassment,
    SexualMinors,
    SelfHarm,
    SelfHarmInstructions,
    HarassmentThreatening,
    Hate,
    HateThreatening,
    Profanity,
    ReligionOrPolitics,
    MedicalAndLegal,
    Guardrail,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GuardrailType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Custom => serializer.serialize_str("custom"),
            Self::PromptInjection => serializer.serialize_str("prompt_injection"),
            Self::SelfHarmIntent => serializer.serialize_str("self_harm_intent"),
            Self::ViolenceGraphic => serializer.serialize_str("violence_graphic"),
            Self::Sexual => serializer.serialize_str("sexual"),
            Self::Violence => serializer.serialize_str("violence"),
            Self::Harassment => serializer.serialize_str("harassment"),
            Self::SexualMinors => serializer.serialize_str("sexual_minors"),
            Self::SelfHarm => serializer.serialize_str("self_harm"),
            Self::SelfHarmInstructions => serializer.serialize_str("self_harm_instructions"),
            Self::HarassmentThreatening => serializer.serialize_str("harassment_threatening"),
            Self::Hate => serializer.serialize_str("hate"),
            Self::HateThreatening => serializer.serialize_str("hate_threatening"),
            Self::Profanity => serializer.serialize_str("profanity"),
            Self::ReligionOrPolitics => serializer.serialize_str("religion_or_politics"),
            Self::MedicalAndLegal => serializer.serialize_str("medical_and_legal"),
            Self::Guardrail => serializer.serialize_str("guardrail"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GuardrailType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "custom" => Ok(Self::Custom),
            "prompt_injection" => Ok(Self::PromptInjection),
            "self_harm_intent" => Ok(Self::SelfHarmIntent),
            "violence_graphic" => Ok(Self::ViolenceGraphic),
            "sexual" => Ok(Self::Sexual),
            "violence" => Ok(Self::Violence),
            "harassment" => Ok(Self::Harassment),
            "sexual_minors" => Ok(Self::SexualMinors),
            "self_harm" => Ok(Self::SelfHarm),
            "self_harm_instructions" => Ok(Self::SelfHarmInstructions),
            "harassment_threatening" => Ok(Self::HarassmentThreatening),
            "hate" => Ok(Self::Hate),
            "hate_threatening" => Ok(Self::HateThreatening),
            "profanity" => Ok(Self::Profanity),
            "religion_or_politics" => Ok(Self::ReligionOrPolitics),
            "medical_and_legal" => Ok(Self::MedicalAndLegal),
            "guardrail" => Ok(Self::Guardrail),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GuardrailType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom => write!(f, "custom"),
            Self::PromptInjection => write!(f, "prompt_injection"),
            Self::SelfHarmIntent => write!(f, "self_harm_intent"),
            Self::ViolenceGraphic => write!(f, "violence_graphic"),
            Self::Sexual => write!(f, "sexual"),
            Self::Violence => write!(f, "violence"),
            Self::Harassment => write!(f, "harassment"),
            Self::SexualMinors => write!(f, "sexual_minors"),
            Self::SelfHarm => write!(f, "self_harm"),
            Self::SelfHarmInstructions => write!(f, "self_harm_instructions"),
            Self::HarassmentThreatening => write!(f, "harassment_threatening"),
            Self::Hate => write!(f, "hate"),
            Self::HateThreatening => write!(f, "hate_threatening"),
            Self::Profanity => write!(f, "profanity"),
            Self::ReligionOrPolitics => write!(f, "religion_or_politics"),
            Self::MedicalAndLegal => write!(f, "medical_and_legal"),
            Self::Guardrail => write!(f, "guardrail"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
