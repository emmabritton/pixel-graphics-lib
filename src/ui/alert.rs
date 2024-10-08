use crate::ui::alert::AlertResult::{Negative, Positive};
use crate::ui::prelude::*;
use crate::ui::styles::AlertStyle;
use buffer_graphics_lib::prelude::Positioning::Center;
use buffer_graphics_lib::prelude::*;

const BUTTON_Y: isize = 28;
const ALERT_SIZE: (usize, usize) = (200, 50);
const ACK_OFFSET: Coord = Coord::new(90, BUTTON_Y);
const NEGATIVE_OFFSET: Coord = Coord::new(6, BUTTON_Y);
const TEXT_POS: Coord = Coord::new(ALERT_SIZE.0 as isize / 2, 10);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AlertResult {
    Positive,
    Negative,
}

#[derive(Debug)]
pub struct Alert {
    negative: Option<Button>,
    positive: Button,
    message: Vec<Text>,
    background: ShapeCollection,
    bounds: Rect,
    style: AlertStyle,
}

impl Alert {
    ///
    /// # Parameters
    /// * `width` - Screen width
    /// * `height` - Screen height
    pub fn new_question(
        message: &[&str],
        negative_text: &str,
        positive_text: &str,
        width: usize,
        height: usize,
        style: &AlertStyle,
    ) -> Self {
        let mut message = message;
        let pos = Coord::from((width / 2, height / 2)) - Coord::from(ALERT_SIZE) / 2;
        let (bounds, background) = Self::background(style, pos);
        let min = Button::calc_bounds(Coord::default(), positive_text, None, style.button.font)
            .width()
            .max(
                Button::calc_bounds(Coord::default(), negative_text, None, style.button.font)
                    .width(),
            );
        let positive = Button::new(
            pos + (ALERT_SIZE.0 - min - 6, BUTTON_Y as usize),
            positive_text,
            Some(min),
            &style.button,
        );
        let negative = Button::new(
            pos + NEGATIVE_OFFSET,
            negative_text,
            Some(min),
            &style.button,
        );
        if message.is_empty() {
            message = &[""];
        }
        Self {
            negative: Some(negative),
            positive,
            message: Self::text(message, pos, style.text, style.font),
            background,
            bounds,
            style: style.clone(),
        }
    }

    ///
    /// # Parameters
    /// * `width` - Screen width
    /// * `height` - Screen height
    pub fn new_warning(message: &[&str], width: usize, height: usize, style: &AlertStyle) -> Self {
        let pos = Coord::from((width / 2, height / 2)) - Coord::from(ALERT_SIZE) / 2;
        let (bounds, background) = Self::background(style, pos);
        let positive = Button::new(pos + ACK_OFFSET, "OK", Some(20), &style.button);
        Self {
            negative: None,
            positive,
            message: Self::text(message, pos, style.warning_text, style.font),
            background,
            bounds,
            style: style.clone(),
        }
    }

    fn text(lines: &[&str], pos: Coord, color: Color, font: PixelFont) -> Vec<Text> {
        let mut output = vec![];
        for (i, line) in lines.iter().enumerate() {
            output.push(Text::new(
                line,
                TextPos::px(pos + TEXT_POS + (0, i * (font.size().1 + font.spacing() * 2))),
                (color, font, WrappingStrategy::Cutoff(30), Center),
            ));
        }
        output
    }

    fn background(style: &AlertStyle, start: Coord) -> (Rect, ShapeCollection) {
        let rect = Rect::new_with_size(start, ALERT_SIZE.0, ALERT_SIZE.1);
        let mut back = ShapeCollection::default();
        if let Some(color) = style.background {
            InsertShape::insert_above(&mut back, rect.clone(), fill(color));
        }
        if let Some(color) = style.shadow {
            InsertShape::insert_above(&mut back, rect.translate_by(coord!(1, 1)), stroke(color));
        }
        if let Some(color) = style.border {
            InsertShape::insert_above(&mut back, rect.clone(), stroke(color));
        }

        (rect, back)
    }
}

impl Alert {
    pub fn change_text(&mut self, text: &[&str]) {
        let pos = self.bounds.top_left();
        let color = self.message[0].formatting().color();
        let font = self.message[0].formatting().font();

        let mut output = vec![];
        for (i, line) in text.iter().enumerate() {
            output.push(Text::new(
                line,
                TextPos::px(pos + TEXT_POS + (0, i * (font.size().1 + font.spacing() * 2))),
                (color, font, WrappingStrategy::Cutoff(30), Center),
            ));
        }
        self.message = output
    }

    #[must_use]
    pub fn on_mouse_click(&mut self, down: Coord, up: Coord) -> Option<AlertResult> {
        if self.positive.on_mouse_click(down, up) {
            return Some(Positive);
        }
        if let Some(neg) = &mut self.negative {
            if neg.on_mouse_click(down, up) {
                return Some(Negative);
            }
        }
        None
    }

    pub fn render(&self, graphics: &mut Graphics, mouse: &MouseData) {
        if let Some(color) = self.style.shade {
            graphics.draw_rect(
                Rect::new_with_size((0, 0), graphics.width(), graphics.height()),
                fill(color),
            );
        }
        self.background.render(graphics);
        self.positive.render(graphics, mouse);
        if let Some(neg) = &self.negative {
            neg.render(graphics, mouse);
        }
        for line in &self.message {
            line.render(graphics);
        }
    }
}
