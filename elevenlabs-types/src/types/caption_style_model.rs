pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CaptionStyleModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<CaptionStyleTemplateModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_font: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub text_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_align: Option<CaptionStyleModelTextAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_style: Option<CaptionStyleModelTextStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_weight: Option<CaptionStyleModelTextWeight>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_transform: Option<CaptionStyleModelTextTransform>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_blend_mode: Option<CaptionStyleModelTextBlendMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_shadow: Option<StudioTextStyleShadowModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_outline: Option<StudioTextStyleOutlineModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub background_opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub background_blur: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub background_border_radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_highlights_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_highlights_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_highlights_background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub word_highlights_opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub word_highlights_border_radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub word_highlights_blur: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_animation: Option<CaptionStyleSectionAnimationModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_animation: Option<CaptionStyleWordAnimationModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_animation: Option<CaptionStyleCharacterAnimationModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub width_pct: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_placement: Option<CaptionStyleHorizontalPlacementModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_placement: Option<CaptionStyleVerticalPlacementModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_break_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_lines_per_section: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_words_per_line: Option<i64>,
}

impl CaptionStyleModel {
    pub fn builder() -> CaptionStyleModelBuilder {
        <CaptionStyleModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CaptionStyleModelBuilder {
    template: Option<CaptionStyleTemplateModel>,
    text_font: Option<String>,
    text_scale: Option<f64>,
    text_color: Option<String>,
    text_align: Option<CaptionStyleModelTextAlign>,
    text_style: Option<CaptionStyleModelTextStyle>,
    text_weight: Option<CaptionStyleModelTextWeight>,
    text_transform: Option<CaptionStyleModelTextTransform>,
    text_blend_mode: Option<CaptionStyleModelTextBlendMode>,
    text_shadow: Option<StudioTextStyleShadowModel>,
    text_outline: Option<StudioTextStyleOutlineModel>,
    background_enabled: Option<bool>,
    background_color: Option<String>,
    background_opacity: Option<f64>,
    background_blur: Option<f64>,
    background_border_radius: Option<f64>,
    word_highlights_enabled: Option<bool>,
    word_highlights_color: Option<String>,
    word_highlights_background_color: Option<String>,
    word_highlights_opacity: Option<f64>,
    word_highlights_border_radius: Option<f64>,
    word_highlights_blur: Option<f64>,
    section_animation: Option<CaptionStyleSectionAnimationModel>,
    word_animation: Option<CaptionStyleWordAnimationModel>,
    character_animation: Option<CaptionStyleCharacterAnimationModel>,
    cursor_enabled: Option<bool>,
    width_pct: Option<f64>,
    horizontal_placement: Option<CaptionStyleHorizontalPlacementModel>,
    vertical_placement: Option<CaptionStyleVerticalPlacementModel>,
    auto_break_enabled: Option<bool>,
    max_lines_per_section: Option<i64>,
    max_words_per_line: Option<i64>,
}

impl CaptionStyleModelBuilder {
    pub fn template(mut self, value: CaptionStyleTemplateModel) -> Self {
        self.template = Some(value);
        self
    }

    pub fn text_font(mut self, value: impl Into<String>) -> Self {
        self.text_font = Some(value.into());
        self
    }

    pub fn text_scale(mut self, value: f64) -> Self {
        self.text_scale = Some(value);
        self
    }

    pub fn text_color(mut self, value: impl Into<String>) -> Self {
        self.text_color = Some(value.into());
        self
    }

    pub fn text_align(mut self, value: CaptionStyleModelTextAlign) -> Self {
        self.text_align = Some(value);
        self
    }

    pub fn text_style(mut self, value: CaptionStyleModelTextStyle) -> Self {
        self.text_style = Some(value);
        self
    }

    pub fn text_weight(mut self, value: CaptionStyleModelTextWeight) -> Self {
        self.text_weight = Some(value);
        self
    }

    pub fn text_transform(mut self, value: CaptionStyleModelTextTransform) -> Self {
        self.text_transform = Some(value);
        self
    }

    pub fn text_blend_mode(mut self, value: CaptionStyleModelTextBlendMode) -> Self {
        self.text_blend_mode = Some(value);
        self
    }

    pub fn text_shadow(mut self, value: StudioTextStyleShadowModel) -> Self {
        self.text_shadow = Some(value);
        self
    }

    pub fn text_outline(mut self, value: StudioTextStyleOutlineModel) -> Self {
        self.text_outline = Some(value);
        self
    }

    pub fn background_enabled(mut self, value: bool) -> Self {
        self.background_enabled = Some(value);
        self
    }

    pub fn background_color(mut self, value: impl Into<String>) -> Self {
        self.background_color = Some(value.into());
        self
    }

    pub fn background_opacity(mut self, value: f64) -> Self {
        self.background_opacity = Some(value);
        self
    }

    pub fn background_blur(mut self, value: f64) -> Self {
        self.background_blur = Some(value);
        self
    }

    pub fn background_border_radius(mut self, value: f64) -> Self {
        self.background_border_radius = Some(value);
        self
    }

    pub fn word_highlights_enabled(mut self, value: bool) -> Self {
        self.word_highlights_enabled = Some(value);
        self
    }

    pub fn word_highlights_color(mut self, value: impl Into<String>) -> Self {
        self.word_highlights_color = Some(value.into());
        self
    }

    pub fn word_highlights_background_color(mut self, value: impl Into<String>) -> Self {
        self.word_highlights_background_color = Some(value.into());
        self
    }

    pub fn word_highlights_opacity(mut self, value: f64) -> Self {
        self.word_highlights_opacity = Some(value);
        self
    }

    pub fn word_highlights_border_radius(mut self, value: f64) -> Self {
        self.word_highlights_border_radius = Some(value);
        self
    }

    pub fn word_highlights_blur(mut self, value: f64) -> Self {
        self.word_highlights_blur = Some(value);
        self
    }

    pub fn section_animation(mut self, value: CaptionStyleSectionAnimationModel) -> Self {
        self.section_animation = Some(value);
        self
    }

    pub fn word_animation(mut self, value: CaptionStyleWordAnimationModel) -> Self {
        self.word_animation = Some(value);
        self
    }

    pub fn character_animation(mut self, value: CaptionStyleCharacterAnimationModel) -> Self {
        self.character_animation = Some(value);
        self
    }

    pub fn cursor_enabled(mut self, value: bool) -> Self {
        self.cursor_enabled = Some(value);
        self
    }

    pub fn width_pct(mut self, value: f64) -> Self {
        self.width_pct = Some(value);
        self
    }

    pub fn horizontal_placement(mut self, value: CaptionStyleHorizontalPlacementModel) -> Self {
        self.horizontal_placement = Some(value);
        self
    }

    pub fn vertical_placement(mut self, value: CaptionStyleVerticalPlacementModel) -> Self {
        self.vertical_placement = Some(value);
        self
    }

    pub fn auto_break_enabled(mut self, value: bool) -> Self {
        self.auto_break_enabled = Some(value);
        self
    }

    pub fn max_lines_per_section(mut self, value: i64) -> Self {
        self.max_lines_per_section = Some(value);
        self
    }

    pub fn max_words_per_line(mut self, value: i64) -> Self {
        self.max_words_per_line = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CaptionStyleModel`].
    pub fn build(self) -> Result<CaptionStyleModel, BuildError> {
        Ok(CaptionStyleModel {
            template: self.template,
            text_font: self.text_font,
            text_scale: self.text_scale,
            text_color: self.text_color,
            text_align: self.text_align,
            text_style: self.text_style,
            text_weight: self.text_weight,
            text_transform: self.text_transform,
            text_blend_mode: self.text_blend_mode,
            text_shadow: self.text_shadow,
            text_outline: self.text_outline,
            background_enabled: self.background_enabled,
            background_color: self.background_color,
            background_opacity: self.background_opacity,
            background_blur: self.background_blur,
            background_border_radius: self.background_border_radius,
            word_highlights_enabled: self.word_highlights_enabled,
            word_highlights_color: self.word_highlights_color,
            word_highlights_background_color: self.word_highlights_background_color,
            word_highlights_opacity: self.word_highlights_opacity,
            word_highlights_border_radius: self.word_highlights_border_radius,
            word_highlights_blur: self.word_highlights_blur,
            section_animation: self.section_animation,
            word_animation: self.word_animation,
            character_animation: self.character_animation,
            cursor_enabled: self.cursor_enabled,
            width_pct: self.width_pct,
            horizontal_placement: self.horizontal_placement,
            vertical_placement: self.vertical_placement,
            auto_break_enabled: self.auto_break_enabled,
            max_lines_per_section: self.max_lines_per_section,
            max_words_per_line: self.max_words_per_line,
        })
    }
}
