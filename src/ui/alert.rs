use crate::ui::alert::AlertResult::{Negative, Positive};
use crate::ui::prelude::Positioning::Center;
use crate::ui::prelude::*;
use crate::ui::styles::AlertStyle;

const BUTTON_Y: isize = 28;
const ALERT_SIZE: (usize, usize) = (200, 50);
const ACK_OFFSET: Coord = Coord::new(46, BUTTON_Y);
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
        let min = Button::calc_bounds(
            Coord::default(),
            positive_text,
            None,
            style.button.text_size,
        )
        .width()
        .max(
            Button::calc_bounds(
                Coord::default(),
                negative_text,
                None,
                style.button.text_size,
            )
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
            message: Self::text(message, pos, style.text, style.text_size),
            background,
            bounds,
            style: style.clone(),
        }
    }

    pub fn new_warning(message: &[&str], width: usize, height: usize, style: &AlertStyle) -> Self {
        let pos = Coord::from((width / 2, height / 2)) - Coord::from(ALERT_SIZE) / 2;
        let (bounds, background) = Self::background(style, pos);
        let positive = Button::new(pos + ACK_OFFSET, "OK", Some(25), &style.button);
        Self {
            negative: None,
            positive,
            message: Self::text(message, pos, style.warning_text, style.text_size),
            background,
            bounds,
            style: style.clone(),
        }
    }

    fn text(lines: &[&str], pos: Coord, color: Color, text_size: TextSize) -> Vec<Text> {
        let mut output = vec![];
        for (i, line) in lines.iter().enumerate() {
            output.push(Text::new(
                line,
                TextPos::px(
                    pos + TEXT_POS
                        + (
                            0,
                            i * (text_size.get_size().1 + text_size.get_spacing() * 2),
                        ),
                ),
                (color, text_size, WrappingStrategy::Cutoff(30), Center),
            ));
        }
        output
    }

    fn background(style: &AlertStyle, start: Coord) -> (Rect, ShapeCollection) {
        let rect = Rect::new_with_size(start, ALERT_SIZE.0, ALERT_SIZE.1);
        let mut back = ShapeCollection::new();
        if let Some(color) = style.background {
            InsertShape::insert_above(&mut back, rect.clone(), fill(color));
        }
        if let Some(color) = style.shadow {
            InsertShape::insert_above(&mut back, rect.translate_by((1, 1)), stroke(color));
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
        let size = self.message[0].formatting().size();

        let mut output = vec![];
        for (i, line) in text.iter().enumerate() {
            output.push(Text::new(
                line,
                TextPos::px(pos + TEXT_POS + (0, i * (size.get_size().1 + size.get_spacing() * 2))),
                (color, size, WrappingStrategy::Cutoff(30), Center),
            ));
        }
        self.message = output
    }

    pub fn on_mouse_click(&self, mouse_xy: Coord) -> Option<AlertResult> {
        if self.positive.on_mouse_click(mouse_xy) {
            return Some(Positive);
        }
        if let Some(neg) = &self.negative {
            if neg.on_mouse_click(mouse_xy) {
                return Some(Negative);
            }
        }
        None
    }
}

impl Ui for Alert {
    fn bounds(&self) -> &Rect {
        &self.bounds
    }

    fn render(&self, graphics: &mut Graphics, mouse_xy: Coord) {
        if let Some(color) = self.style.shade {
            graphics.draw_rect(
                Rect::new_with_size((0, 0), graphics.width(), graphics.height()),
                fill(color),
            );
        }
        self.background.render(graphics);
        self.positive.render(graphics, mouse_xy);
        if let Some(neg) = &self.negative {
            neg.render(graphics, mouse_xy);
        }
        for line in &self.message {
            line.render(graphics);
        }
    }
}
