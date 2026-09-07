pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AgentConversationTicketIssueType {
    KnowledgeGap,
    IncorrectInformation,
    DocumentationGap,
    ProductFeedback,
    PlatformBug,
    ToolIssue,
    MissingTool,
    UnnecessaryEscalation,
    WrongAction,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AgentConversationTicketIssueType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::KnowledgeGap => serializer.serialize_str("knowledge_gap"),
            Self::IncorrectInformation => serializer.serialize_str("incorrect_information"),
            Self::DocumentationGap => serializer.serialize_str("documentation_gap"),
            Self::ProductFeedback => serializer.serialize_str("product_feedback"),
            Self::PlatformBug => serializer.serialize_str("platform_bug"),
            Self::ToolIssue => serializer.serialize_str("tool_issue"),
            Self::MissingTool => serializer.serialize_str("missing_tool"),
            Self::UnnecessaryEscalation => serializer.serialize_str("unnecessary_escalation"),
            Self::WrongAction => serializer.serialize_str("wrong_action"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AgentConversationTicketIssueType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "knowledge_gap" => Ok(Self::KnowledgeGap),
            "incorrect_information" => Ok(Self::IncorrectInformation),
            "documentation_gap" => Ok(Self::DocumentationGap),
            "product_feedback" => Ok(Self::ProductFeedback),
            "platform_bug" => Ok(Self::PlatformBug),
            "tool_issue" => Ok(Self::ToolIssue),
            "missing_tool" => Ok(Self::MissingTool),
            "unnecessary_escalation" => Ok(Self::UnnecessaryEscalation),
            "wrong_action" => Ok(Self::WrongAction),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AgentConversationTicketIssueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::KnowledgeGap => write!(f, "knowledge_gap"),
            Self::IncorrectInformation => write!(f, "incorrect_information"),
            Self::DocumentationGap => write!(f, "documentation_gap"),
            Self::ProductFeedback => write!(f, "product_feedback"),
            Self::PlatformBug => write!(f, "platform_bug"),
            Self::ToolIssue => write!(f, "tool_issue"),
            Self::MissingTool => write!(f, "missing_tool"),
            Self::UnnecessaryEscalation => write!(f, "unnecessary_escalation"),
            Self::WrongAction => write!(f, "wrong_action"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
