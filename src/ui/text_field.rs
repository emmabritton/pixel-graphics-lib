use crate::prelude::*;
use crate::ui::prelude::*;
use crate::ui::styles::TextFieldStyle;
use crate::ui::UiElement;
use crate::utilities::key_code_to_char;
use buffer_graphics_lib::prelude::Positioning::LeftCenter;
use buffer_graphics_lib::prelude::WrappingStrategy::Cutoff;
use buffer_graphics_lib::prelude::*;

const CURSOR_BLINK_RATE: f64 = 0.5;

#[macro_export]
macro_rules! swap_focus {
    ($focus:expr, $( $unfocus:expr ),* ) => {{
        $focus.focus();
        $($unfocus.unfocus();)*
    }};
}

#[macro_export]
macro_rules! unfocus {
    ( $( $unfocus:expr ),* ) => {$($unfocus.unfocus();)*};
}

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
            TextFilter::Letters => chr.is_ascii_lowercase(),
            TextFilter::Numbers => chr.is_ascii_digit(),
            TextFilter::Hex => chr.is_ascii_hexdigit(),
            TextFilter::NegativeNumbers => chr.is_ascii_digit() || chr == '-',
            TextFilter::Decimal => chr.is_ascii_digit() || chr == '-' || chr == '.',
            TextFilter::Symbols => SUPPORTED_SYMBOLS.contains(&chr),
            TextFilter::Whitespace => chr == ' ',
            TextFilter::Filename => {
                chr.is_ascii_lowercase()
                    || chr.is_ascii_digit()
                    || ['(', ')', '-', '.', '_'].contains(&chr)
            }
            TextFilter::All => true,
        }
    }
}

#[derive(Debug)]
pub struct TextField {
    content: String,
    max_char_count: usize,
    bounds: Rect,
    focused: bool,
    background: Drawable<Rect>,
    border: Drawable<Rect>,
    cursor_pos: usize,
    cursor_blink_visible: bool,
    next_cursor_change: f64,
    text_size: TextSize,
    cursor: Drawable<Rect>,
    filters: Vec<TextFilter>,
    style: TextFieldStyle,
    state: ElementState,
    visible_count: usize,
    first_visible: usize,
}

impl TextField {
    /// UI element that allows text input
    /// Only supports characters in A-Z, a-z, 0-9, and some [symbols][SUPPORTED_SYMBOLS]
    /// a-z will be rendered as A-Z
    /// Does not support multiline
    ///
    /// By default the width of the field is `max_length * font width` but this can be restricted/overridden using `size_limits`
    ///
    /// # Params
    /// * `xy` - Coord of top left corne
    /// * `max_length` - Max number of chars
    /// * `text_size` - Size of text, effects width and height
    /// * `size_limits` - Optional min and optional max width of field in pixels (including border + padding)
    /// * `filters` - Filter allowed key, if empty then defaults to [All][TextFilter::All]
    pub fn new<P: Into<Coord>>(
        xy: P,
        max_length: usize,
        text_size: TextSize,
        size_limits: (Option<usize>, Option<usize>),
        initial_content: &str,
        filters: &[TextFilter],
        style: &TextFieldStyle,
    ) -> Self {
        let rect = Rect::new_with_size(
            xy,
            ((text_size.get_size().0 + text_size.get_spacing()) * max_length
                + text_size.get_spacing())
            .max(size_limits.0.unwrap_or_default())
            .min(size_limits.1.unwrap_or(usize::MAX)),
            ((text_size.get_size().1 + text_size.get_spacing()) as f32 * 1.4) as usize,
        );
        let visible_count = rect.width() / (text_size.get_size().0 + text_size.get_spacing());
        let (background, border) = Self::layout(&rect);
        let cursor =
            Drawable::from_obj(Rect::new((0, 0), (1, text_size.get_size().1)), fill(BLACK));
        let mut filters = filters.to_vec();
        if filters.is_empty() {
            filters.push(TextFilter::All);
        }
        TextField {
            cursor_pos: 0,
            visible_count,
            first_visible: 0,
            max_char_count: max_length,
            content: initial_content.to_string(),
            bounds: rect,
            focused: false,
            background,
            border,
            cursor_blink_visible: true,
            next_cursor_change: 0.0,
            text_size,
            cursor,
            filters,
            style: style.clone(),
            state: ElementState::Normal,
        }
    }

    fn layout(bounds: &Rect) -> (Drawable<Rect>, Drawable<Rect>) {
        let background = Drawable::from_obj(bounds.clone(), fill(WHITE));
        let border = Drawable::from_obj(bounds.clone(), stroke(DARK_GRAY));
        (background, border)
    }
}

impl TextField {
    #[inline]
    pub fn clear(&mut self) {
        self.content.clear();
    }

    #[inline]
    pub fn set_content(&mut self, text: &str) {
        self.content = text.to_string();
    }

    #[inline]
    pub fn content(&self) -> &str {
        &self.content
    }

    #[inline]
    pub fn is_focused(&self) -> bool {
        self.focused
    }

    #[inline]
    pub fn unfocus(&mut self) {
        self.focused = false
    }

    #[inline]
    pub fn focus(&mut self) {
        self.focused = true
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.content.len() == self.max_char_count
    }

    pub fn on_mouse_click(&mut self, mouse_xy: Coord) -> bool {
        if self.state != ElementState::Disabled {
            self.focused = self.bounds.contains(mouse_xy);
            return self.focused;
        }
        false
    }

    pub fn on_key_press(&mut self, key: VirtualKeyCode, held_keys: &Vec<&VirtualKeyCode>) {
        if !self.focused || self.state == ElementState::Disabled {
            return;
        }
        match key {
            VirtualKeyCode::Left => {
                if self.cursor_pos > 0 {
                    if self.cursor_pos > self.first_visible {
                        self.cursor_pos -= 1;
                    } else {
                        self.cursor_pos -= 1;
                        self.first_visible -= 1;
                    }
                }
            }
            VirtualKeyCode::Right => {
                if self.cursor_pos < self.content.chars().count() {
                    self.cursor_pos += 1;
                    if self.cursor_pos > self.first_visible + self.visible_count {
                        self.first_visible += 1;
                    }
                }
            }
            VirtualKeyCode::Back => {
                if !self.content.is_empty() && self.cursor_pos > 0 {
                    self.cursor_pos -= 1;
                    self.content.remove(self.cursor_pos);
                    let len = self.content.chars().count();
                    if self.visible_count >= len {
                        self.first_visible = 0;
                    } else {
                        while len < self.first_visible + self.visible_count {
                            self.first_visible -= 1;
                        }
                    }
                }
            }
            VirtualKeyCode::Delete => {
                let len = self.content.chars().count();
                if !self.content.is_empty() && self.cursor_pos < len {
                    self.content.remove(self.cursor_pos);
                    let len = self.content.chars().count();
                    if self.visible_count >= len {
                        self.first_visible = 0;
                    } else {
                        while len < self.first_visible + self.visible_count {
                            self.first_visible -= 1;
                        }
                    }
                }
            }
            _ => {
                if let Some((lower, upper)) = key_code_to_char(key) {
                    let shift_pressed = held_keys.contains(&&VirtualKeyCode::LShift)
                        || held_keys.contains(&&VirtualKeyCode::RShift);
                    for filter in &self.filters {
                        let char = if shift_pressed { upper } else { lower };
                        if filter.is_char_allowed(char) {
                            if !self.is_full() {
                                self.content.insert(self.cursor_pos, char);
                                if self.cursor_pos == self.content.chars().count() - 1 {
                                    self.cursor_pos += 1;
                                }
                                if self.cursor_pos > self.first_visible + self.visible_count {
                                    self.first_visible += 1;
                                }
                            }
                            break;
                        }
                    }
                }
            }
        }
    }
}

impl UiElement for TextField {
    fn set_position(&mut self, top_left: Coord) {
        self.bounds = self.bounds.move_to(top_left);
        let (background, border) = Self::layout(&self.bounds);
        self.background = background;
        self.border = border;
    }

    #[must_use]
    fn bounds(&self) -> &Rect {
        &self.bounds
    }

    fn render(&self, graphics: &mut Graphics, mouse_xy: Coord) {
        let (error, disabled) = self.state.get_err_dis();
        let hovered = self.bounds.contains(mouse_xy);
        if let Some(color) = self
            .style
            .background_color
            .get(hovered, self.focused, error, disabled)
        {
            self.background.with_draw_type(fill(color)).render(graphics);
        }
        if let Some(color) = self
            .style
            .border_color
            .get(hovered, self.focused, error, disabled)
        {
            self.border.with_draw_type(stroke(color)).render(graphics);
        }
        if let Some(color) = self
            .style
            .text_color
            .get(hovered, self.focused, error, disabled)
        {
            graphics.draw_text(
                &self
                    .content
                    .chars()
                    .skip(self.first_visible)
                    .collect::<String>(),
                TextPos::Px(
                    self.bounds.left() + self.text_size.get_spacing() as isize,
                    self.bounds.top()
                        + (self.bounds.height() as isize / 2)
                        + self.text_size.get_spacing() as isize,
                ),
                (
                    color,
                    self.text_size,
                    Cutoff(self.visible_count),
                    LeftCenter,
                ),
            );
        }
        if self.focused && self.cursor_blink_visible {
            let xy = self.bounds.top_left()
                + (
                    (self.text_size.get_size().0 + self.text_size.get_spacing())
                        * (self.cursor_pos - self.first_visible)
                        + 1,
                    self.text_size.get_spacing() + 1,
                );
            if let Some(color) = self
                .style
                .cursor
                .get(hovered, self.focused, error, disabled)
            {
                self.cursor
                    .with_draw_type(fill(color))
                    .with_move(xy)
                    .render(graphics);
            }
        }
    }

    fn update(&mut self, timing: &Timing) {
        if self.next_cursor_change < 0.0 {
            self.cursor_blink_visible = !self.cursor_blink_visible;
            self.next_cursor_change = CURSOR_BLINK_RATE;
        }
        self.next_cursor_change -= timing.fixed_time_step;
    }

    #[inline]
    fn set_state(&mut self, state: ElementState) {
        self.state = state;
        if self.state == ElementState::Disabled {
            self.focused = false;
        }
    }

    #[inline]
    fn get_state(&self) -> ElementState {
        self.state
    }
}
