use crate::prelude::*;
use crate::ui::prelude::*;
use crate::ui::styles::TextFieldStyle;
use crate::ui::PixelView;
use crate::utilities::key_code_to_char;
use buffer_graphics_lib::prelude::Positioning::LeftCenter;
use buffer_graphics_lib::prelude::WrappingStrategy::Cutoff;
use buffer_graphics_lib::prelude::*;

const CURSOR_BLINK_RATE: f64 = 0.5;

#[macro_export]
macro_rules! swap_focus {
    ($focus:expr, $( $unfocus:expr ),* $(,)? ) => {{
        $focus.focus();
        $($unfocus.unfocus();)*
    }};
}

#[macro_export]
macro_rules! unfocus {
    ( $( $unfocus:expr ),* $(,)? ) => {$($unfocus.unfocus();)*};
}

#[derive(Debug, Eq, PartialEq, Clone)]
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
    /// Letters, numbers, some punctuations (!,.?')
    Sentence,
    /// Letters and _-().
    Filename,
    /// Whatever you need
    Raw(Vec<char>),
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
            TextFilter::Raw(valid) => valid.contains(&chr),
            TextFilter::Sentence => {
                chr.is_ascii_lowercase()
                    || chr.is_ascii_digit()
                    || ['.', ',', '\'', '?', '!'].contains(&chr)
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
    font: PixelFont,
    cursor: Drawable<Rect>,
    filters: Vec<TextFilter>,
    style: TextFieldStyle,
    state: ViewState,
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
        font: PixelFont,
        size_limits: (Option<usize>, Option<usize>),
        initial_content: &str,
        filters: &[TextFilter],
        style: &TextFieldStyle,
    ) -> Self {
        let rect = Rect::new_with_size(
            xy,
            ((font.size().0 + font.spacing()) * max_length + font.spacing())
                .max(size_limits.0.unwrap_or_default())
                .min(size_limits.1.unwrap_or(usize::MAX)),
            ((font.size().1 + font.spacing()) as f32 * 1.4) as usize,
        );
        let visible_count = rect.width() / (font.size().0 + font.spacing());
        let (background, border) = Self::layout(&rect);
        let cursor = Drawable::from_obj(Rect::new((0, 0), (1, font.size().1)), fill(BLACK));
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
            font,
            cursor,
            filters,
            style: style.clone(),
            state: ViewState::Normal,
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

    pub fn on_mouse_click(&mut self, down: Coord, up: Coord) -> bool {
        if self.state != ViewState::Disabled {
            self.focused = self.bounds.contains(down) && self.bounds.contains(up);
            self.cursor_pos = (((up.x - self.bounds.left()) / (self.font.char_width() as isize))
                .max(0) as usize)
                .min(self.content.len());
            return self.focused;
        }
        false
    }

    pub fn on_key_press(&mut self, key: KeyCode, held_keys: &[KeyCode]) {
        if !self.focused || self.state == ViewState::Disabled {
            return;
        }
        match key {
            KeyCode::ArrowLeft => {
                if self.cursor_pos > 0 {
                    if self.cursor_pos > self.first_visible {
                        self.cursor_pos -= 1;
                    } else {
                        self.cursor_pos -= 1;
                        self.first_visible -= 1;
                    }
                }
            }
            KeyCode::ArrowRight => {
                if self.cursor_pos < self.content.chars().count() {
                    self.cursor_pos += 1;
                    if self.cursor_pos > self.first_visible + self.visible_count {
                        self.first_visible += 1;
                    }
                }
            }
            KeyCode::Backspace => {
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
            KeyCode::Delete => {
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
                    let shift_pressed = held_keys.contains(&KeyCode::ShiftLeft)
                        || held_keys.contains(&KeyCode::ShiftRight);
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

impl PixelView for TextField {
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

    fn render(&self, graphics: &mut Graphics, mouse: &MouseData) {
        let (error, disabled) = self.state.get_err_dis();
        let hovered = self.bounds.contains(mouse.xy);
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
                    self.bounds.left() + self.font.spacing() as isize,
                    self.bounds.top()
                        + (self.bounds.height() as isize / 2)
                        + self.font.spacing() as isize,
                ),
                (color, self.font, Cutoff(self.visible_count), LeftCenter),
            );
        }
        if self.focused && self.cursor_blink_visible {
            let xy = self.bounds.top_left()
                + (
                    (self.font.size().0 + self.font.spacing())
                        * (self.cursor_pos - self.first_visible)
                        + 1,
                    self.font.spacing() + 1,
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
    fn set_state(&mut self, state: ViewState) {
        self.state = state;
        if self.state == ViewState::Disabled {
            self.focused = false;
        }
    }

    #[inline]
    fn get_state(&self) -> ViewState {
        self.state
    }
}

impl LayoutView for TextField {
    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds.clone();
        self.set_position(bounds.top_left());
    }
}
