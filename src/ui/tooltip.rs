use crate::ui::prelude::*;
use crate::ui::styles::TooltipStyle;
use crate::Timing;
use buffer_graphics_lib::prelude::DrawType::Fill;
use buffer_graphics_lib::prelude::*;

#[derive(Debug)]
pub struct Tooltip {
    label: String,
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
        let bounds = Self::calc_bounds(anchor.into(), positioning, text, style.font);
        let bounds = Rect::new_with_size(
            bounds.top_left(),
            bounds.width() + (style.padding),
            bounds.height() + (style.padding),
        );
        let label = text;
        let (border, shadow, background, text) = Self::layout(&bounds, label, style);
        Self {
            label: label.to_string(),
            text,
            background,
            border,
            shadow,
            style: style.clone(),
        }
    }

    fn layout(
        bounds: &Rect,
        label: &str,
        style: &TooltipStyle,
    ) -> (Polyline, Polyline, Drawable<Rect>, Text) {
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
            label,
            TextPos::px(bounds.top_left() + (style.padding, style.padding)),
            (WHITE, style.font, WrappingStrategy::SpaceBeforeCol(20)),
        );
        (border, shadow, background, text)
    }
}

impl Tooltip {
    pub fn calc_bounds(xy: Coord, positioning: Positioning, text: &str, font: PixelFont) -> Rect {
        let (w, h) = font.measure(&WrappingStrategy::SpaceBeforeCol(20).wrap(text).join("\n"));
        let anchor = positioning.calc((xy.x, xy.y), w, h);
        Rect::new_with_size(Coord::new(anchor.0, anchor.1), w, h)
    }
}

impl UiElement for Tooltip {
    fn set_position(&mut self, top_left: Coord) {
        self.background = self.background.with_move(top_left);
        let (border, shadow, _, text) =
            Self::layout(self.background.obj(), &self.label, &self.style);
        self.border = border;
        self.shadow = shadow;
        self.text = text;
    }

    fn bounds(&self) -> &Rect {
        self.background.obj()
    }

    fn render(&self, graphics: &mut Graphics, _: &MouseData) {
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
