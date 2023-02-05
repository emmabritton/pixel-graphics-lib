use crate::buffer_graphics_lib::prelude::*;
use crate::buffer_graphics_lib::shapes::polyline::Polyline;
use crate::buffer_graphics_lib::text::format::Positioning::Center;
use crate::buffer_graphics_lib::text::pos::TextPos;
use crate::buffer_graphics_lib::text::wrapping::WrappingStrategy;
use crate::buffer_graphics_lib::text::Text;
use crate::buffer_graphics_lib::text::TextSize::Normal;
use crate::ui::Ui;

const CLR_TEXT: Color = WHITE;
const CLR_BORDER: Color = LIGHT_GRAY;
const CLR_SHADOW: Color = DARK_GRAY;
const CLR_HOVER: Color = CYAN;
const CLR_SELECTED: Color = WHITE;
const CLR_SELECTED_SHADOW: Color = WHITE;

#[derive(Debug)]
pub struct ToggleButton {
    text: Text,
    bounds: Rect,
    border: Polyline,
    shadow: Polyline,
    selected: bool,
}

impl ToggleButton {
    pub fn new<P: Into<Coord>>(xy: P, text: &'static str, min_width: Option<usize>) -> Self {
        let min_width = min_width.unwrap_or_default();
        let (w, h) = Normal.measure(text, WrappingStrategy::None);
        let bounds = Rect::new_with_size(
            xy,
            ((w as f32 * 1.2) as usize).max(min_width),
            (h as f32 * 2.0) as usize,
        );
        let border = Polyline::rounded_rect(
            bounds.left(),
            bounds.top(),
            bounds.right(),
            bounds.bottom(),
            6,
            CLR_BORDER,
        )
        .unwrap();
        let shadow = Polyline::rounded_rect(
            bounds.left() + 1,
            bounds.top() + 1,
            bounds.right() + 1,
            bounds.bottom() + 1,
            6,
            CLR_SHADOW,
        )
        .unwrap();
        let text = Text::new(
            text,
            TextPos::px(bounds.center() + (0, 1)),
            (CLR_TEXT, Normal, WrappingStrategy::None, Center),
        );
        Self {
            text,
            bounds,
            border,
            shadow,
            selected: false,
        }
    }
}

impl ToggleButton {
    #[must_use]
    pub fn on_mouse_click(&mut self, xy: Coord) -> bool {
        if self.bounds.contains(xy) {
            self.selected = true;
            true
        } else {
            false
        }
    }

    pub fn is_selected(&self) -> bool {
        self.selected
    }

    pub fn set_selected(&mut self, value: bool) {
        self.selected = value;
    }
}

impl Ui for ToggleButton {
    #[must_use]
    fn bounds(&self) -> &Rect {
        &self.bounds
    }

    fn render(&self, graphics: &mut Graphics, mouse_xy: Coord) {
        let border = self.border.with_color(if self.bounds.contains(mouse_xy) {
            CLR_HOVER
        } else if self.selected {
            CLR_SELECTED
        } else {
            CLR_BORDER
        });
        let shadow = self.shadow.with_color(if self.selected {
            CLR_SELECTED_SHADOW
        } else {
            CLR_SHADOW
        });
        graphics.draw(&shadow);
        graphics.draw(&border);
        graphics.draw(&self.text);
    }
}
