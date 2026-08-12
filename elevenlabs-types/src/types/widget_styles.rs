pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WidgetStyles {
    /// The base background color.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    /// The color of the base background when hovered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_hover: Option<String>,
    /// The color of the base background when active (clicked).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_active: Option<String>,
    /// The color of the border against the base background.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_border: Option<String>,
    /// The color of subtle text against the base background.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_subtle: Option<String>,
    /// The color of primary text against the base background.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_primary: Option<String>,
    /// The color of error text against the base background.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_error: Option<String>,
    /// The accent background color.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent: Option<String>,
    /// The color of the accent background when hovered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_hover: Option<String>,
    /// The color of the accent background when active (clicked).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_active: Option<String>,
    /// The color of the border against the accent background.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_border: Option<String>,
    /// The color of subtle text against the accent background.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_subtle: Option<String>,
    /// The color of primary text against the accent background.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_primary: Option<String>,
    /// The padding around the edges of the viewport.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub overlay_padding: Option<f64>,
    /// The radius of the buttons.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub button_radius: Option<f64>,
    /// The radius of the input fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub input_radius: Option<f64>,
    /// The radius of the chat bubbles.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub bubble_radius: Option<f64>,
    /// The default radius of sheets.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub sheet_radius: Option<f64>,
    /// The radius of the sheet in compact mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub compact_sheet_radius: Option<f64>,
    /// The radius of the dropdown sheet.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub dropdown_sheet_radius: Option<f64>,
}

impl WidgetStyles {
    pub fn builder() -> WidgetStylesBuilder {
        <WidgetStylesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WidgetStylesBuilder {
    base: Option<String>,
    base_hover: Option<String>,
    base_active: Option<String>,
    base_border: Option<String>,
    base_subtle: Option<String>,
    base_primary: Option<String>,
    base_error: Option<String>,
    accent: Option<String>,
    accent_hover: Option<String>,
    accent_active: Option<String>,
    accent_border: Option<String>,
    accent_subtle: Option<String>,
    accent_primary: Option<String>,
    overlay_padding: Option<f64>,
    button_radius: Option<f64>,
    input_radius: Option<f64>,
    bubble_radius: Option<f64>,
    sheet_radius: Option<f64>,
    compact_sheet_radius: Option<f64>,
    dropdown_sheet_radius: Option<f64>,
}

impl WidgetStylesBuilder {
    pub fn base(mut self, value: impl Into<String>) -> Self {
        self.base = Some(value.into());
        self
    }

    pub fn base_hover(mut self, value: impl Into<String>) -> Self {
        self.base_hover = Some(value.into());
        self
    }

    pub fn base_active(mut self, value: impl Into<String>) -> Self {
        self.base_active = Some(value.into());
        self
    }

    pub fn base_border(mut self, value: impl Into<String>) -> Self {
        self.base_border = Some(value.into());
        self
    }

    pub fn base_subtle(mut self, value: impl Into<String>) -> Self {
        self.base_subtle = Some(value.into());
        self
    }

    pub fn base_primary(mut self, value: impl Into<String>) -> Self {
        self.base_primary = Some(value.into());
        self
    }

    pub fn base_error(mut self, value: impl Into<String>) -> Self {
        self.base_error = Some(value.into());
        self
    }

    pub fn accent(mut self, value: impl Into<String>) -> Self {
        self.accent = Some(value.into());
        self
    }

    pub fn accent_hover(mut self, value: impl Into<String>) -> Self {
        self.accent_hover = Some(value.into());
        self
    }

    pub fn accent_active(mut self, value: impl Into<String>) -> Self {
        self.accent_active = Some(value.into());
        self
    }

    pub fn accent_border(mut self, value: impl Into<String>) -> Self {
        self.accent_border = Some(value.into());
        self
    }

    pub fn accent_subtle(mut self, value: impl Into<String>) -> Self {
        self.accent_subtle = Some(value.into());
        self
    }

    pub fn accent_primary(mut self, value: impl Into<String>) -> Self {
        self.accent_primary = Some(value.into());
        self
    }

    pub fn overlay_padding(mut self, value: f64) -> Self {
        self.overlay_padding = Some(value);
        self
    }

    pub fn button_radius(mut self, value: f64) -> Self {
        self.button_radius = Some(value);
        self
    }

    pub fn input_radius(mut self, value: f64) -> Self {
        self.input_radius = Some(value);
        self
    }

    pub fn bubble_radius(mut self, value: f64) -> Self {
        self.bubble_radius = Some(value);
        self
    }

    pub fn sheet_radius(mut self, value: f64) -> Self {
        self.sheet_radius = Some(value);
        self
    }

    pub fn compact_sheet_radius(mut self, value: f64) -> Self {
        self.compact_sheet_radius = Some(value);
        self
    }

    pub fn dropdown_sheet_radius(mut self, value: f64) -> Self {
        self.dropdown_sheet_radius = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WidgetStyles`].
    pub fn build(self) -> Result<WidgetStyles, BuildError> {
        Ok(WidgetStyles {
            base: self.base,
            base_hover: self.base_hover,
            base_active: self.base_active,
            base_border: self.base_border,
            base_subtle: self.base_subtle,
            base_primary: self.base_primary,
            base_error: self.base_error,
            accent: self.accent,
            accent_hover: self.accent_hover,
            accent_active: self.accent_active,
            accent_border: self.accent_border,
            accent_subtle: self.accent_subtle,
            accent_primary: self.accent_primary,
            overlay_padding: self.overlay_padding,
            button_radius: self.button_radius,
            input_radius: self.input_radius,
            bubble_radius: self.bubble_radius,
            sheet_radius: self.sheet_radius,
            compact_sheet_radius: self.compact_sheet_radius,
            dropdown_sheet_radius: self.dropdown_sheet_radius,
        })
    }
}
