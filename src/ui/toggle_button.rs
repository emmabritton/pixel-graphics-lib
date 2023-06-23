use crate::buffer_graphics_lib::prelude::*;
use crate::buffer_graphics_lib::shapes::polyline::Polyline;
use crate::buffer_graphics_lib::text::format::Positioning::Center;
use crate::buffer_graphics_lib::text::pos::TextPos;
use crate::buffer_graphics_lib::text::wrapping::WrappingStrategy;
use crate::buffer_graphics_lib::text::Text;
use crate::ui::prelude::*;
use crate::ui::styles::ToggleButtonStyle;
use crate::Timing;

#[derive(Debug)]
pub struct ToggleButton {
    label: String,
    text: Text,
    bounds: Rect,
    border: Polyline,
    shadow: Polyline,
    selected: bool,
    style: ToggleButtonStyle,
    state: ElementState,
}

impl ToggleButton {
    pub fn new<P: Into<Coord>>(
        xy: P,
        text: &'static str,
        min_width: Option<usize>,
        style: &ToggleButtonStyle,
    ) -> Self {
        let min_width = min_width.unwrap_or_default();
        let (w, h) = TextSize::Normal.measure(text, WrappingStrategy::None);
        let bounds = Rect::new_with_size(
            xy,
            ((w as f32 * 1.2) as usize).max(min_width),
            (h as f32 * 2.0) as usize,
        );
        let label = text;
        let (text, border, shadow) = Self::layout(&bounds, style, label);
        Self {
            label: label.to_string(),
            text,
            bounds,
            border,
            shadow,
            selected: false,
            style: style.clone(),
            state: ElementState::Normal,
        }
    }

    fn layout(bounds: &Rect, style: &ToggleButtonStyle, text: &str) -> (Text, Polyline, Polyline) {
        let border = Polyline::rounded_rect(
            bounds.left(),
            bounds.top(),
            bounds.right(),
            bounds.bottom(),
            style.rounding,
            WHITE,
        )
        .unwrap();
        let shadow = Polyline::rounded_rect(
            bounds.left() + 1,
            bounds.top() + 1,
            bounds.right() + 1,
            bounds.bottom() + 1,
            style.rounding,
            WHITE,
        )
        .unwrap();
        let text = Text::new(
            text,
            TextPos::px(bounds.center() + (0, 1)),
            (WHITE, style.text_size, WrappingStrategy::None, Center),
        );
        (text, border, shadow)
    }
}

impl ToggleButton {
    #[must_use]
    pub fn is_selected(&self) -> bool {
        self.selected
    }

    pub fn set_selected(&mut self, value: bool) {
        self.selected = value;
    }

    #[must_use]
    pub fn on_mouse_click(&mut self, mouse_xy: Coord) -> bool {
        if self.state != ElementState::Disabled && self.bounds.contains(mouse_xy) {
            self.selected = true;
            true
        } else {
            false
        }
    }
}

impl UiElement for ToggleButton {
    fn set_position(&mut self, top_left: Coord) {
        self.bounds = self.bounds.move_to(top_left);
        let (text, border, shadow) = Self::layout(&self.bounds, &self.style, &self.label);
        self.border = border;
        self.shadow = shadow;
        self.text = text;
    }

    #[must_use]
    fn bounds(&self) -> &Rect {
        &self.bounds
    }

    fn render(&self, graphics: &mut Graphics, mouse_xy: Coord) {
        let hovering = self.bounds.contains(mouse_xy);
        let (error, disabled) = self.state.get_err_dis();
        if let Some(color) = self
            .style
            .shadow
            .get(hovering, self.selected, error, disabled)
        {
            self.shadow.with_color(color).render(graphics);
        }
        if let Some(color) = self
            .style
            .border
            .get(hovering, self.selected, error, disabled)
        {
            self.border.with_color(color).render(graphics);
        }
        if let Some(color) = self
            .style
            .text
            .get(hovering, self.selected, error, disabled)
        {
            self.text.with_color(color).render(graphics);
        }
    }

    fn update(&mut self, _: &Timing) {}

    #[inline]
    fn set_state(&mut self, new_state: ElementState) {
        self.state = new_state;
    }

    #[inline]
    fn get_state(&self) -> ElementState {
        self.state
    }
}
