use crate::buffer_graphics_lib::shapes::polyline::Polyline;
use crate::ui::prelude::*;
use crate::ui::tooltip::Tooltip;
use crate::ui::{PixelView, ViewState};
use crate::Timing;

#[derive(Debug)]
pub struct IconButton {
    tooltip: Tooltip,
    icon: IndexedImage,
    icon_xy: Coord,
    bounds: Rect,
    border: Polyline,
    shadow: Polyline,
    style: IconButtonStyle,
    state: ViewState,
    tooltip_text: String,
    tooltip_positioning: Positioning,
}

impl IconButton {
    pub fn new<P: Into<Coord>>(
        xy: P,
        tooltip_text: &str,
        tooltip_positioning: Positioning,
        icon: IndexedImage,
        style: &IconButtonStyle,
    ) -> Self {
        let xy = xy.into();
        let (w, h) = icon.size();
        let bounds = Rect::new_with_size(
            xy,
            w as usize + style.padding + style.padding,
            h as usize + style.padding + style.padding,
        );
        let (icon_xy, border, shadow, tooltip) =
            Self::layout(&bounds, style, tooltip_text, tooltip_positioning, (w, h));
        Self {
            tooltip,
            icon,
            icon_xy,
            bounds,
            border,
            shadow,
            style: style.clone(),
            state: ViewState::Normal,
            tooltip_text: tooltip_text.to_string(),
            tooltip_positioning,
        }
    }

    fn layout(
        bounds: &Rect,
        style: &IconButtonStyle,
        tooltip_text: &str,
        tooltip_positioning: Positioning,
        (w, h): (u8, u8),
    ) -> (Coord, Polyline, Polyline, Tooltip) {
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
        let tooltip = Tooltip::new(
            bounds.top_left() + (w, h),
            tooltip_text,
            tooltip_positioning,
            &style.tooltip,
        );
        (
            bounds.top_left() + (style.padding, style.padding) + (1, 1),
            border,
            shadow,
            tooltip,
        )
    }
}

impl IconButton {
    #[must_use]
    pub fn on_mouse_click(&mut self, down: Coord, up: Coord) -> bool {
        if self.state != ViewState::Disabled {
            self.bounds.contains(down) && self.bounds.contains(up)
        } else {
            false
        }
    }
}

impl PixelView for IconButton {
    fn set_position(&mut self, top_left: Coord) {
        self.bounds = self.bounds.move_to(top_left);
        let (icon_xy, border, shadow, tooltip) = Self::layout(
            &self.bounds,
            &self.style,
            &self.tooltip_text,
            self.tooltip_positioning,
            (self.icon.width(), self.icon.height()),
        );
        self.icon_xy = icon_xy;
        self.border = border;
        self.shadow = shadow;
        self.tooltip = tooltip;
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
        graphics.draw_indexed_image(self.icon_xy, &self.icon);
        if !disabled && hovering {
            self.tooltip.render(graphics, mouse);
        }
    }

    fn update(&mut self, _: &Timing) {}

    #[inline]
    fn set_state(&mut self, state: ViewState) {
        self.state = state;
    }

    #[inline]
    #[must_use]
    fn get_state(&self) -> ViewState {
        self.state
    }
}

impl LayoutView for IconButton {
    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds.clone();
        self.set_position(bounds.top_left());
    }
}
