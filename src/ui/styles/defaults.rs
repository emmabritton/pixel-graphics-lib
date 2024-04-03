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
            title_text: TextFormat::new(
                WrappingStrategy::None,
                PixelFont::Standard8x10,
                WHITE,
                LeftTop,
            ),
            body_text: TextFormat::new(
                WrappingStrategy::None,
                PixelFont::Standard6x7,
                WHITE,
                LeftTop,
            ),
            tooltip: Default::default(),
            icon_button: Default::default(),
            toggle_icon_button: Default::default(),
            menu: MenuBarStyle::default(),
        }
    }
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self {
            text: ColorSet::new_values(WHITE, WHITE, WHITE, LIGHT_GRAY),
            font: PixelFont::Standard6x7,
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
            font: PixelFont::Standard6x7,
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
            font: PixelFont::Standard6x7,
            button: ButtonStyle::default(),
            border: Some(LIGHT_GRAY),
            shadow: Some(DARK_GRAY),
            shade: Some(Color::new(0, 0, 0, 127)),
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
            font: PixelFont::Standard4x5,
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

impl Default for MenuBarStyle {
    fn default() -> Self {
        Self {
            background: ColorSet::new_same(BLUE.with_brightness(0.8)),
            border: ColorSet::new_same(WHITE),
            menu_item: MenuItemStyle::default(),
            dropdown_item: DropdownItemStyle::default(),
        }
    }
}

impl Default for MenuItemStyle {
    fn default() -> Self {
        Self {
            background: FocusColorSet::menu(BLUE.with_brightness(0.6), TRANSPARENT, TRANSPARENT),
            text: FocusColorSet::new_values(OFF_WHITE, WHITE, WHITE, LIGHT_GRAY, MID_GRAY),
            font: Default::default(),
            dropdown_background: Some(BLUE.with_brightness(0.6)),
            padding: Padding::new(2, 2, 2, 1),
        }
    }
}

impl Default for DropdownItemStyle {
    fn default() -> Self {
        Self {
            background: FocusColorSet::menu(BLUE.with_brightness(0.5), TRANSPARENT, TRANSPARENT),
            text: FocusColorSet::new_values(OFF_WHITE, WHITE, WHITE, LIGHT_GRAY, MID_GRAY),
            font: PixelFont::default(),
            arrow: DropdownItemStyle::dropdown_arrow_for_font(
                PixelFont::default(),
                FocusColorSet::new_values(OFF_WHITE, LIGHT_GRAY, WHITE, LIGHT_GRAY, MID_GRAY),
            ),
            padding: Padding::new(2, 2, 2, 1),
        }
    }
}

impl DropdownItemStyle {
    fn dropdown_arrow_for_font(font: PixelFont, colors: FocusColorSet) -> IconSet {
        let mut buffer = Graphics::create_buffer(font.size().0, font.size().1);
        let mut graphics =
            Graphics::new(&mut buffer, font.size().0, font.size().1).unwrap_or_else(|err| {
                panic!(
                    "Unable to create graphics using {font:?} when generating menu arrow: {err:?}"
                )
            });

        graphics.draw_triangle(
            Triangle::right_angle(
                (0, 0),
                font.size().0.min(font.size().1),
                AnglePosition::Right,
            )
            .move_center_to(coord!(font.size().0 / 2, font.size().1 / 2)),
            fill(WHITE),
        );
        let icon = graphics
            .copy_to_indexed_image(false)
            .unwrap_or_else(|err| panic!("Unable to create menu arrow icon: {err:?}"));

        let idx = icon.get_palette().iter().position(|c| c == &WHITE).unwrap_or_else(|| panic!("Unable to find color in menubar arrow, please raise an issue on Github emmabritton/pixel-graphics-lib"));

        let recolor = |image: &IndexedImage, color: Color| {
            let mut img = image.clone();
            img.set_color_unchecked(idx as u8, color);
            img
        };

        IconSet {
            normal: Some(recolor(&icon, colors.normal.unwrap_or(WHITE))),
            focused: Some(recolor(
                &icon,
                colors.focused.unwrap_or(colors.normal.unwrap_or(WHITE)),
            )),
            hover: Some(recolor(
                &icon,
                colors.hover.unwrap_or(colors.normal.unwrap_or(WHITE)),
            )),
            error: Some(recolor(
                &icon,
                colors.error.unwrap_or(colors.disabled.unwrap_or(MID_GRAY)),
            )),
            disabled: Some(recolor(
                &icon,
                colors.disabled.unwrap_or(colors.error.unwrap_or(MID_GRAY)),
            )),
        }
    }
}
