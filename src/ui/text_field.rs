use crate::buffer_graphics_lib::prelude::Positioning::LeftCenter;
use crate::prelude::*;
use crate::ui::prelude::WrappingStrategy::Cutoff;
use crate::ui::styles::TextFieldStyle;
use crate::ui::Ui;
use crate::utilities::key_code_to_char;

const CURSOR_BLINK_RATE: f64 = 0.5;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TextFilter {
    /// a-z
    Letters,
    /// 0-9
    Numbers,
    /// 0-9 a-f
    Hex,
    /// 0-9, -
    NegativeNumbers,
    /// 0-9, -, .
    Decimal,
    /// !@$ etc
    Symbols,
    /// Space
    Whitespace,
    /// Letters and _-().
    Filename,
    /// Any char
    All,
}

impl TextFilter {
    pub fn is_char_allowed(&self, chr: char) -> bool {
        match self {
            TextFilter::Letters => ('a'..='z').contains(&chr),
            TextFilter::Numbers => ('0'..='9').contains(&chr),
            TextFilter::Hex => ('0'..='9').contains(&chr) || ('a'..='f').contains(&chr),
            TextFilter::NegativeNumbers => ('0'..='9').contains(&chr) || chr == '-',
            TextFilter::Decimal => ('0'..='9').contains(&chr) || chr == '-' || chr == '.',
            TextFilter::Symbols => SUPPORTED_SYMBOLS.contains(&chr),
            TextFilter::Whitespace => chr == ' ',
            TextFilter::Filename => {
                ('a'..='z').contains(&chr)
                    || ('0'..='9').contains(&chr)
                    || ['(', ')', '-', '.', '_'].contains(&chr)
            }
            TextFilter::All => true,
        }
    }
}

#[derive(Debug)]
pub struct TextField {
    content: String,
    char_width: usize,
    bounds: Rect,
    focused: bool,
    background: Drawable<Rect>,
    border: Drawable<Rect>,
    cursor_blink_visible: bool,
    next_cursor_change: f64,
    text_size: TextSize,
    cursor: Drawable<Rect>,
    filters: Vec<TextFilter>,
    style: TextFieldStyle,
}

impl TextField {
    pub fn new<P: Into<Coord>>(
        xy: P,
        max_length: usize,
        text_size: TextSize,
        min_width: Option<usize>,
        initial_content: &str,
        filters: &[TextFilter],
        style: &TextFieldStyle,
    ) -> Self {
        let rect = Rect::new_with_size(
            xy,
            ((text_size.get_size().0 + text_size.get_spacing()) * max_length
                + text_size.get_spacing())
            .max(min_width.unwrap_or_default()),
            ((text_size.get_size().1 + text_size.get_spacing()) as f32 * 1.4) as usize,
        );
        let background = Drawable::from_obj(rect.clone(), fill(WHITE));
        let border = Drawable::from_obj(rect.clone(), stroke(DARK_GRAY));
        let cursor = Drawable::from_obj(
            Rect::new(
                (0, 0),
                Coord::from(text_size.get_size()) + (0, text_size.get_spacing()),
            ),
            fill(BLACK),
        );
        TextField {
            char_width: max_length,
            content: initial_content.to_string(),
            bounds: rect,
            focused: false,
            background,
            border,
            cursor_blink_visible: true,
            next_cursor_change: 0.0,
            text_size,
            cursor,
            filters: filters.to_vec(),
            style: style.clone(),
        }
    }
}

impl TextField {
    pub fn clear(&mut self) {
        self.content.clear();
    }

    pub fn set_content(&mut self, text: &str) {
        self.content = text.to_string();
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn is_focused(&self) -> bool {
        self.focused
    }

    pub fn unfocus(&mut self) {
        self.focused = false
    }

    pub fn focus(&mut self) {
        self.focused = true
    }

    pub fn is_full(&self) -> bool {
        self.content.len() == self.char_width
    }

    pub fn on_key_press(&mut self, key: VirtualKeyCode) {
        if !self.focused {
            return;
        }

        let input = key_code_to_char(key);
        match input {
            Some((lower, _)) => {
                for filter in &self.filters {
                    if filter.is_char_allowed(lower) {
                        if !self.is_full() {
                            self.content.push(lower);
                        }
                        break;
                    }
                }
            }
            None => {
                if key == VirtualKeyCode::Back && !self.content.is_empty() {
                    self.content.remove(self.content.len() - 1);
                }
            }
        }
    }

    pub fn on_mouse_click(&mut self, mouse_xy: Coord) {
        self.focused = self.bounds.contains(mouse_xy);
    }

    pub fn update(&mut self, timing: &Timing) {
        if self.next_cursor_change < 0.0 {
            self.cursor_blink_visible = !self.cursor_blink_visible;
            self.next_cursor_change = CURSOR_BLINK_RATE;
        }
        self.next_cursor_change -= timing.fixed_time_step;
    }
}

impl Ui for TextField {
    #[must_use]
    fn bounds(&self) -> &Rect {
        &self.bounds
    }

    fn render(&self, graphics: &mut Graphics, xy: Coord) {
        let hovered = self.bounds.contains(xy);
        if let Some(color) = self.style.background_color.get(hovered, self.focused) {
            self.background.with_draw_type(fill(color)).render(graphics);
        }
        if let Some(color) = self.style.border_color.get(hovered, self.focused) {
            self.border.with_draw_type(stroke(color)).render(graphics);
        }
        if let Some(color) = self.style.text_color.get(hovered, self.focused) {
            graphics.draw_text(
                &self.content,
                Px(
                    self.bounds.left() + self.text_size.get_spacing() as isize,
                    self.bounds.top()
                        + (self.bounds.height() as isize / 2)
                        + self.text_size.get_spacing() as isize,
                ),
                (color, self.text_size, Cutoff(self.char_width), LeftCenter),
            );
        }
        if self.focused && self.cursor_blink_visible {
            let mut xy = self.bounds.top_left()
                + (
                    self.text_size
                        .measure(&self.content, WrappingStrategy::None)
                        .0
                        + 1,
                    self.text_size.get_spacing(),
                );
            if self.is_full() {
                xy = xy
                    - (
                        self.text_size.get_size().0 + self.text_size.get_spacing(),
                        0,
                    );
            }
            if let Some(color) = self.style.cursor.get(hovered, self.focused) {
                self.cursor
                    .with_draw_type(fill(color))
                    .with_move(xy)
                    .render(graphics);
            }
        }
    }
}
