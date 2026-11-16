pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WebhookTypes {
    ProjectConversionStatus,
    ChapterConversionStatus,
    PostCallTranscription,
    PostCallTranscriptionOtel,
    PostCallAudio,
    PostCallUnredactedTranscription,
    PostCallUnredactedTranscriptionOtel,
    PostCallUnredactedAudio,
    PostCallAnalysis,
    CallInitiationFailure,
    ConversationInitiationClientData,
    AnsweringMachineDetection,
    VoiceRemovalNotice,
    VoiceRemovalNoticeWithdrawn,
    VoiceRemoved,
    SpeechToTextTranscription,
    WebhookError,
    SpeechToTextBatch,
    AgentQaEvaluationCompleted,
    ConvaiAlertingNotification,
    FlowsGeneration,
    FlowsTemplateRun,
    DubbingProjectReady,
    DubbingProjectFailed,
    DubbingLanguageCompleted,
    DubbingLanguageFailed,
    SupportTicketMessageAdded,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WebhookTypes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ProjectConversionStatus => serializer.serialize_str("project_conversion_status"),
            Self::ChapterConversionStatus => serializer.serialize_str("chapter_conversion_status"),
            Self::PostCallTranscription => serializer.serialize_str("post_call_transcription"),
            Self::PostCallTranscriptionOtel => serializer.serialize_str("post_call_transcription_otel"),
            Self::PostCallAudio => serializer.serialize_str("post_call_audio"),
            Self::PostCallUnredactedTranscription => serializer.serialize_str("post_call_unredacted_transcription"),
            Self::PostCallUnredactedTranscriptionOtel => serializer.serialize_str("post_call_unredacted_transcription_otel"),
            Self::PostCallUnredactedAudio => serializer.serialize_str("post_call_unredacted_audio"),
            Self::PostCallAnalysis => serializer.serialize_str("post_call_analysis"),
            Self::CallInitiationFailure => serializer.serialize_str("call_initiation_failure"),
            Self::ConversationInitiationClientData => serializer.serialize_str("conversation_initiation_client_data"),
            Self::AnsweringMachineDetection => serializer.serialize_str("answering_machine_detection"),
            Self::VoiceRemovalNotice => serializer.serialize_str("voice_removal_notice"),
            Self::VoiceRemovalNoticeWithdrawn => serializer.serialize_str("voice_removal_notice_withdrawn"),
            Self::VoiceRemoved => serializer.serialize_str("voice_removed"),
            Self::SpeechToTextTranscription => serializer.serialize_str("speech_to_text_transcription"),
            Self::WebhookError => serializer.serialize_str("webhook_error"),
            Self::SpeechToTextBatch => serializer.serialize_str("speech_to_text_batch"),
            Self::AgentQaEvaluationCompleted => serializer.serialize_str("agent_qa_evaluation_completed"),
            Self::ConvaiAlertingNotification => serializer.serialize_str("convai_alerting_notification"),
            Self::FlowsGeneration => serializer.serialize_str("flows_generation"),
            Self::FlowsTemplateRun => serializer.serialize_str("flows_template_run"),
            Self::DubbingProjectReady => serializer.serialize_str("dubbing_project_ready"),
            Self::DubbingProjectFailed => serializer.serialize_str("dubbing_project_failed"),
            Self::DubbingLanguageCompleted => serializer.serialize_str("dubbing_language_completed"),
            Self::DubbingLanguageFailed => serializer.serialize_str("dubbing_language_failed"),
            Self::SupportTicketMessageAdded => serializer.serialize_str("support_ticket_message_added"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WebhookTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "project_conversion_status" => Ok(Self::ProjectConversionStatus),
            "chapter_conversion_status" => Ok(Self::ChapterConversionStatus),
            "post_call_transcription" => Ok(Self::PostCallTranscription),
            "post_call_transcription_otel" => Ok(Self::PostCallTranscriptionOtel),
            "post_call_audio" => Ok(Self::PostCallAudio),
            "post_call_unredacted_transcription" => Ok(Self::PostCallUnredactedTranscription),
            "post_call_unredacted_transcription_otel" => Ok(Self::PostCallUnredactedTranscriptionOtel),
            "post_call_unredacted_audio" => Ok(Self::PostCallUnredactedAudio),
            "post_call_analysis" => Ok(Self::PostCallAnalysis),
            "call_initiation_failure" => Ok(Self::CallInitiationFailure),
            "conversation_initiation_client_data" => Ok(Self::ConversationInitiationClientData),
            "answering_machine_detection" => Ok(Self::AnsweringMachineDetection),
            "voice_removal_notice" => Ok(Self::VoiceRemovalNotice),
            "voice_removal_notice_withdrawn" => Ok(Self::VoiceRemovalNoticeWithdrawn),
            "voice_removed" => Ok(Self::VoiceRemoved),
            "speech_to_text_transcription" => Ok(Self::SpeechToTextTranscription),
            "webhook_error" => Ok(Self::WebhookError),
            "speech_to_text_batch" => Ok(Self::SpeechToTextBatch),
            "agent_qa_evaluation_completed" => Ok(Self::AgentQaEvaluationCompleted),
            "convai_alerting_notification" => Ok(Self::ConvaiAlertingNotification),
            "flows_generation" => Ok(Self::FlowsGeneration),
            "flows_template_run" => Ok(Self::FlowsTemplateRun),
            "dubbing_project_ready" => Ok(Self::DubbingProjectReady),
            "dubbing_project_failed" => Ok(Self::DubbingProjectFailed),
            "dubbing_language_completed" => Ok(Self::DubbingLanguageCompleted),
            "dubbing_language_failed" => Ok(Self::DubbingLanguageFailed),
            "support_ticket_message_added" => Ok(Self::SupportTicketMessageAdded),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WebhookTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProjectConversionStatus => write!(f, "project_conversion_status"),
            Self::ChapterConversionStatus => write!(f, "chapter_conversion_status"),
            Self::PostCallTranscription => write!(f, "post_call_transcription"),
            Self::PostCallTranscriptionOtel => write!(f, "post_call_transcription_otel"),
            Self::PostCallAudio => write!(f, "post_call_audio"),
            Self::PostCallUnredactedTranscription => write!(f, "post_call_unredacted_transcription"),
            Self::PostCallUnredactedTranscriptionOtel => write!(f, "post_call_unredacted_transcription_otel"),
            Self::PostCallUnredactedAudio => write!(f, "post_call_unredacted_audio"),
            Self::PostCallAnalysis => write!(f, "post_call_analysis"),
            Self::CallInitiationFailure => write!(f, "call_initiation_failure"),
            Self::ConversationInitiationClientData => write!(f, "conversation_initiation_client_data"),
            Self::AnsweringMachineDetection => write!(f, "answering_machine_detection"),
            Self::VoiceRemovalNotice => write!(f, "voice_removal_notice"),
            Self::VoiceRemovalNoticeWithdrawn => write!(f, "voice_removal_notice_withdrawn"),
            Self::VoiceRemoved => write!(f, "voice_removed"),
            Self::SpeechToTextTranscription => write!(f, "speech_to_text_transcription"),
            Self::WebhookError => write!(f, "webhook_error"),
            Self::SpeechToTextBatch => write!(f, "speech_to_text_batch"),
            Self::AgentQaEvaluationCompleted => write!(f, "agent_qa_evaluation_completed"),
            Self::ConvaiAlertingNotification => write!(f, "convai_alerting_notification"),
            Self::FlowsGeneration => write!(f, "flows_generation"),
            Self::FlowsTemplateRun => write!(f, "flows_template_run"),
            Self::DubbingProjectReady => write!(f, "dubbing_project_ready"),
            Self::DubbingProjectFailed => write!(f, "dubbing_project_failed"),
            Self::DubbingLanguageCompleted => write!(f, "dubbing_language_completed"),
            Self::DubbingLanguageFailed => write!(f, "dubbing_language_failed"),
            Self::SupportTicketMessageAdded => write!(f, "support_ticket_message_added"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
