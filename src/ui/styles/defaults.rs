use crate::prelude::MIN_FILE_DIALOG_SIZE;
use crate::ui::prelude::*;
use buffer_graphics_lib::prelude::Positioning::LeftTop;
use buffer_graphics_lib::prelude::*;

impl Default for UiStyle {
    fn default() -> Self {
        Self {
            button: Default::default(),
            text_field: Default::default(),
            toggle_button: Default::default(),
            alert: Default::default(),
            dialog: DialogStyle::new_default_size(MIN_FILE_DIALOG_SIZE),
            background: Color {
                r: 30,
                g: 30,
                b: 140,
                a: 255,
            },
            title_text: TextFormat::new(WrappingStrategy::None, TextSize::Large, WHITE, LeftTop),
            body_text: TextFormat::new(WrappingStrategy::None, TextSize::Normal, WHITE, LeftTop),
            tooltip: Default::default(),
            icon_button: Default::default(),
            toggle_icon_button: Default::default(),
        }
    }
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self {
            text: ColorSet::new_values(WHITE, WHITE, WHITE, LIGHT_GRAY),
            text_size: TextSize::Normal,
            border: ColorSet::new_values(LIGHT_GRAY, CYAN, RED, DARK_GRAY),
            shadow: ColorSet::new_same(DARK_GRAY),
            rounding: 2,
        }
    }
}

impl Default for ToggleButtonStyle {
    fn default() -> Self {
        Self {
            text: ToggleColorSet::new_same(WHITE),
            text_size: TextSize::Normal,
            border: ToggleColorSet::new_values(LIGHT_GRAY, CYAN, WHITE, CYAN, RED, DARK_GRAY),
            shadow: ToggleColorSet::new_values(
                DARK_GRAY, DARK_GRAY, WHITE, WHITE, DARK_GRAY, DARK_GRAY,
            ),
            rounding: 6,
        }
    }
}

impl Default for TextFieldStyle {
    fn default() -> Self {
        Self {
            text_color: FocusColorSet::new_values(BLACK, BLACK, BLACK, RED, DARK_GRAY),
            background_color: FocusColorSet::new_same(WHITE),
            border_color: FocusColorSet::new_values(DARK_GRAY, DARK_GRAY, CYAN, RED, LIGHT_GRAY),
            cursor: FocusColorSet::new_same(BLACK),
        }
    }
}

impl Default for AlertStyle {
    fn default() -> Self {
        Self {
            background: Some(Color {
                r: 20,
                g: 20,
                b: 120,
                a: 255,
            }),
            text: WHITE,
            warning_text: RED,
            text_size: TextSize::Normal,
            button: ButtonStyle::default(),
            border: Some(LIGHT_GRAY),
            shadow: Some(DARK_GRAY),
            shade: Some(Color::new(0., 0., 0., 0.5)),
        }
    }
}

impl DialogStyle {
    pub fn new_default_size(size: (usize, usize)) -> Self {
        let bounds = Rect::new_with_size((0, 0), size.0, size.1);
        Self::new_default(bounds.translate_by(Coord::from(size) / 2))
    }

    pub fn new_default(bounds: Rect) -> Self {
        Self::new(
            bounds,
            Some(Color {
                r: 30,
                g: 30,
                b: 140,
                a: 255,
            }),
            WHITE,
            ButtonStyle::default(),
            TextFieldStyle::default(),
            ToggleButtonStyle::default(),
            Some(LIGHT_GRAY),
            Some(DARK_GRAY),
            Some(Color {
                r: 0,
                g: 0,
                b: 0,
                a: 150,
            }),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        bounds: Rect,
        background: Option<Color>,
        text: Color,
        button: ButtonStyle,
        text_field: TextFieldStyle,
        toggle_button: ToggleButtonStyle,
        border: Option<Color>,
        shadow: Option<Color>,
        shade: Option<Color>,
    ) -> Self {
        Self {
            bounds,
            background,
            text,
            button,
            text_field,
            toggle_button,
            border,
            shadow,
            shade,
        }
    }
}

impl Default for TooltipStyle {
    fn default() -> Self {
        Self {
            text: ColorSet::new_same(WHITE),
            background: ColorSet::new_same(BLACK),
            border: ColorSet::new_same(LIGHT_GRAY),
            shadow: ColorSet::new_same(DARK_GRAY),
            size: TextSize::Small,
            padding: 2,
        }
    }
}

impl Default for IconButtonStyle {
    fn default() -> Self {
        Self {
            tooltip: TooltipStyle::default(),
            border: ColorSet::new_values(LIGHT_GRAY, CYAN, RED, DARK_GRAY),
            shadow: ColorSet::new_same(DARK_GRAY),
            rounding: 2,
            padding: 4,
        }
    }
}

impl Default for ToggleIconButtonStyle {
    fn default() -> Self {
        Self {
            tooltip: TooltipStyle::default(),
            border: ToggleColorSet::new_values(LIGHT_GRAY, CYAN, WHITE, CYAN, RED, DARK_GRAY),
            shadow: ToggleColorSet::new_values(
                DARK_GRAY, DARK_GRAY, WHITE, WHITE, DARK_GRAY, DARK_GRAY,
            ),
            rounding: 6,
            padding: 4,
        }
    }
}
