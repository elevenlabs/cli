pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WidgetTextContents {
    /// Call to action displayed inside the compact and full variants.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_label: Option<String>,
    /// Text and ARIA label for the start call button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_call: Option<String>,
    /// Text and ARIA label for the start chat button (text only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_chat: Option<String>,
    /// Text and ARIA label for the new call button. Displayed when the caller already finished at least one call in order ot start the next one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_call: Option<String>,
    /// Text and ARIA label for the end call button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_call: Option<String>,
    /// ARIA label for the mute microphone button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute_microphone: Option<String>,
    /// ARIA label for the change language dropdown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_language: Option<String>,
    /// ARIA label for the collapse button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapse: Option<String>,
    /// ARIA label for the expand button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    /// Text displayed when the user copies a value using the copy button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copied: Option<String>,
    /// Text and ARIA label for the accept terms button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_terms: Option<String>,
    /// Text and ARIA label for the cancel terms button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismiss_terms: Option<String>,
    /// Status displayed when the agent is listening.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listening_status: Option<String>,
    /// Status displayed when the agent is speaking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaking_status: Option<String>,
    /// Status displayed when the agent is connecting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connecting_status: Option<String>,
    /// Status displayed when the agent is chatting (text only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chatting_status: Option<String>,
    /// ARIA label for the text message input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_label: Option<String>,
    /// Placeholder text for the text message input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_placeholder: Option<String>,
    /// Placeholder text for the text message input (text only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_placeholder_text_only: Option<String>,
    /// Placeholder text for the text message input when starting a new conversation (text only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_placeholder_new_conversation: Option<String>,
    /// Information message displayed when the user ends the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ended_conversation: Option<String>,
    /// Information message displayed when the agent ends the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ended_conversation: Option<String>,
    /// Text label used next to the conversation ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    /// Text label used when an error occurs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_occurred: Option<String>,
    /// Text and ARIA label used for the copy ID button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_id: Option<String>,
    /// Text displayed to prompt the user for feedback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiate_feedback: Option<String>,
    /// Text displayed to request additional feedback details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_follow_up_feedback: Option<String>,
    /// Text displayed to thank the user for providing feedback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thanks_for_feedback: Option<String>,
    /// Additional text displayed explaining the value of user feedback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thanks_for_feedback_details: Option<String>,
    /// Placeholder text for the follow-up feedback input field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_up_feedback_placeholder: Option<String>,
    /// Text and ARIA label for the submit button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit: Option<String>,
    /// Text and ARIA label for the go back button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub go_back: Option<String>,
    /// Text and ARIA label for the send message button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_message: Option<String>,
    /// Text and ARIA label for the switch to text mode button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_mode: Option<String>,
    /// Text and ARIA label for the switch to voice mode button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_mode: Option<String>,
    /// Toast notification displayed when switching to text mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switched_to_text_mode: Option<String>,
    /// Toast notification displayed when switching to voice mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switched_to_voice_mode: Option<String>,
    /// Text and ARIA label for the copy button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy: Option<String>,
    /// Text and ARIA label for the download button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download: Option<String>,
    /// Text and ARIA label for the wrap toggle button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrap: Option<String>,
    /// Status text displayed when the agent is processing a tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_working: Option<String>,
    /// Status text displayed when the agent finishes processing a tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_done: Option<String>,
    /// Status text displayed when the agent encounters an error during a tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_error: Option<String>,
    /// Text and ARIA label for the attach file button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_file: Option<String>,
    /// ARIA label for the remove file button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_file: Option<String>,
    /// Error message displayed when a file fails to upload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_upload_error: Option<String>,
    /// Error message displayed when an unsupported file type is selected. Followed by the list of accepted types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type_unsupported: Option<String>,
    /// Error message displayed when a file exceeds the maximum size limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_too_large: Option<String>,
    /// Error message displayed when the maximum number of files for a conversation is reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_limit_reached: Option<String>,
    /// Status text displayed while the agent is typing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typing_indicator: Option<String>,
}

impl WidgetTextContents {
    pub fn builder() -> WidgetTextContentsBuilder {
        <WidgetTextContentsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WidgetTextContentsBuilder {
    main_label: Option<String>,
    start_call: Option<String>,
    start_chat: Option<String>,
    new_call: Option<String>,
    end_call: Option<String>,
    mute_microphone: Option<String>,
    change_language: Option<String>,
    collapse: Option<String>,
    expand: Option<String>,
    copied: Option<String>,
    accept_terms: Option<String>,
    dismiss_terms: Option<String>,
    listening_status: Option<String>,
    speaking_status: Option<String>,
    connecting_status: Option<String>,
    chatting_status: Option<String>,
    input_label: Option<String>,
    input_placeholder: Option<String>,
    input_placeholder_text_only: Option<String>,
    input_placeholder_new_conversation: Option<String>,
    user_ended_conversation: Option<String>,
    agent_ended_conversation: Option<String>,
    conversation_id: Option<String>,
    error_occurred: Option<String>,
    copy_id: Option<String>,
    initiate_feedback: Option<String>,
    request_follow_up_feedback: Option<String>,
    thanks_for_feedback: Option<String>,
    thanks_for_feedback_details: Option<String>,
    follow_up_feedback_placeholder: Option<String>,
    submit: Option<String>,
    go_back: Option<String>,
    send_message: Option<String>,
    text_mode: Option<String>,
    voice_mode: Option<String>,
    switched_to_text_mode: Option<String>,
    switched_to_voice_mode: Option<String>,
    copy: Option<String>,
    download: Option<String>,
    wrap: Option<String>,
    agent_working: Option<String>,
    agent_done: Option<String>,
    agent_error: Option<String>,
    attach_file: Option<String>,
    remove_file: Option<String>,
    file_upload_error: Option<String>,
    file_type_unsupported: Option<String>,
    file_too_large: Option<String>,
    file_limit_reached: Option<String>,
    typing_indicator: Option<String>,
}

impl WidgetTextContentsBuilder {
    pub fn main_label(mut self, value: impl Into<String>) -> Self {
        self.main_label = Some(value.into());
        self
    }

    pub fn start_call(mut self, value: impl Into<String>) -> Self {
        self.start_call = Some(value.into());
        self
    }

    pub fn start_chat(mut self, value: impl Into<String>) -> Self {
        self.start_chat = Some(value.into());
        self
    }

    pub fn new_call(mut self, value: impl Into<String>) -> Self {
        self.new_call = Some(value.into());
        self
    }

    pub fn end_call(mut self, value: impl Into<String>) -> Self {
        self.end_call = Some(value.into());
        self
    }

    pub fn mute_microphone(mut self, value: impl Into<String>) -> Self {
        self.mute_microphone = Some(value.into());
        self
    }

    pub fn change_language(mut self, value: impl Into<String>) -> Self {
        self.change_language = Some(value.into());
        self
    }

    pub fn collapse(mut self, value: impl Into<String>) -> Self {
        self.collapse = Some(value.into());
        self
    }

    pub fn expand(mut self, value: impl Into<String>) -> Self {
        self.expand = Some(value.into());
        self
    }

    pub fn copied(mut self, value: impl Into<String>) -> Self {
        self.copied = Some(value.into());
        self
    }

    pub fn accept_terms(mut self, value: impl Into<String>) -> Self {
        self.accept_terms = Some(value.into());
        self
    }

    pub fn dismiss_terms(mut self, value: impl Into<String>) -> Self {
        self.dismiss_terms = Some(value.into());
        self
    }

    pub fn listening_status(mut self, value: impl Into<String>) -> Self {
        self.listening_status = Some(value.into());
        self
    }

    pub fn speaking_status(mut self, value: impl Into<String>) -> Self {
        self.speaking_status = Some(value.into());
        self
    }

    pub fn connecting_status(mut self, value: impl Into<String>) -> Self {
        self.connecting_status = Some(value.into());
        self
    }

    pub fn chatting_status(mut self, value: impl Into<String>) -> Self {
        self.chatting_status = Some(value.into());
        self
    }

    pub fn input_label(mut self, value: impl Into<String>) -> Self {
        self.input_label = Some(value.into());
        self
    }

    pub fn input_placeholder(mut self, value: impl Into<String>) -> Self {
        self.input_placeholder = Some(value.into());
        self
    }

    pub fn input_placeholder_text_only(mut self, value: impl Into<String>) -> Self {
        self.input_placeholder_text_only = Some(value.into());
        self
    }

    pub fn input_placeholder_new_conversation(mut self, value: impl Into<String>) -> Self {
        self.input_placeholder_new_conversation = Some(value.into());
        self
    }

    pub fn user_ended_conversation(mut self, value: impl Into<String>) -> Self {
        self.user_ended_conversation = Some(value.into());
        self
    }

    pub fn agent_ended_conversation(mut self, value: impl Into<String>) -> Self {
        self.agent_ended_conversation = Some(value.into());
        self
    }

    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn error_occurred(mut self, value: impl Into<String>) -> Self {
        self.error_occurred = Some(value.into());
        self
    }

    pub fn copy_id(mut self, value: impl Into<String>) -> Self {
        self.copy_id = Some(value.into());
        self
    }

    pub fn initiate_feedback(mut self, value: impl Into<String>) -> Self {
        self.initiate_feedback = Some(value.into());
        self
    }

    pub fn request_follow_up_feedback(mut self, value: impl Into<String>) -> Self {
        self.request_follow_up_feedback = Some(value.into());
        self
    }

    pub fn thanks_for_feedback(mut self, value: impl Into<String>) -> Self {
        self.thanks_for_feedback = Some(value.into());
        self
    }

    pub fn thanks_for_feedback_details(mut self, value: impl Into<String>) -> Self {
        self.thanks_for_feedback_details = Some(value.into());
        self
    }

    pub fn follow_up_feedback_placeholder(mut self, value: impl Into<String>) -> Self {
        self.follow_up_feedback_placeholder = Some(value.into());
        self
    }

    pub fn submit(mut self, value: impl Into<String>) -> Self {
        self.submit = Some(value.into());
        self
    }

    pub fn go_back(mut self, value: impl Into<String>) -> Self {
        self.go_back = Some(value.into());
        self
    }

    pub fn send_message(mut self, value: impl Into<String>) -> Self {
        self.send_message = Some(value.into());
        self
    }

    pub fn text_mode(mut self, value: impl Into<String>) -> Self {
        self.text_mode = Some(value.into());
        self
    }

    pub fn voice_mode(mut self, value: impl Into<String>) -> Self {
        self.voice_mode = Some(value.into());
        self
    }

    pub fn switched_to_text_mode(mut self, value: impl Into<String>) -> Self {
        self.switched_to_text_mode = Some(value.into());
        self
    }

    pub fn switched_to_voice_mode(mut self, value: impl Into<String>) -> Self {
        self.switched_to_voice_mode = Some(value.into());
        self
    }

    pub fn copy(mut self, value: impl Into<String>) -> Self {
        self.copy = Some(value.into());
        self
    }

    pub fn download(mut self, value: impl Into<String>) -> Self {
        self.download = Some(value.into());
        self
    }

    pub fn wrap(mut self, value: impl Into<String>) -> Self {
        self.wrap = Some(value.into());
        self
    }

    pub fn agent_working(mut self, value: impl Into<String>) -> Self {
        self.agent_working = Some(value.into());
        self
    }

    pub fn agent_done(mut self, value: impl Into<String>) -> Self {
        self.agent_done = Some(value.into());
        self
    }

    pub fn agent_error(mut self, value: impl Into<String>) -> Self {
        self.agent_error = Some(value.into());
        self
    }

    pub fn attach_file(mut self, value: impl Into<String>) -> Self {
        self.attach_file = Some(value.into());
        self
    }

    pub fn remove_file(mut self, value: impl Into<String>) -> Self {
        self.remove_file = Some(value.into());
        self
    }

    pub fn file_upload_error(mut self, value: impl Into<String>) -> Self {
        self.file_upload_error = Some(value.into());
        self
    }

    pub fn file_type_unsupported(mut self, value: impl Into<String>) -> Self {
        self.file_type_unsupported = Some(value.into());
        self
    }

    pub fn file_too_large(mut self, value: impl Into<String>) -> Self {
        self.file_too_large = Some(value.into());
        self
    }

    pub fn file_limit_reached(mut self, value: impl Into<String>) -> Self {
        self.file_limit_reached = Some(value.into());
        self
    }

    pub fn typing_indicator(mut self, value: impl Into<String>) -> Self {
        self.typing_indicator = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WidgetTextContents`].
    pub fn build(self) -> Result<WidgetTextContents, BuildError> {
        Ok(WidgetTextContents {
            main_label: self.main_label,
            start_call: self.start_call,
            start_chat: self.start_chat,
            new_call: self.new_call,
            end_call: self.end_call,
            mute_microphone: self.mute_microphone,
            change_language: self.change_language,
            collapse: self.collapse,
            expand: self.expand,
            copied: self.copied,
            accept_terms: self.accept_terms,
            dismiss_terms: self.dismiss_terms,
            listening_status: self.listening_status,
            speaking_status: self.speaking_status,
            connecting_status: self.connecting_status,
            chatting_status: self.chatting_status,
            input_label: self.input_label,
            input_placeholder: self.input_placeholder,
            input_placeholder_text_only: self.input_placeholder_text_only,
            input_placeholder_new_conversation: self.input_placeholder_new_conversation,
            user_ended_conversation: self.user_ended_conversation,
            agent_ended_conversation: self.agent_ended_conversation,
            conversation_id: self.conversation_id,
            error_occurred: self.error_occurred,
            copy_id: self.copy_id,
            initiate_feedback: self.initiate_feedback,
            request_follow_up_feedback: self.request_follow_up_feedback,
            thanks_for_feedback: self.thanks_for_feedback,
            thanks_for_feedback_details: self.thanks_for_feedback_details,
            follow_up_feedback_placeholder: self.follow_up_feedback_placeholder,
            submit: self.submit,
            go_back: self.go_back,
            send_message: self.send_message,
            text_mode: self.text_mode,
            voice_mode: self.voice_mode,
            switched_to_text_mode: self.switched_to_text_mode,
            switched_to_voice_mode: self.switched_to_voice_mode,
            copy: self.copy,
            download: self.download,
            wrap: self.wrap,
            agent_working: self.agent_working,
            agent_done: self.agent_done,
            agent_error: self.agent_error,
            attach_file: self.attach_file,
            remove_file: self.remove_file,
            file_upload_error: self.file_upload_error,
            file_type_unsupported: self.file_type_unsupported,
            file_too_large: self.file_too_large,
            file_limit_reached: self.file_limit_reached,
            typing_indicator: self.typing_indicator,
        })
    }
}
