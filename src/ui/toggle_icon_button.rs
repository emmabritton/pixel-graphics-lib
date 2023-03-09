use crate::buffer_graphics_lib::prelude::*;
use crate::buffer_graphics_lib::shapes::polyline::Polyline;
use crate::prelude::styles::IconButtonStyle;
use crate::Timing;
use crate::ui::{ElementState, UiElement};
use crate::ui::tooltip::Tooltip;

#[derive(Debug)]
pub struct IconButton {
    tooltip: Tooltip,
    icon: IndexedImage,
    icon_xy: Coord,
    bounds: Rect,
    border: Polyline,
    shadow: Polyline,
    style: IconButtonStyle,
    state: ElementState
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
        let (w,h) = icon.size();
        let bounds = Rect::new_with_size(xy, w as usize+ style.padding+ style.padding , h as usize+ style.padding+ style.padding);
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
        let tooltip = Tooltip::new(xy + (w,h), tooltip_text, tooltip_positioning, &style.tooltip);
        Self {
            tooltip,
            icon,
            icon_xy: xy + (style.padding,style.padding) + (1,1),
            bounds,
            border,
            shadow,
            style: style.clone(),
            state: ElementState::Normal
        }
    }
}

impl IconButton {
    #[must_use]
    pub fn on_mouse_click(&mut self, xy: Coord) -> bool {
        if self.state != ElementState::Disabled {
            self.bounds.contains(xy)
        } else {
            false
        }
    }
}

impl UiElement for IconButton {
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
        graphics.draw_indexed_image(self.icon_xy, &self.icon);
        if !disabled && hovering {
            self.tooltip.render(graphics, mouse_xy);
        }
    }

    fn update(&mut self, _: &Timing) {

    }

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
