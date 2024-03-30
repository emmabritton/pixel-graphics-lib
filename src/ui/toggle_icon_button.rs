use crate::prelude::*;
use crate::ui::prelude::LayoutView;
use crate::ui::styles::ToggleIconButtonStyle;
use crate::ui::tooltip::Tooltip;
use crate::ui::{ElementState, UiElement};

#[derive(Debug)]
pub struct ToggleIconButton {
    tooltip: Tooltip,
    icon: IndexedImage,
    icon_xy: Coord,
    bounds: Rect,
    border: Polyline,
    shadow: Polyline,
    style: ToggleIconButtonStyle,
    state: ElementState,
    selected: bool,
    tooltip_text: String,
    tooltip_positioning: Positioning,
}

impl ToggleIconButton {
    pub fn new<P: Into<Coord>>(
        xy: P,
        tooltip_text: &str,
        tooltip_positioning: Positioning,
        icon: IndexedImage,
        style: &ToggleIconButtonStyle,
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
            state: ElementState::Normal,
            selected: false,
            tooltip_text: tooltip_text.to_string(),
            tooltip_positioning,
        }
    }

    fn layout(
        bounds: &Rect,
        style: &ToggleIconButtonStyle,
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

impl ToggleIconButton {
    #[must_use]
    pub fn is_selected(&self) -> bool {
        self.selected
    }

    pub fn set_selected(&mut self, value: bool) {
        self.selected = value;
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

impl UiElement for ToggleIconButton {
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
        graphics.draw_indexed_image(self.icon_xy, &self.icon);
        if !disabled && hovering {
            self.tooltip.render(graphics, mouse);
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

impl LayoutView for ToggleIconButton {
    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds.clone();
        self.set_position(bounds.top_left());
    }
}
