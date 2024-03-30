use crate::prelude::*;
use crate::ui::layout::LayoutView;
use crate::ui::styles::ButtonStyle;
use crate::ui::{ElementState, UiElement};

#[derive(Debug)]
pub struct Button {
    label: String, //needed for relayout
    text: Text,
    bounds: Rect,
    border: Polyline,
    shadow: Polyline,
    style: ButtonStyle,
    state: ElementState,
}

impl Button {
    pub fn new<P: Into<Coord>>(
        xy: P,
        text: &str,
        min_width: Option<usize>,
        style: &ButtonStyle,
    ) -> Self {
        let bounds = Self::calc_bounds(xy.into(), text, min_width, style.font);
        let label = text.to_string();
        let (text, border, shadow) = Self::layout(&bounds, style, text);
        Self {
            label,
            text,
            bounds,
            border,
            shadow,
            style: style.clone(),
            state: ElementState::Normal,
        }
    }

    fn layout(bounds: &Rect, style: &ButtonStyle, text: &str) -> (Text, Polyline, Polyline) {
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
            (
                WHITE,
                style.font,
                WrappingStrategy::AtCol(style.font.px_to_cols(bounds.width())),
                Positioning::Center,
            ),
        );
        (text, border, shadow)
    }

    pub fn calc_bounds(xy: Coord, text: &str, min_width: Option<usize>, font: PixelFont) -> Rect {
        let min_width = min_width.unwrap_or_default();
        let (w, h) = font.measure(text);
        Rect::new_with_size(
            xy,
            ((w as f32 * 1.2) as usize).max(min_width),
            (h as f32 * 2.0) as usize,
        )
    }

    #[must_use]
    pub fn on_mouse_click(&mut self, down: Coord, up: Coord) -> bool {
        if self.state != ElementState::Disabled {
            self.bounds.contains(down) && self.bounds.contains(up)
        } else {
            false
        }
    }
}

impl UiElement for Button {
    fn set_position(&mut self, top_left: Coord) {
        self.bounds = self.bounds.move_to(top_left);
        let (text, border, shadow) = Self::layout(&self.bounds, &self.style, &self.label);
        self.text = text;
        self.shadow = shadow;
        self.border = border;
    }

    #[must_use]
    fn bounds(&self) -> &Rect {
        &self.bounds
    }

    fn render(&self, graphics: &mut Graphics, mouse: &MouseData) {
        let (error, disabled) = self.state.get_err_dis();
        let hovering = self.bounds.contains(mouse.xy);
        if let Some(color) = self.style.shadow.get(hovering, error, disabled) {
            self.shadow.with_color(color).render(graphics);
        }
        if let Some(color) = self.style.border.get(hovering, error, disabled) {
            self.border.with_color(color).render(graphics);
        }
        if let Some(color) = self.style.text.get(hovering, error, disabled) {
            self.text.with_color(color).render(graphics);
        }
    }

    fn update(&mut self, _: &Timing) {}

    #[inline]
    fn set_state(&mut self, state: ElementState) {
        self.state = state;
    }

    #[inline]
    #[must_use]
    fn get_state(&self) -> ElementState {
        self.state
    }
}

impl LayoutView for Button {
    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds.clone();
        self.set_position(bounds.top_left());
    }
}
