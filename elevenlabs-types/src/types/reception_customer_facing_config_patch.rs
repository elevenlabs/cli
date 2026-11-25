pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Rules, procedures and transfer rules are changed through their own endpoints, not here.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceptionCustomerFacingConfigPatch {
    /// Extra instructions for the agent (e.g., 'Always ask callers to spell out their name')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_instructions: Option<StrippedStr>,
    /// Personality traits for the receptionist, e.g. `['professional', 'concise']`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tone_traits: Option<Vec<StrippedStr>>,
    /// Details the agent should always try to collect from callers (presets + custom labels, e.g. ['full name', 'phone number']).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_to_collect: Option<Vec<StrippedStr>>,
    /// Model the receptionist runs on. Null uses the default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm: Option<Llm>,
    /// Reasoning effort for the receptionist's model, up to `medium` and only among the efforts the model supports. Null uses the model's default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<LlmReasoningEffort>,
    /// When true, the whole knowledge base is given to the receptionist up front instead of searched on each turn. Faster, but only for small knowledge bases; larger ones are still searched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_rag: Option<bool>,
    /// Filter out far-field human speech before transcription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_voice_detection: Option<bool>,
    /// How quickly the receptionist decides the caller has finished speaking. Eager replies soonest; patient waits for a clearer end of turn.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_eagerness: Option<ReceptionTurnEagerness>,
    /// ElevenLabs voice ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
    /// Primary language code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<SupportedLang>,
    /// Additional language codes the agent can communicate in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_languages: Option<Vec<SupportedLang>>,
    /// The first message the agent says when starting a conversation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_message: Option<String>,
    /// How the first message plays. interruptible: callers can talk over it. uninterruptible: it always plays to the end. recording_notice: it opens with a short 'this call may be recorded' notice in the receptionist's language, spoken by a separate voice, and always plays to the end.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_message_mode: Option<FirstMessageMode>,
    /// Allow the end_call system tool during conversations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_end_call_tool: Option<bool>,
    /// Allow the skip_turn system tool during conversations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_skip_turn_tool: Option<bool>,
    /// Allow the play_keypad_touch_tone (DTMF) system tool during conversations. When enabled, the receptionist can play touch-tone digits on a live call to navigate automated phone menus or answer carrier prompts such as a 'press 1 to accept' call-screening step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_play_keypad_touch_tone_tool: Option<bool>,
    /// Show the business's own number, rather than the caller's, as the caller ID on transferred calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_caller_number_on_transfers: Option<bool>,
    /// Keep recording after a call is transferred and add that audio to the call recording.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_after_transfers: Option<bool>,
    /// When enabled, the receptionist may only look up or modify appointments and client records tied to the caller's verified phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_secure_mode: Option<bool>,
    /// When enabled, inbound calls to US numbers are screened before the receptionist answers: calls whose caller ID failed carrier (STIR/SHAKEN) verification are rejected, and unknown callers whose caller ID the carrier could not vouch for must press a key to be connected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_spam_protection: Option<bool>,
}

impl ReceptionCustomerFacingConfigPatch {
    pub fn builder() -> ReceptionCustomerFacingConfigPatchBuilder {
        <ReceptionCustomerFacingConfigPatchBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionCustomerFacingConfigPatchBuilder {
    additional_instructions: Option<StrippedStr>,
    tone_traits: Option<Vec<StrippedStr>>,
    details_to_collect: Option<Vec<StrippedStr>>,
    llm: Option<Llm>,
    reasoning_effort: Option<LlmReasoningEffort>,
    disable_rag: Option<bool>,
    background_voice_detection: Option<bool>,
    turn_eagerness: Option<ReceptionTurnEagerness>,
    voice_id: Option<String>,
    language: Option<SupportedLang>,
    additional_languages: Option<Vec<SupportedLang>>,
    first_message: Option<String>,
    first_message_mode: Option<FirstMessageMode>,
    enable_end_call_tool: Option<bool>,
    enable_skip_turn_tool: Option<bool>,
    enable_play_keypad_touch_tone_tool: Option<bool>,
    hide_caller_number_on_transfers: Option<bool>,
    record_after_transfers: Option<bool>,
    enable_secure_mode: Option<bool>,
    enable_spam_protection: Option<bool>,
}

impl ReceptionCustomerFacingConfigPatchBuilder {
    pub fn additional_instructions(mut self, value: StrippedStr) -> Self {
        self.additional_instructions = Some(value);
        self
    }

    pub fn tone_traits(mut self, value: Vec<StrippedStr>) -> Self {
        self.tone_traits = Some(value);
        self
    }

    pub fn details_to_collect(mut self, value: Vec<StrippedStr>) -> Self {
        self.details_to_collect = Some(value);
        self
    }

    pub fn llm(mut self, value: Llm) -> Self {
        self.llm = Some(value);
        self
    }

    pub fn reasoning_effort(mut self, value: LlmReasoningEffort) -> Self {
        self.reasoning_effort = Some(value);
        self
    }

    pub fn disable_rag(mut self, value: bool) -> Self {
        self.disable_rag = Some(value);
        self
    }

    pub fn background_voice_detection(mut self, value: bool) -> Self {
        self.background_voice_detection = Some(value);
        self
    }

    pub fn turn_eagerness(mut self, value: ReceptionTurnEagerness) -> Self {
        self.turn_eagerness = Some(value);
        self
    }

    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    pub fn language(mut self, value: SupportedLang) -> Self {
        self.language = Some(value);
        self
    }

    pub fn additional_languages(mut self, value: Vec<SupportedLang>) -> Self {
        self.additional_languages = Some(value);
        self
    }

    pub fn first_message(mut self, value: impl Into<String>) -> Self {
        self.first_message = Some(value.into());
        self
    }

    pub fn first_message_mode(mut self, value: FirstMessageMode) -> Self {
        self.first_message_mode = Some(value);
        self
    }

    pub fn enable_end_call_tool(mut self, value: bool) -> Self {
        self.enable_end_call_tool = Some(value);
        self
    }

    pub fn enable_skip_turn_tool(mut self, value: bool) -> Self {
        self.enable_skip_turn_tool = Some(value);
        self
    }

    pub fn enable_play_keypad_touch_tone_tool(mut self, value: bool) -> Self {
        self.enable_play_keypad_touch_tone_tool = Some(value);
        self
    }

    pub fn hide_caller_number_on_transfers(mut self, value: bool) -> Self {
        self.hide_caller_number_on_transfers = Some(value);
        self
    }

    pub fn record_after_transfers(mut self, value: bool) -> Self {
        self.record_after_transfers = Some(value);
        self
    }

    pub fn enable_secure_mode(mut self, value: bool) -> Self {
        self.enable_secure_mode = Some(value);
        self
    }

    pub fn enable_spam_protection(mut self, value: bool) -> Self {
        self.enable_spam_protection = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceptionCustomerFacingConfigPatch`].
    pub fn build(self) -> Result<ReceptionCustomerFacingConfigPatch, BuildError> {
        Ok(ReceptionCustomerFacingConfigPatch {
            additional_instructions: self.additional_instructions,
            tone_traits: self.tone_traits,
            details_to_collect: self.details_to_collect,
            llm: self.llm,
            reasoning_effort: self.reasoning_effort,
            disable_rag: self.disable_rag,
            background_voice_detection: self.background_voice_detection,
            turn_eagerness: self.turn_eagerness,
            voice_id: self.voice_id,
            language: self.language,
            additional_languages: self.additional_languages,
            first_message: self.first_message,
            first_message_mode: self.first_message_mode,
            enable_end_call_tool: self.enable_end_call_tool,
            enable_skip_turn_tool: self.enable_skip_turn_tool,
            enable_play_keypad_touch_tone_tool: self.enable_play_keypad_touch_tone_tool,
            hide_caller_number_on_transfers: self.hide_caller_number_on_transfers,
            record_after_transfers: self.record_after_transfers,
            enable_secure_mode: self.enable_secure_mode,
            enable_spam_protection: self.enable_spam_protection,
        })
    }
}
