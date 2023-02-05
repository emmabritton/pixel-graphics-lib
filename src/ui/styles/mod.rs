use buffer_graphics_lib::prelude::*;

pub mod defaults;
pub mod impls;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UiStyle {
    pub button: ButtonStyle,
    pub text_field: TextFieldStyle,
    pub toggle_button: ToggleButtonStyle,
    pub alert: AlertStyle,
    pub dialog: DialogStyle,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ColorSet {
    pub normal: Option<Color>,
    pub hover: Option<Color>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ToggleColorSet {
    pub normal: Option<Color>,
    pub hover: Option<Color>,
    pub toggled: Option<Color>,
    pub hover_toggled: Option<Color>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FocusColorSet {
    pub normal: Option<Color>,
    pub hover: Option<Color>,
    pub focused: Option<Color>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TextFieldStyle {
    pub text_color: FocusColorSet,
    pub text_size: TextSize,
    pub background_color: FocusColorSet,
    pub border_color: FocusColorSet,
    pub cursor: FocusColorSet,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ButtonStyle {
    pub text: ColorSet,
    pub text_size: TextSize,
    pub border: ColorSet,
    pub shadow: ColorSet,
    pub rounding: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ToggleButtonStyle {
    pub text: ToggleColorSet,
    pub border: ToggleColorSet,
    pub shadow: ToggleColorSet,
    pub text_size: TextSize,
    pub rounding: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AlertStyle {
    pub background: Option<Color>,
    pub text: Color,
    pub warning_text: Color,
    pub text_size: TextSize,
    pub button: ButtonStyle,
    pub border: Option<Color>,
    pub shadow: Option<Color>,
    pub shade: Option<Color>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DialogStyle {
    pub bounds: Rect,
    pub background: Option<Color>,
    pub text: Color,
    pub button: ButtonStyle,
    pub text_field: TextFieldStyle,
    pub toggle_button: ToggleButtonStyle,
    pub border: Option<Color>,
    pub shadow: Option<Color>,
    pub shade: Option<Color>,
}
