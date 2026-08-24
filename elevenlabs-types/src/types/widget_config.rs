pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WidgetConfig {
    /// The variant of the widget
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<EmbedVariant>,
    /// The placement of the widget on the screen
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<WidgetPlacement>,
    /// Whether the widget is expandable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expandable: Option<WidgetExpandable>,
    /// The avatar of the widget
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<WidgetConfigAvatar>,
    /// The feedback mode of the widget
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_mode: Option<WidgetFeedbackMode>,
    /// Configuration for feedback collected at the end of the conversation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_feedback: Option<WidgetEndFeedbackConfig>,
    /// The background color of the widget
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bg_color: Option<String>,
    /// The text color of the widget
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    /// The button color of the widget
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btn_color: Option<String>,
    /// The button text color of the widget
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btn_text_color: Option<String>,
    /// The border color of the widget
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_color: Option<String>,
    /// The focus color of the widget
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_color: Option<String>,
    /// The border radius of the widget
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_radius: Option<i64>,
    /// The button radius of the widget
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btn_radius: Option<i64>,
    /// The action text of the widget
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_text: Option<String>,
    /// The start call text of the widget
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_call_text: Option<String>,
    /// The end call text of the widget
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_call_text: Option<String>,
    /// The expand text of the widget
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_text: Option<String>,
    /// The text to display when the agent is listening
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listening_text: Option<String>,
    /// The text to display when the agent is speaking
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaking_text: Option<String>,
    /// The text to display when sharing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shareable_page_text: Option<String>,
    /// Whether to show terms and conditions on the shareable page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shareable_page_show_terms: Option<bool>,
    /// The text to display for terms and conditions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_text: Option<String>,
    /// The HTML to display for terms and conditions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_html: Option<String>,
    /// The key to display for terms and conditions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_key: Option<String>,
    /// Whether to show the avatar when the widget is collapsed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_avatar_when_collapsed: Option<bool>,
    /// Whether to disable the banner
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_banner: Option<bool>,
    /// The override link for the widget
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_link: Option<String>,
    /// List of allowed hostnames for clickable markdown links. Use { hostname: '*' } to allow any domain. Empty means no links are allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown_link_allowed_hosts: Option<Vec<AllowlistItem>>,
    /// Whether to automatically include www. variants of allowed hosts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown_link_include_www: Option<bool>,
    /// Whether to allow http:// in addition to https:// for allowed hosts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown_link_allow_http: Option<bool>,
    /// Whether to enable mic muting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mic_muting_enabled: Option<bool>,
    /// Whether the widget should show the conversation transcript as it goes on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript_enabled: Option<bool>,
    /// Whether the user should be able to send text messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_input_enabled: Option<bool>,
    /// Whether to enable the conversation mode toggle in the widget
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_mode_toggle_enabled: Option<bool>,
    /// Whether the widget should be expanded by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_expanded: Option<bool>,
    /// Whether the widget should always be expanded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_expanded: Option<bool>,
    /// Whether the widget can be dismissed by the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismissible: Option<bool>,
    /// Whether to show agent working/done/error status during tool use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_agent_status: Option<bool>,
    /// Whether to show the conversation ID after disconnection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_conversation_id: Option<bool>,
    /// Whether to strip audio markup from messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strip_audio_tags: Option<bool>,
    /// Theme for code block syntax highlighting. Defaults to auto-detection by the widget when not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syntax_highlight_theme: Option<WidgetConfigSyntaxHighlightTheme>,
    /// Text contents of the widget
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_contents: Option<WidgetTextContents>,
    /// Styles for the widget
    #[serde(skip_serializing_if = "Option::is_none")]
    pub styles: Option<WidgetStyles>,
    /// Whether to show the resize button
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_resize_button: Option<bool>,
    /// Whether to show the language selector
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_selector: Option<bool>,
    /// Whether the widget can switch to text only mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_text_only: Option<bool>,
    /// The custom avatar path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_avatar_path: Option<String>,
    /// Language presets for the widget
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_presets: Option<HashMap<String, WidgetLanguagePreset>>,
}

impl WidgetConfig {
    pub fn builder() -> WidgetConfigBuilder {
        <WidgetConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WidgetConfigBuilder {
    variant: Option<EmbedVariant>,
    placement: Option<WidgetPlacement>,
    expandable: Option<WidgetExpandable>,
    avatar: Option<WidgetConfigAvatar>,
    feedback_mode: Option<WidgetFeedbackMode>,
    end_feedback: Option<WidgetEndFeedbackConfig>,
    bg_color: Option<String>,
    text_color: Option<String>,
    btn_color: Option<String>,
    btn_text_color: Option<String>,
    border_color: Option<String>,
    focus_color: Option<String>,
    border_radius: Option<i64>,
    btn_radius: Option<i64>,
    action_text: Option<String>,
    start_call_text: Option<String>,
    end_call_text: Option<String>,
    expand_text: Option<String>,
    listening_text: Option<String>,
    speaking_text: Option<String>,
    shareable_page_text: Option<String>,
    shareable_page_show_terms: Option<bool>,
    terms_text: Option<String>,
    terms_html: Option<String>,
    terms_key: Option<String>,
    show_avatar_when_collapsed: Option<bool>,
    disable_banner: Option<bool>,
    override_link: Option<String>,
    markdown_link_allowed_hosts: Option<Vec<AllowlistItem>>,
    markdown_link_include_www: Option<bool>,
    markdown_link_allow_http: Option<bool>,
    mic_muting_enabled: Option<bool>,
    transcript_enabled: Option<bool>,
    text_input_enabled: Option<bool>,
    conversation_mode_toggle_enabled: Option<bool>,
    default_expanded: Option<bool>,
    always_expanded: Option<bool>,
    dismissible: Option<bool>,
    show_agent_status: Option<bool>,
    show_conversation_id: Option<bool>,
    strip_audio_tags: Option<bool>,
    syntax_highlight_theme: Option<WidgetConfigSyntaxHighlightTheme>,
    text_contents: Option<WidgetTextContents>,
    styles: Option<WidgetStyles>,
    show_resize_button: Option<bool>,
    language_selector: Option<bool>,
    supports_text_only: Option<bool>,
    custom_avatar_path: Option<String>,
    language_presets: Option<HashMap<String, WidgetLanguagePreset>>,
}

impl WidgetConfigBuilder {
    pub fn variant(mut self, value: EmbedVariant) -> Self {
        self.variant = Some(value);
        self
    }

    pub fn placement(mut self, value: WidgetPlacement) -> Self {
        self.placement = Some(value);
        self
    }

    pub fn expandable(mut self, value: WidgetExpandable) -> Self {
        self.expandable = Some(value);
        self
    }

    pub fn avatar(mut self, value: WidgetConfigAvatar) -> Self {
        self.avatar = Some(value);
        self
    }

    pub fn feedback_mode(mut self, value: WidgetFeedbackMode) -> Self {
        self.feedback_mode = Some(value);
        self
    }

    pub fn end_feedback(mut self, value: WidgetEndFeedbackConfig) -> Self {
        self.end_feedback = Some(value);
        self
    }

    pub fn bg_color(mut self, value: impl Into<String>) -> Self {
        self.bg_color = Some(value.into());
        self
    }

    pub fn text_color(mut self, value: impl Into<String>) -> Self {
        self.text_color = Some(value.into());
        self
    }

    pub fn btn_color(mut self, value: impl Into<String>) -> Self {
        self.btn_color = Some(value.into());
        self
    }

    pub fn btn_text_color(mut self, value: impl Into<String>) -> Self {
        self.btn_text_color = Some(value.into());
        self
    }

    pub fn border_color(mut self, value: impl Into<String>) -> Self {
        self.border_color = Some(value.into());
        self
    }

    pub fn focus_color(mut self, value: impl Into<String>) -> Self {
        self.focus_color = Some(value.into());
        self
    }

    pub fn border_radius(mut self, value: i64) -> Self {
        self.border_radius = Some(value);
        self
    }

    pub fn btn_radius(mut self, value: i64) -> Self {
        self.btn_radius = Some(value);
        self
    }

    pub fn action_text(mut self, value: impl Into<String>) -> Self {
        self.action_text = Some(value.into());
        self
    }

    pub fn start_call_text(mut self, value: impl Into<String>) -> Self {
        self.start_call_text = Some(value.into());
        self
    }

    pub fn end_call_text(mut self, value: impl Into<String>) -> Self {
        self.end_call_text = Some(value.into());
        self
    }

    pub fn expand_text(mut self, value: impl Into<String>) -> Self {
        self.expand_text = Some(value.into());
        self
    }

    pub fn listening_text(mut self, value: impl Into<String>) -> Self {
        self.listening_text = Some(value.into());
        self
    }

    pub fn speaking_text(mut self, value: impl Into<String>) -> Self {
        self.speaking_text = Some(value.into());
        self
    }

    pub fn shareable_page_text(mut self, value: impl Into<String>) -> Self {
        self.shareable_page_text = Some(value.into());
        self
    }

    pub fn shareable_page_show_terms(mut self, value: bool) -> Self {
        self.shareable_page_show_terms = Some(value);
        self
    }

    pub fn terms_text(mut self, value: impl Into<String>) -> Self {
        self.terms_text = Some(value.into());
        self
    }

    pub fn terms_html(mut self, value: impl Into<String>) -> Self {
        self.terms_html = Some(value.into());
        self
    }

    pub fn terms_key(mut self, value: impl Into<String>) -> Self {
        self.terms_key = Some(value.into());
        self
    }

    pub fn show_avatar_when_collapsed(mut self, value: bool) -> Self {
        self.show_avatar_when_collapsed = Some(value);
        self
    }

    pub fn disable_banner(mut self, value: bool) -> Self {
        self.disable_banner = Some(value);
        self
    }

    pub fn override_link(mut self, value: impl Into<String>) -> Self {
        self.override_link = Some(value.into());
        self
    }

    pub fn markdown_link_allowed_hosts(mut self, value: Vec<AllowlistItem>) -> Self {
        self.markdown_link_allowed_hosts = Some(value);
        self
    }

    pub fn markdown_link_include_www(mut self, value: bool) -> Self {
        self.markdown_link_include_www = Some(value);
        self
    }

    pub fn markdown_link_allow_http(mut self, value: bool) -> Self {
        self.markdown_link_allow_http = Some(value);
        self
    }

    pub fn mic_muting_enabled(mut self, value: bool) -> Self {
        self.mic_muting_enabled = Some(value);
        self
    }

    pub fn transcript_enabled(mut self, value: bool) -> Self {
        self.transcript_enabled = Some(value);
        self
    }

    pub fn text_input_enabled(mut self, value: bool) -> Self {
        self.text_input_enabled = Some(value);
        self
    }

    pub fn conversation_mode_toggle_enabled(mut self, value: bool) -> Self {
        self.conversation_mode_toggle_enabled = Some(value);
        self
    }

    pub fn default_expanded(mut self, value: bool) -> Self {
        self.default_expanded = Some(value);
        self
    }

    pub fn always_expanded(mut self, value: bool) -> Self {
        self.always_expanded = Some(value);
        self
    }

    pub fn dismissible(mut self, value: bool) -> Self {
        self.dismissible = Some(value);
        self
    }

    pub fn show_agent_status(mut self, value: bool) -> Self {
        self.show_agent_status = Some(value);
        self
    }

    pub fn show_conversation_id(mut self, value: bool) -> Self {
        self.show_conversation_id = Some(value);
        self
    }

    pub fn strip_audio_tags(mut self, value: bool) -> Self {
        self.strip_audio_tags = Some(value);
        self
    }

    pub fn syntax_highlight_theme(mut self, value: WidgetConfigSyntaxHighlightTheme) -> Self {
        self.syntax_highlight_theme = Some(value);
        self
    }

    pub fn text_contents(mut self, value: WidgetTextContents) -> Self {
        self.text_contents = Some(value);
        self
    }

    pub fn styles(mut self, value: WidgetStyles) -> Self {
        self.styles = Some(value);
        self
    }

    pub fn show_resize_button(mut self, value: bool) -> Self {
        self.show_resize_button = Some(value);
        self
    }

    pub fn language_selector(mut self, value: bool) -> Self {
        self.language_selector = Some(value);
        self
    }

    pub fn supports_text_only(mut self, value: bool) -> Self {
        self.supports_text_only = Some(value);
        self
    }

    pub fn custom_avatar_path(mut self, value: impl Into<String>) -> Self {
        self.custom_avatar_path = Some(value.into());
        self
    }

    pub fn language_presets(mut self, value: HashMap<String, WidgetLanguagePreset>) -> Self {
        self.language_presets = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WidgetConfig`].
    pub fn build(self) -> Result<WidgetConfig, BuildError> {
        Ok(WidgetConfig {
            variant: self.variant,
            placement: self.placement,
            expandable: self.expandable,
            avatar: self.avatar,
            feedback_mode: self.feedback_mode,
            end_feedback: self.end_feedback,
            bg_color: self.bg_color,
            text_color: self.text_color,
            btn_color: self.btn_color,
            btn_text_color: self.btn_text_color,
            border_color: self.border_color,
            focus_color: self.focus_color,
            border_radius: self.border_radius,
            btn_radius: self.btn_radius,
            action_text: self.action_text,
            start_call_text: self.start_call_text,
            end_call_text: self.end_call_text,
            expand_text: self.expand_text,
            listening_text: self.listening_text,
            speaking_text: self.speaking_text,
            shareable_page_text: self.shareable_page_text,
            shareable_page_show_terms: self.shareable_page_show_terms,
            terms_text: self.terms_text,
            terms_html: self.terms_html,
            terms_key: self.terms_key,
            show_avatar_when_collapsed: self.show_avatar_when_collapsed,
            disable_banner: self.disable_banner,
            override_link: self.override_link,
            markdown_link_allowed_hosts: self.markdown_link_allowed_hosts,
            markdown_link_include_www: self.markdown_link_include_www,
            markdown_link_allow_http: self.markdown_link_allow_http,
            mic_muting_enabled: self.mic_muting_enabled,
            transcript_enabled: self.transcript_enabled,
            text_input_enabled: self.text_input_enabled,
            conversation_mode_toggle_enabled: self.conversation_mode_toggle_enabled,
            default_expanded: self.default_expanded,
            always_expanded: self.always_expanded,
            dismissible: self.dismissible,
            show_agent_status: self.show_agent_status,
            show_conversation_id: self.show_conversation_id,
            strip_audio_tags: self.strip_audio_tags,
            syntax_highlight_theme: self.syntax_highlight_theme,
            text_contents: self.text_contents,
            styles: self.styles,
            show_resize_button: self.show_resize_button,
            language_selector: self.language_selector,
            supports_text_only: self.supports_text_only,
            custom_avatar_path: self.custom_avatar_path,
            language_presets: self.language_presets,
        })
    }
}
