use crate::prelude::*;
use crate::ui::prelude::*;

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

    pub fn singleline<P: Into<Coord>>(
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
                    WrappingStrategy::Cutoff(font.px_to_cols(width_px)),
                    font,
                    color,
                    Positioning::LeftTop,
                ),
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

impl Label {
    pub fn update_text(&mut self, content: &str) {
        self.text = Text::new(content, self.text.pos(), self.text.formatting().clone());
    }
}

impl PixelView for Label {
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

    fn set_state(&mut self, _: ViewState) {
        unimplemented!("Label doesn't support state");
    }

    fn get_state(&self) -> ViewState {
        ViewState::Normal
    }
}

impl LayoutView for Label {
    fn set_bounds(&mut self, bounds: Rect) {
        let lines = self
            .text
            .contents()
            .iter()
            .map(|arr| String::from_utf8_lossy(arr).to_string())
            .collect::<Vec<String>>()
            .join("\n");
        let cols = self.text.formatting().font().px_to_cols(bounds.width());
        let strat = match self.text.formatting().wrapping() {
            WrappingStrategy::None => WrappingStrategy::None,
            WrappingStrategy::SpaceBeforeCol(_) => WrappingStrategy::SpaceBeforeCol(cols),
            WrappingStrategy::AtColWithHyphen(_) => WrappingStrategy::AtColWithHyphen(cols),
            WrappingStrategy::Cutoff(_) => WrappingStrategy::Cutoff(cols),
            WrappingStrategy::Ellipsis(_) => WrappingStrategy::Ellipsis(cols),
            WrappingStrategy::AtCol(_) => WrappingStrategy::AtCol(cols),
        };

        self.text = Text::new(
            &lines,
            self.text.pos(),
            TextFormat::new(
                strat,
                self.text.formatting().font(),
                self.text.formatting().color(),
                self.text.formatting().positioning(),
            ),
        );
    }
}
