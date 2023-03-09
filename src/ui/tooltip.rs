use crate::prelude::{ElementState, UiElement};
use crate::ui::styles::TooltipStyle;
use crate::Timing;
use buffer_graphics_lib::prelude::DrawType::Fill;
use buffer_graphics_lib::prelude::*;

#[derive(Debug)]
pub struct Tooltip {
    text: Text,
    background: Drawable<Rect>,
    border: Polyline,
    shadow: Polyline,
    style: TooltipStyle,
}

impl Tooltip {
    pub fn new<P: Into<Coord>>(
        anchor: P,
        text: &str,
        positioning: Positioning,
        style: &TooltipStyle,
    ) -> Self {
        let bounds = Self::calc_bounds(anchor.into(), positioning, text, style.size);
        let bounds = Rect::new_with_size(
            bounds.top_left(),
            bounds.width() + (style.padding),
            bounds.height() + (style.padding),
        );
        let border = Polyline::rounded_rect(
            bounds.left(),
            bounds.top(),
            bounds.right(),
            bounds.bottom(),
            0,
            WHITE,
        )
        .unwrap();
        let shadow = Polyline::rounded_rect(
            bounds.left() + 1,
            bounds.top() + 1,
            bounds.right() + 1,
            bounds.bottom() + 1,
            0,
            WHITE,
        )
        .unwrap();
        let background = Drawable::from_obj(
            Rect::new(bounds.top_left(), bounds.bottom_right()),
            Fill(BLACK),
        );
        let text = Text::new(
            text,
            TextPos::px(bounds.top_left() + (style.padding, style.padding)),
            (WHITE, style.size, WrappingStrategy::SpaceBeforeCol(20)),
        );
        Self {
            text,
            background,
            border,
            shadow,
            style: style.clone(),
        }
    }
}

impl Tooltip {
    pub fn calc_bounds(
        xy: Coord,
        positioning: Positioning,
        text: &str,
        text_size: TextSize,
    ) -> Rect {
        let (w, h) = text_size.measure(text, WrappingStrategy::SpaceBeforeCol(20));
        let anchor = positioning.calc((xy.x, xy.y), w, h);
        Rect::new_with_size(Coord::new(anchor.0, anchor.1), w, h)
    }
}

impl UiElement for Tooltip {
    fn bounds(&self) -> &Rect {
        self.background.obj()
    }

    fn render(&self, graphics: &mut Graphics, _: Coord) {
        if let Some(color) = self.style.shadow.get(false, false, false) {
            self.shadow.with_color(color).render(graphics);
        }
        if let Some(color) = self.style.background.get(false, false, false) {
            self.background.with_draw_type(Fill(color)).render(graphics);
        }
        if let Some(color) = self.style.border.get(false, false, false) {
            self.border.with_color(color).render(graphics);
        }
        if let Some(color) = self.style.text.get(false, false, false) {
            self.text.with_color(color).render(graphics);
        }
    }

    fn update(&mut self, _: &Timing) {}

    fn set_state(&mut self, _: ElementState) {
        unimplemented!("Tooltip doesn't have state")
    }

    fn get_state(&self) -> ElementState {
        unimplemented!("Tooltip doesn't have state")
    }
}
