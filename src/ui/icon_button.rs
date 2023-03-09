use winit::event::VirtualKeyCode;
use crate::buffer_graphics_lib::prelude::*;
use crate::buffer_graphics_lib::shapes::polyline::Polyline;
use crate::buffer_graphics_lib::text::format::Positioning::Center;
use crate::buffer_graphics_lib::text::pos::TextPos;
use crate::buffer_graphics_lib::text::wrapping::WrappingStrategy;
use crate::buffer_graphics_lib::text::Text;
use crate::Timing;
use crate::ui::styles::ButtonStyle;
use crate::ui::{ElementState, UiElement};

#[derive(Debug)]
pub struct Button {
    text: Text,
    bounds: Rect,
    border: Polyline,
    shadow: Polyline,
    style: ButtonStyle,
    state: ElementState
}

impl Button {
    pub fn new<P: Into<Coord>>(
        xy: P,
        text: &str,
        min_width: Option<usize>,
        style: &ButtonStyle,
    ) -> Self {
        let bounds = Self::calc_bounds(xy.into(), text, min_width, style.text_size);
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
        Self {
            text,
            bounds,
            border,
            shadow,
            style: style.clone(),
            state: ElementState::Normal
        }
    }

    pub fn calc_bounds(
        xy: Coord,
        text: &str,
        min_width: Option<usize>,
        text_size: TextSize,
    ) -> Rect {
        let min_width = min_width.unwrap_or_default();
        let (w, h) = text_size.measure(text, WrappingStrategy::None);
        Rect::new_with_size(
            xy,
            ((w as f32 * 1.2) as usize).max(min_width),
            (h as f32 * 2.0) as usize,
        )
    }
}

impl UiElement for Button {
    #[must_use]
    fn bounds(&self) -> &Rect {
        &self.bounds
    }

    fn render(&self, graphics: &mut Graphics, mouse_xy: Coord) {
        let (error, disabled) = self.state.get_err_dis();
        let hovering = self.bounds.contains(mouse_xy);
        if let Some(color) = self.style.shadow.get(hovering, error,disabled) {
            self.shadow.with_color(color).render(graphics);
        }
        if let Some(color) = self.style.border.get(hovering, error,disabled) {
            self.border.with_color(color).render(graphics);
        }
        if let Some(color) = self.style.text.get(hovering, error,disabled) {
            self.text.with_color(color).render(graphics);
        }
    }

    #[inline]
    #[must_use]
     fn get_state(&self) -> ElementState {
        self.state
    }

    #[inline]
    fn set_state(&mut self, state: ElementState) {
        self.state = state;
    }

    #[must_use]
    #[inline]
    fn on_mouse_click(&mut self, xy: Coord) -> bool {
        if self.state != ElementState::Disabled {
            self.bounds.contains(xy)
        } else {
            false
        }
    }

    fn update(&mut self, _: &Timing) {

    }

    fn on_key_press(&mut self, _: VirtualKeyCode) {

    }
}