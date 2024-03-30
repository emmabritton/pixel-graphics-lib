use crate::prelude::*;
use crate::ui::*;

#[derive(Debug)]
pub struct Label {
    text: Text,
}

impl Label {
    pub fn new(text: Text) -> Self {
        Self { text }
    }

    pub fn full(
        str: &str,
        pos: TextPos,
        color: Color,
        font: PixelFont,
        positioning: Positioning,
        wrapping: WrappingStrategy,
    ) -> Self {
        Self {
            text: Text::new(
                str,
                pos,
                TextFormat::new(wrapping, font, color, positioning),
            ),
        }
    }

    pub fn multiline<P: Into<Coord>>(
        str: &str,
        pos: P,
        color: Color,
        font: PixelFont,
        width_px: usize,
    ) -> Self {
        Self {
            text: Text::new(
                str,
                TextPos::px(pos.into()),
                TextFormat::new(
                    WrappingStrategy::SpaceBeforeCol(font.px_to_cols(width_px)),
                    font,
                    color,
                    Positioning::LeftTop,
                ),
            ),
        }
    }
}

impl UiElement for Label {
    fn set_position(&mut self, top_left: Coord) {
        self.text = self.text.with_pos(TextPos::px(top_left));
    }

    fn bounds(&self) -> &Rect {
        self.text.bounds()
    }

    fn render(&self, graphics: &mut Graphics, _: &MouseData) {
        self.text.render(graphics);
    }

    fn update(&mut self, _: &Timing) {}

    fn set_state(&mut self, _: ElementState) {
        unimplemented!("Label doesn't support state");
    }

    fn get_state(&self) -> ElementState {
        ElementState::Normal
    }
}
