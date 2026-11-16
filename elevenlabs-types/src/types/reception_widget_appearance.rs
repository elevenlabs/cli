pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// How the website widget looks. A null label follows the receptionist's language.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReceptionWidgetAppearance {
    /// First color of the widget's orb gradient, as a hex color.
    #[serde(rename = "orb_color_1")]
    #[serde(default)]
    pub orb_color1: String,
    /// Second color of the widget's orb gradient, as a hex color.
    #[serde(rename = "orb_color_2")]
    #[serde(default)]
    pub orb_color2: String,
    /// Widget size, from the smallest ('tiny') to the largest ('full').
    pub variant: ReceptionWidgetAppearanceVariant,
    /// Where on the page the widget sits.
    pub placement: ReceptionWidgetAppearancePlacement,
    /// Whether visitors can close the widget.
    #[serde(default)]
    pub dismissible: bool,
    /// Text shown on the widget; null uses the default for the receptionist's language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_label: Option<String>,
    /// Text on the button that starts a call; null uses the default for the receptionist's language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_call: Option<String>,
}

impl ReceptionWidgetAppearance {
    pub fn builder() -> ReceptionWidgetAppearanceBuilder {
        <ReceptionWidgetAppearanceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionWidgetAppearanceBuilder {
    orb_color1: Option<String>,
    orb_color2: Option<String>,
    variant: Option<ReceptionWidgetAppearanceVariant>,
    placement: Option<ReceptionWidgetAppearancePlacement>,
    dismissible: Option<bool>,
    main_label: Option<String>,
    start_call: Option<String>,
}

impl ReceptionWidgetAppearanceBuilder {
    pub fn orb_color1(mut self, value: impl Into<String>) -> Self {
        self.orb_color1 = Some(value.into());
        self
    }

    pub fn orb_color2(mut self, value: impl Into<String>) -> Self {
        self.orb_color2 = Some(value.into());
        self
    }

    pub fn variant(mut self, value: ReceptionWidgetAppearanceVariant) -> Self {
        self.variant = Some(value);
        self
    }

    pub fn placement(mut self, value: ReceptionWidgetAppearancePlacement) -> Self {
        self.placement = Some(value);
        self
    }

    pub fn dismissible(mut self, value: bool) -> Self {
        self.dismissible = Some(value);
        self
    }

    pub fn main_label(mut self, value: impl Into<String>) -> Self {
        self.main_label = Some(value.into());
        self
    }

    pub fn start_call(mut self, value: impl Into<String>) -> Self {
        self.start_call = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReceptionWidgetAppearance`].
    /// This method will fail if any of the following fields are not set:
    /// - [`orb_color1`](ReceptionWidgetAppearanceBuilder::orb_color1)
    /// - [`orb_color2`](ReceptionWidgetAppearanceBuilder::orb_color2)
    /// - [`variant`](ReceptionWidgetAppearanceBuilder::variant)
    /// - [`placement`](ReceptionWidgetAppearanceBuilder::placement)
    /// - [`dismissible`](ReceptionWidgetAppearanceBuilder::dismissible)
    pub fn build(self) -> Result<ReceptionWidgetAppearance, BuildError> {
        Ok(ReceptionWidgetAppearance {
            orb_color1: self.orb_color1.ok_or_else(|| BuildError::missing_field("orb_color1"))?,
            orb_color2: self.orb_color2.ok_or_else(|| BuildError::missing_field("orb_color2"))?,
            variant: self.variant.ok_or_else(|| BuildError::missing_field("variant"))?,
            placement: self.placement.ok_or_else(|| BuildError::missing_field("placement"))?,
            dismissible: self.dismissible.ok_or_else(|| BuildError::missing_field("dismissible"))?,
            main_label: self.main_label,
            start_call: self.start_call,
        })
    }
}
