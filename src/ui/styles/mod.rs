use buffer_graphics_lib::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod defaults;
pub mod impls;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct UiStyle {
    pub button: ButtonStyle,
    pub text_field: TextFieldStyle,
    pub toggle_button: ToggleButtonStyle,
    pub alert: AlertStyle,
    pub dialog: DialogStyle,
    pub background: Color,
    pub title_text: TextFormat,
    pub body_text: TextFormat,
    pub tooltip: TooltipStyle,
    pub icon_button: IconButtonStyle,
    pub checkbox: CheckboxStyle,
    pub toggle_icon_button: ToggleIconButtonStyle,
    pub menu: MenuBarStyle,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ColorSet {
    pub normal: Option<Color>,
    pub hover: Option<Color>,
    pub error: Option<Color>,
    pub disabled: Option<Color>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ToggleColorSet {
    pub normal: Option<Color>,
    pub hover: Option<Color>,
    pub toggled: Option<Color>,
    pub hover_toggled: Option<Color>,
    pub error: Option<Color>,
    pub disabled: Option<Color>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FocusColorSet {
    pub normal: Option<Color>,
    pub hover: Option<Color>,
    pub focused: Option<Color>,
    pub error: Option<Color>,
    pub disabled: Option<Color>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TextFieldStyle {
    pub text_color: FocusColorSet,
    pub background_color: FocusColorSet,
    pub border_color: FocusColorSet,
    pub cursor: FocusColorSet,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ButtonStyle {
    pub text: ColorSet,
    pub font: PixelFont,
    pub border: ColorSet,
    pub shadow: ColorSet,
    pub rounding: usize,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MenuBarStyle {
    pub background: ColorSet,
    pub border: ColorSet,
    pub menu_item: MenuItemStyle,
    pub dropdown_item: DropdownItemStyle,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MenuItemStyle {
    pub background: FocusColorSet,
    pub text: FocusColorSet,
    pub font: PixelFont,
    pub dropdown_background: Option<Color>,
    pub padding: Padding,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DropdownItemStyle {
    pub background: FocusColorSet,
    pub text: FocusColorSet,
    pub font: PixelFont,
    pub arrow: IconSet,
    pub padding: Padding,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Padding {
    left: usize,
    top: usize,
    right: usize,
    bottom: usize,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IconButtonStyle {
    pub tooltip: TooltipStyle,
    pub border: ColorSet,
    pub shadow: ColorSet,
    pub rounding: usize,
    pub padding: usize,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CheckboxStyle {
    pub checked_icon: IndexedImage,
    pub check_box: IndexedImage,
    pub text: ColorSet,
    pub icon: ColorSet,
    pub font: PixelFont,
    pub spacing: usize,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TooltipStyle {
    pub text: ColorSet,
    pub background: ColorSet,
    pub border: ColorSet,
    pub shadow: ColorSet,
    pub font: PixelFont,
    pub padding: usize,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ToggleButtonStyle {
    pub text: ToggleColorSet,
    pub border: ToggleColorSet,
    pub shadow: ToggleColorSet,
    pub font: PixelFont,
    pub rounding: usize,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ToggleIconButtonStyle {
    pub tooltip: TooltipStyle,
    pub border: ToggleColorSet,
    pub shadow: ToggleColorSet,
    pub rounding: usize,
    pub padding: usize,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlertStyle {
    pub background: Option<Color>,
    pub text: Color,
    pub warning_text: Color,
    pub font: PixelFont,
    pub button: ButtonStyle,
    pub border: Option<Color>,
    pub shadow: Option<Color>,
    pub shade: Option<Color>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DialogStyle {
    pub bounds: Rect,
    pub background: Option<Color>,
    pub text: Color,
    pub border: Option<Color>,
    pub shadow: Option<Color>,
    pub shade: Option<Color>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IconSet {
    pub normal: Option<IndexedImage>,
    pub focused: Option<IndexedImage>,
    pub hover: Option<IndexedImage>,
    pub error: Option<IndexedImage>,
    pub disabled: Option<IndexedImage>,
}
