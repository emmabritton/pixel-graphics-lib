/// TextSize is used to set the size and positioning in pixels of text
#[derive(Copy, Clone, Debug, Hash)]
pub enum TextSize {
    Small,
    Normal,
}

#[derive(Copy, Clone, Debug)]
pub enum TextPos {
    Px(isize, isize),
    /// See [TextSize::get_max_characters] for maximum x and y
    Coord(usize, usize),
}

impl TextPos {
    pub fn usize_px(x: usize, y: usize) -> TextPos {
        TextPos::Px(x as isize, y as isize)
    }

    pub fn f32_px(x: f32, y: f32) -> TextPos {
        TextPos::Px(x as isize, y as isize)
    }
}

impl TextSize {
    /// Returns width, height of text size in pixels
    pub fn get_size(&self) -> (usize, usize) {
        match self {
            TextSize::Small => (small_letters::CHAR_WIDTH, small_letters::CHAR_HEIGHT),
            TextSize::Normal => (normal_letters::CHAR_WIDTH, normal_letters::CHAR_HEIGHT),
        }
    }

    /// Returns the spacing between letters in pixels
    pub fn get_margin(&self) -> usize {
        match self {
            TextSize::Small => 1,
            TextSize::Normal => 2,
        }
    }

    /// Returns the max of (columns, rows) for this text size for the specified screen size
    pub fn get_max_characters(&self, screen_width: usize, screen_height: usize) -> (usize, usize) {
        let size = self.get_size();
        if screen_width < size.0 || screen_height < size.1 {
            return (0, 0);
        }
        let sw = screen_width as f32;
        let cw = (size.0 + self.get_margin()) as f32;
        let sh = screen_height as f32;
        let ch = (size.1 + self.get_margin()) as f32;
        let columns = (sw / cw).floor() as usize;
        let rows = (sh / ch).floor() as usize;
        (columns - 1, rows - 1)
    }
}

pub mod small_letters {
    pub const CHAR_WIDTH: usize = 4;
    pub const CHAR_HEIGHT: usize = 5;

    pub fn get_px(chr: char) -> [bool; 20] {
        match chr.to_ascii_uppercase() {
            'A' => A,
            'B' => B,
            'C' => C,
            'D' => D,
            'E' => E,
            'F' => F,
            'G' => G,
            'H' => H,
            'I' => I,
            'J' => J,
            'K' => K,
            'L' => L,
            'M' => M,
            'N' => N,
            'O' => O,
            'P' => P,
            'R' => R,
            'S' => S,
            'T' => T,
            'U' => U,
            'Q' => Q,
            'W' => W,
            'V' => V,
            'X' => X,
            'Y' => Y,
            'Z' => Z,
            '!' => EXCLAIM,
            '.' => PERIOD,
            ':' => COLON,
            '+' => PLUS,
            '-' => MINUS,
            '_' => UNDERSCORE,
            '=' => EQUALS,
            '[' => SQUARE_L,
            ']' => SQUARE_R,
            '(' => PAREN_L,
            ')' => PAREN_R,
            '<' => ANGLE_L,
            '>' => ANGLE_R,
            '"' => DOUBLE_QUOTE,
            '\'' => QUOTE,
            '&' => AMPERSAND,
            '?' => QUESTION,
            '/' => SLASH,
            '*' => ASTERISK,
            '%' => PERCENT,
            '#' => HASH,
            '0' => ZERO,
            '1' => ONE,
            '2' => TWO,
            '3' => THREE,
            '4' => FOUR,
            '5' => FIVE,
            '6' => SIX,
            '7' => SEVEN,
            '8' => EIGHT,
            '9' => NINE,
            ',' => COMMA,
            _ => UNKNOWN,
        }
    }

    const LETTER_PX_COUNT: usize = 4 * 5;

    const A: [bool; LETTER_PX_COUNT] = [
        false, true, true, false, true, false, false, true, true, true, true, true, true, false,
        false, true, true, false, false, true,
    ];
    const B: [bool; LETTER_PX_COUNT] = [
        true, true, true, false, true, false, false, true, true, true, true, false, true, false,
        false, true, true, true, true, false,
    ];
    const C: [bool; LETTER_PX_COUNT] = [
        false, true, true, false, true, false, false, true, true, false, false, false, true, false,
        false, true, false, true, true, false,
    ];
    const D: [bool; LETTER_PX_COUNT] = [
        true, true, true, false, true, false, false, true, true, false, false, true, true, false,
        false, true, true, true, true, false,
    ];
    const E: [bool; LETTER_PX_COUNT] = [
        true, true, true, true, true, false, false, false, true, true, true, false, true, false,
        false, false, true, true, true, true,
    ];
    const F: [bool; LETTER_PX_COUNT] = [
        true, true, true, true, true, false, false, false, true, true, true, false, true, false,
        false, false, true, false, false, false,
    ];
    const G: [bool; LETTER_PX_COUNT] = [
        false, true, true, false, true, false, false, false, true, false, true, true, true, false,
        false, true, false, true, true, false,
    ];
    const H: [bool; LETTER_PX_COUNT] = [
        true, false, false, true, true, false, false, true, true, true, true, true, true, false,
        false, true, true, false, false, true,
    ];
    const I: [bool; LETTER_PX_COUNT] = [
        false, false, true, false, false, false, true, false, false, false, true, false, false,
        false, true, false, false, false, true, false,
    ];
    const J: [bool; LETTER_PX_COUNT] = [
        false, false, false, true, false, false, false, true, false, false, false, true, true,
        false, false, true, false, true, true, false,
    ];
    const K: [bool; LETTER_PX_COUNT] = [
        true, false, false, true, true, false, true, false, true, true, false, false, true, false,
        true, false, true, false, false, true,
    ];
    const L: [bool; LETTER_PX_COUNT] = [
        true, false, false, false, true, false, false, false, true, false, false, false, true,
        false, false, false, true, true, true, true,
    ];
    const M: [bool; LETTER_PX_COUNT] = [
        true, false, false, true, true, true, true, true, true, false, false, true, true, false,
        false, true, true, false, false, true,
    ];
    const N: [bool; LETTER_PX_COUNT] = [
        true, false, false, true, true, true, false, true, true, false, true, true, true, false,
        false, true, true, false, false, true,
    ];
    const O: [bool; LETTER_PX_COUNT] = [
        false, true, true, false, true, false, false, true, true, false, false, true, true, false,
        false, true, false, true, true, false,
    ];
    const P: [bool; LETTER_PX_COUNT] = [
        true, true, true, false, true, false, false, true, true, true, true, false, true, false,
        false, false, true, false, false, false,
    ];
    const R: [bool; LETTER_PX_COUNT] = [
        true, true, true, false, true, false, false, true, true, true, true, false, true, false,
        true, false, true, false, false, true,
    ];
    const S: [bool; LETTER_PX_COUNT] = [
        false, true, true, true, true, false, false, false, false, true, true, false, false, false,
        false, true, true, true, true, false,
    ];
    const T: [bool; LETTER_PX_COUNT] = [
        true, true, true, true, false, false, true, false, false, false, true, false, false, false,
        true, false, false, false, true, false,
    ];
    const U: [bool; LETTER_PX_COUNT] = [
        true, false, false, true, true, false, false, true, true, false, false, true, true, false,
        false, true, false, true, true, false,
    ];
    const Q: [bool; LETTER_PX_COUNT] = [
        false, true, true, false, true, false, false, true, true, false, false, true, true, false,
        true, true, false, true, true, true,
    ];
    const W: [bool; LETTER_PX_COUNT] = [
        true, false, false, true, true, false, false, true, true, false, false, true, true, true,
        true, true, true, false, false, true,
    ];
    const V: [bool; LETTER_PX_COUNT] = [
        true, false, true, false, true, false, true, false, true, false, true, false, true, false,
        true, false, false, true, false, false,
    ];
    const X: [bool; LETTER_PX_COUNT] = [
        true, false, true, false, true, false, true, false, false, true, false, false, true, false,
        true, false, true, false, true, false,
    ];
    const Y: [bool; LETTER_PX_COUNT] = [
        true, false, true, false, true, false, true, false, false, true, false, false, false, true,
        false, false, false, true, false, false,
    ];
    const Z: [bool; LETTER_PX_COUNT] = [
        true, true, true, true, false, false, false, true, false, true, true, false, true, false,
        false, false, true, true, true, true,
    ];
    const AMPERSAND: [bool; LETTER_PX_COUNT] = [
        false, true, true, false, true, false, false, false, false, true, false, true, true, false,
        true, false, false, true, false, true,
    ];
    const EXCLAIM: [bool; LETTER_PX_COUNT] = [
        false, false, true, false, false, false, true, false, false, false, true, false, false,
        false, false, false, false, false, true, false,
    ];
    const PERIOD: [bool; LETTER_PX_COUNT] = [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, true, false, false,
    ];
    const COMMA: [bool; LETTER_PX_COUNT] = [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        true, false, false, false, true, false, false,
    ];
    const COLON: [bool; LETTER_PX_COUNT] = [
        false, false, false, false, false, true, false, false, false, false, false, false, false,
        true, false, false, false, false, false, false,
    ];
    const PLUS: [bool; LETTER_PX_COUNT] = [
        false, false, false, false, false, true, false, false, true, true, true, false, false,
        true, false, false, false, false, false, false,
    ];
    const MINUS: [bool; LETTER_PX_COUNT] = [
        false, false, false, false, false, false, false, false, true, true, true, false, false,
        false, false, false, false, false, false, false,
    ];
    const EQUALS: [bool; LETTER_PX_COUNT] = [
        false, false, false, false, true, true, true, false, false, false, false, false, true,
        true, true, false, false, false, false, false,
    ];
    const SQUARE_L: [bool; LETTER_PX_COUNT] = [
        false, true, true, false, false, true, false, false, false, true, false, false, false,
        true, false, false, false, true, true, false,
    ];
    const SQUARE_R: [bool; LETTER_PX_COUNT] = [
        false, true, true, false, false, false, true, false, false, false, true, false, false,
        false, true, false, false, true, true, false,
    ];
    const PAREN_L: [bool; LETTER_PX_COUNT] = [
        false, false, true, false, false, true, false, false, false, true, false, false, false,
        true, false, false, false, false, true, false,
    ];
    const PAREN_R: [bool; LETTER_PX_COUNT] = [
        false, true, false, false, false, false, true, false, false, false, true, false, false,
        false, true, false, false, true, false, false,
    ];
    const ANGLE_L: [bool; LETTER_PX_COUNT] = [
        false, false, false, true, false, false, true, false, false, true, false, false, false,
        false, true, false, false, false, false, true,
    ];
    const ANGLE_R: [bool; LETTER_PX_COUNT] = [
        true, false, false, false, false, true, false, false, false, false, true, false, false,
        true, false, false, true, false, false, false,
    ];
    const DOUBLE_QUOTE: [bool; LETTER_PX_COUNT] = [
        false, false, false, false, false, true, false, true, false, true, false, true, false,
        false, false, false, false, false, false, false,
    ];
    const QUOTE: [bool; LETTER_PX_COUNT] = [
        false, false, false, false, false, true, false, false, false, true, false, false, false,
        false, false, false, false, false, false, false,
    ];
    const QUESTION: [bool; LETTER_PX_COUNT] = [
        false, true, true, false, true, false, false, true, false, false, false, true, false,
        false, true, false, false, true, false, false,
    ];
    const SLASH: [bool; LETTER_PX_COUNT] = [
        false, false, false, false, false, false, false, true, false, false, true, false, false,
        true, false, false, true, false, false, false,
    ];
    const ASTERISK: [bool; LETTER_PX_COUNT] = [
        false, false, false, false, false, true, false, true, false, false, true, false, false,
        true, false, true, false, false, false, false,
    ];
    const PERCENT: [bool; LETTER_PX_COUNT] = [
        true, false, false, true, false, false, true, false, false, true, false, false, true,
        false, false, true, false, false, false, false,
    ];
    const ZERO: [bool; LETTER_PX_COUNT] = [
        false, true, false, false, true, false, true, false, true, false, true, false, true, false,
        true, false, false, true, false, false,
    ];
    const ONE: [bool; LETTER_PX_COUNT] = [
        false, false, true, false, false, true, true, false, false, false, true, false, false,
        false, true, false, false, true, true, true,
    ];
    const TWO: [bool; LETTER_PX_COUNT] = [
        false, true, true, false, true, false, false, true, false, false, true, false, false, true,
        false, false, true, true, true, true,
    ];
    const THREE: [bool; LETTER_PX_COUNT] = [
        false, true, true, false, false, false, false, true, false, true, true, false, false,
        false, false, true, false, true, true, false,
    ];
    const FOUR: [bool; LETTER_PX_COUNT] = [
        true, false, false, false, true, false, false, false, true, false, true, false, true, true,
        true, true, false, false, true, false,
    ];
    const FIVE: [bool; LETTER_PX_COUNT] = [
        true, true, true, true, true, false, false, false, true, true, true, false, false, false,
        false, true, true, true, true, false,
    ];
    const SIX: [bool; LETTER_PX_COUNT] = [
        false, true, true, false, true, false, false, false, true, true, true, false, true, false,
        false, true, false, true, true, false,
    ];
    const SEVEN: [bool; LETTER_PX_COUNT] = [
        true, true, true, true, false, false, false, true, false, false, true, false, false, true,
        false, false, false, true, false, false,
    ];
    const EIGHT: [bool; LETTER_PX_COUNT] = [
        false, true, true, false, true, false, false, true, false, true, true, false, true, false,
        false, true, false, true, true, false,
    ];
    const NINE: [bool; LETTER_PX_COUNT] = [
        false, true, true, false, true, false, false, true, false, true, true, true, false, false,
        false, true, false, true, true, false,
    ];
    const UNDERSCORE: [bool; LETTER_PX_COUNT] = [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, true, true, true, true,
    ];
    const UNKNOWN: [bool; LETTER_PX_COUNT] = [
        true, true, true, true, true, false, false, true, true, false, false, true, true, false,
        false, true, true, true, true, true,
    ];
    const HASH: [bool; LETTER_PX_COUNT] = [
        false, true, false, true, true, true, true, true, false, true, false, true, true, true,
        true, true, false, true, false, true,
    ];
}

pub mod normal_letters {
    pub const CHAR_WIDTH: usize = 8;
    pub const CHAR_HEIGHT: usize = 10;

    pub fn get_px(chr: char) -> [bool; 80] {
        match chr.to_ascii_uppercase() {
            'A' => A,
            'B' => B,
            'C' => C,
            'D' => D,
            'E' => E,
            'F' => F,
            'G' => G,
            'H' => H,
            'I' => I,
            'J' => J,
            'K' => K,
            'L' => L,
            'M' => M,
            'N' => N,
            'O' => O,
            'P' => P,
            'Q' => Q,
            'R' => R,
            'S' => S,
            'T' => T,
            'U' => U,
            'V' => V,
            'W' => W,
            'X' => X,
            'Y' => Y,
            'Z' => Z,
            '.' => PERIOD,
            '=' => EQUALS,
            '-' => MINUS,
            '+' => PLUS,
            ',' => COMMA,
            '_' => UNDERSCORE,
            '"' => DOUBLE_QUOTE,
            '\'' => QUOTE,
            '!' => EXCLAIM,
            ':' => COLON,
            '?' => QUESTION,
            '/' => SLASH,
            '%' => PERCENT,
            '#' => HASH,
            '(' => PAREN_L,
            ')' => PAREN_R,
            '[' => SQUARE_L,
            ']' => SQUARE_R,
            '<' => ANGLE_L,
            '>' => ANGLE_R,
            '&' => AMPERSAND,
            '*' => ASTERISK,
            '0' => ZERO,
            '1' => ONE,
            '2' => TWO,
            '3' => THREE,
            '4' => FOUR,
            '5' => FIVE,
            '6' => SIX,
            '7' => SEVEN,
            '8' => EIGHT,
            '9' => NINE,
            _ => UNKNOWN,
        }
    }

    const LETTER_PX_COUNT: usize = 8 * 10;

    const A: [bool; LETTER_PX_COUNT] = [
        false, false, true, true, true, true, false, false, false, true, true, true, true, true,
        true, false, true, true, true, false, false, true, true, true, true, true, false, false,
        false, false, true, true, true, true, false, false, false, false, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, false, false, false, false, true, true, true, true, false, false, false, false, true,
        true, true, true, false, false, false, false, true, true,
    ];
    const B: [bool; LETTER_PX_COUNT] = [
        true, true, true, true, true, true, true, false, true, true, true, true, true, true, true,
        true, true, true, false, false, false, false, true, true, true, true, false, false, false,
        false, true, true, true, true, true, true, true, true, true, false, true, true, true, true,
        true, true, true, false, true, true, false, false, false, false, true, true, true, true,
        false, false, false, false, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, false,
    ];
    const C: [bool; LETTER_PX_COUNT] = [
        false, true, true, true, true, true, true, false, true, true, true, true, true, true, true,
        true, true, true, false, false, false, false, true, true, true, true, false, false, false,
        false, false, false, true, true, false, false, false, false, false, false, true, true,
        false, false, false, false, false, false, true, true, false, false, false, false, false,
        false, true, true, false, false, false, false, true, true, true, true, true, true, true,
        true, true, true, false, true, true, true, true, true, true, false,
    ];
    const D: [bool; LETTER_PX_COUNT] = [
        true, true, true, true, true, true, false, false, true, true, true, true, true, true, true,
        false, true, true, false, false, false, true, true, true, true, true, false, false, false,
        false, true, true, true, true, false, false, false, false, true, true, true, true, false,
        false, false, false, true, true, true, true, false, false, false, false, true, true, true,
        true, false, false, false, true, true, true, true, true, true, true, true, true, true,
        false, true, true, true, true, true, true, false, false,
    ];
    const E: [bool; LETTER_PX_COUNT] = [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, false, false, false, false, false, false, true, true, false, false,
        false, false, false, false, true, true, true, true, true, true, false, false, true, true,
        true, true, true, true, false, false, true, true, false, false, false, false, false, false,
        true, true, false, false, false, false, false, false, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true,
    ];
    const F: [bool; LETTER_PX_COUNT] = [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, false, false, false, false, false, false, true, true, false, false,
        false, false, false, false, true, true, true, true, true, true, false, false, true, true,
        true, true, true, true, false, false, true, true, false, false, false, false, false, false,
        true, true, false, false, false, false, false, false, true, true, false, false, false,
        false, false, false, true, true, false, false, false, false, false, false,
    ];
    const G: [bool; LETTER_PX_COUNT] = [
        false, true, true, true, true, true, true, false, true, true, true, true, true, true, true,
        true, true, true, false, false, false, false, true, true, true, true, false, false, false,
        false, false, false, true, true, false, false, false, false, false, false, true, true,
        false, false, true, true, true, true, true, true, false, false, true, true, true, true,
        true, true, false, false, false, false, true, true, true, true, true, true, true, true,
        true, true, false, true, true, true, true, true, true, false,
    ];
    const H: [bool; LETTER_PX_COUNT] = [
        true, true, false, false, false, false, true, true, true, true, false, false, false, false,
        true, true, true, true, false, false, false, false, true, true, true, true, false, false,
        false, false, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, false, false, false, false, true, true, true,
        true, false, false, false, false, true, true, true, true, false, false, false, false, true,
        true, true, true, false, false, false, false, true, true,
    ];
    const I: [bool; LETTER_PX_COUNT] = [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, false, false, false, true, true, false, false, false, false, false, false, true,
        true, false, false, false, false, false, false, true, true, false, false, false, false,
        false, false, true, true, false, false, false, false, false, false, true, true, false,
        false, false, false, false, false, true, true, false, false, false, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true,
    ];
    const J: [bool; LETTER_PX_COUNT] = [
        false, false, false, false, false, false, true, true, false, false, false, false, false,
        false, true, true, false, false, false, false, false, false, true, true, false, false,
        false, false, false, false, true, true, false, false, false, false, false, false, true,
        true, false, false, false, false, false, false, true, true, true, true, false, false,
        false, false, true, true, true, true, false, false, false, false, true, true, true, true,
        true, true, true, true, true, true, false, true, true, true, true, true, true, false,
    ];
    const K: [bool; LETTER_PX_COUNT] = [
        true, true, false, false, false, false, true, true, true, true, false, false, false, true,
        true, true, true, true, false, false, true, true, true, false, true, true, false, true,
        true, true, false, false, true, true, true, true, true, false, false, false, true, true,
        true, true, true, false, false, false, true, true, false, true, true, true, false, false,
        true, true, false, false, true, true, true, false, true, true, false, false, false, true,
        true, true, true, true, false, false, false, false, true, true,
    ];
    const L: [bool; LETTER_PX_COUNT] = [
        true, true, false, false, false, false, false, false, true, true, false, false, false,
        false, false, false, true, true, false, false, false, false, false, false, true, true,
        false, false, false, false, false, false, true, true, false, false, false, false, false,
        false, true, true, false, false, false, false, false, false, true, true, false, false,
        false, false, false, false, true, true, false, false, false, false, false, false, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    ];
    const M: [bool; LETTER_PX_COUNT] = [
        true, true, false, false, false, false, true, true, true, true, false, false, false, false,
        true, true, true, true, true, false, false, true, true, true, true, true, true, false,
        false, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, false, true, true, false, true, true, true, true,
        false, true, true, false, true, true, true, true, false, true, true, false, true, true,
        true, true, false, true, true, false, true, true,
    ];
    const N: [bool; LETTER_PX_COUNT] = [
        true, true, false, false, false, false, true, true, true, true, false, false, false, false,
        true, true, true, true, true, false, false, false, true, true, true, true, true, true,
        false, false, true, true, true, true, true, true, true, false, true, true, true, true,
        false, true, true, true, true, true, true, true, false, false, true, true, true, true,
        true, true, false, false, false, true, true, true, true, true, false, false, false, false,
        true, true, true, true, false, false, false, false, true, true,
    ];
    const O: [bool; LETTER_PX_COUNT] = [
        false, true, true, true, true, true, true, false, true, true, true, true, true, true, true,
        true, true, true, false, false, false, false, true, true, true, true, false, false, false,
        false, true, true, true, true, false, false, false, false, true, true, true, true, false,
        false, false, false, true, true, true, true, false, false, false, false, true, true, true,
        true, false, false, false, false, true, true, true, true, true, true, true, true, true,
        true, false, true, true, true, true, true, true, false,
    ];
    const P: [bool; LETTER_PX_COUNT] = [
        true, true, true, true, true, true, true, false, true, true, true, true, true, true, true,
        true, true, true, false, false, false, false, true, true, true, true, false, false, false,
        false, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, false, true, true, false, false, false, false, false, false, true, true,
        false, false, false, false, false, false, true, true, false, false, false, false, false,
        false, true, true, false, false, false, false, false, false,
    ];
    const Q: [bool; LETTER_PX_COUNT] = [
        false, true, true, true, true, true, true, false, true, true, true, true, true, true, true,
        true, true, true, false, false, false, false, true, true, true, true, false, false, false,
        false, true, true, true, true, false, false, false, false, true, true, true, true, false,
        true, true, false, true, true, true, true, false, true, true, true, true, true, true, true,
        false, false, true, true, true, false, true, true, true, true, true, true, true, true,
        false, true, true, true, true, false, true, true,
    ];
    const R: [bool; LETTER_PX_COUNT] = [
        true, true, true, true, true, true, true, false, true, true, true, true, true, true, true,
        true, true, true, false, false, false, false, true, true, true, true, false, false, false,
        false, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, false, true, true, false, true, true, true, false, false, true, true,
        false, false, true, true, true, false, true, true, false, false, false, true, true, true,
        true, true, false, false, false, false, true, true,
    ];
    const S: [bool; LETTER_PX_COUNT] = [
        false, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, false, false, false, false, false, false, true, true, false, false,
        false, false, false, false, true, true, true, true, true, true, true, false, false, true,
        true, true, true, true, true, true, false, false, false, false, false, false, true, true,
        false, false, false, false, false, false, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, false,
    ];
    const T: [bool; LETTER_PX_COUNT] = [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, false, false, false, true, true, false, false, false, false, false, false, true,
        true, false, false, false, false, false, false, true, true, false, false, false, false,
        false, false, true, true, false, false, false, false, false, false, true, true, false,
        false, false, false, false, false, true, true, false, false, false, false, false, false,
        true, true, false, false, false, false, false, false, true, true, false, false, false,
    ];
    const U: [bool; LETTER_PX_COUNT] = [
        true, true, false, false, false, false, true, true, true, true, false, false, false, false,
        true, true, true, true, false, false, false, false, true, true, true, true, false, false,
        false, false, true, true, true, true, false, false, false, false, true, true, true, true,
        false, false, false, false, true, true, true, true, false, false, false, false, true, true,
        true, true, false, false, false, false, true, true, true, true, true, true, true, true,
        true, true, false, true, true, true, true, true, true, false,
    ];
    const V: [bool; LETTER_PX_COUNT] = [
        true, true, false, false, false, false, true, true, true, true, false, false, false, false,
        true, true, true, true, false, false, false, false, true, true, true, true, false, false,
        false, false, true, true, true, true, false, false, false, false, true, true, true, true,
        false, false, false, false, true, true, true, true, false, false, false, false, true, true,
        false, true, true, false, false, true, true, false, false, false, true, true, true, true,
        false, false, false, false, false, true, true, false, false, false,
    ];
    const W: [bool; LETTER_PX_COUNT] = [
        true, true, false, true, true, false, true, true, true, true, false, true, true, false,
        true, true, true, true, false, true, true, false, true, true, true, true, false, true,
        true, false, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, false, false, true, true, true, true, true,
        true, false, false, true, true, true, true, true, false, false, false, false, true, true,
        true, true, false, false, false, false, true, true,
    ];
    const X: [bool; LETTER_PX_COUNT] = [
        true, true, false, false, false, false, true, true, true, true, false, false, false, false,
        true, true, false, true, true, false, false, true, true, false, false, true, true, false,
        false, true, true, false, false, false, true, true, true, true, false, false, false, false,
        true, true, true, true, false, false, false, true, true, false, false, true, true, false,
        false, true, true, false, false, true, true, false, true, true, false, false, false, false,
        true, true, true, true, false, false, false, false, true, true,
    ];
    const Y: [bool; LETTER_PX_COUNT] = [
        true, true, false, false, false, false, true, true, true, true, false, false, false, false,
        true, true, true, true, false, false, false, false, true, true, true, true, true, false,
        false, true, true, true, false, true, true, true, true, true, true, false, false, false,
        true, true, true, true, false, false, false, false, false, true, true, false, false, false,
        false, false, false, true, true, false, false, false, false, false, false, true, true,
        false, false, false, false, false, false, true, true, false, false, false,
    ];
    const Z: [bool; LETTER_PX_COUNT] = [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, false, false, false, false, false, true, true, true, false, false, false, false,
        true, true, true, false, false, false, false, true, true, true, false, false, false, false,
        true, true, true, false, false, false, false, true, true, true, false, false, false, false,
        true, true, true, false, false, false, false, false, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true,
    ];
    const ASTERISK: [bool; LETTER_PX_COUNT] = [
        false, false, false, false, false, false, false, false, false, false, false, true, false,
        false, false, false, false, true, false, true, false, true, false, false, false, false,
        true, true, true, false, false, false, false, true, true, true, true, true, false, false,
        false, false, true, true, true, false, false, false, false, true, false, true, false, true,
        false, false, false, false, false, true, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
    ];
    const AMPERSAND: [bool; LETTER_PX_COUNT] = [
        false, true, true, true, true, true, false, false, true, true, false, false, false, true,
        true, false, true, true, false, false, false, true, true, false, false, true, true, true,
        true, true, false, false, false, true, true, true, true, true, false, true, true, true,
        false, false, true, true, true, true, true, false, false, false, false, true, true, false,
        true, true, false, false, false, false, true, false, true, true, true, true, true, true,
        true, true, false, true, true, true, true, false, true, true,
    ];
    const ZERO: [bool; LETTER_PX_COUNT] = [
        false, true, true, true, true, true, true, false, true, true, true, true, true, true, true,
        true, true, true, false, false, false, true, true, true, true, true, false, false, true,
        true, true, true, true, true, false, true, true, true, true, true, true, true, true, true,
        true, false, true, true, true, true, true, true, false, false, true, true, true, true,
        true, false, false, false, true, true, true, true, true, true, true, true, true, true,
        false, true, true, true, true, true, true, false,
    ];
    const ONE: [bool; LETTER_PX_COUNT] = [
        false, false, true, true, true, false, false, false, false, true, true, true, true, false,
        false, false, true, true, true, true, true, false, false, false, true, true, false, true,
        true, false, false, false, false, false, false, true, true, false, false, false, false,
        false, false, true, true, false, false, false, false, false, false, true, true, false,
        false, false, false, false, false, true, true, false, false, false, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true,
    ];
    const TWO: [bool; LETTER_PX_COUNT] = [
        false, true, true, true, true, true, true, false, true, true, true, true, true, true, true,
        true, true, true, false, false, false, false, true, true, false, false, false, false,
        false, false, true, true, false, false, true, true, true, true, true, true, false, true,
        true, true, true, true, true, false, true, true, true, false, false, false, false, false,
        true, true, false, false, false, false, false, false, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true,
    ];
    const THREE: [bool; LETTER_PX_COUNT] = [
        false, true, true, true, true, true, true, false, true, true, true, true, true, true, true,
        true, true, true, false, false, false, false, true, true, false, false, false, false,
        false, false, true, true, false, false, true, true, true, true, true, false, false, false,
        true, true, true, true, true, false, false, false, false, false, false, false, true, true,
        true, true, false, false, false, false, true, true, true, true, true, true, true, true,
        true, true, false, true, true, true, true, true, true, false,
    ];
    const FOUR: [bool; LETTER_PX_COUNT] = [
        true, true, false, false, false, false, true, true, true, true, false, false, false, false,
        true, true, true, true, false, false, false, false, true, true, true, true, false, false,
        false, false, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, false, false, false, false, false, false, true, true, false,
        false, false, false, false, false, true, true, false, false, false, false, false, false,
        true, true, false, false, false, false, false, false, true, true,
    ];
    const FIVE: [bool; LETTER_PX_COUNT] = [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, false, false, false, false, false, false, true, true, false, false,
        false, false, false, false, true, true, true, true, true, true, true, false, true, true,
        true, true, true, true, true, true, false, false, false, false, false, false, true, true,
        true, true, false, false, false, false, true, true, true, true, true, true, true, true,
        true, true, false, true, true, true, true, true, true, false,
    ];
    const SIX: [bool; LETTER_PX_COUNT] = [
        false, true, true, true, true, true, true, false, true, true, true, true, true, true, true,
        true, true, true, false, false, false, false, true, true, true, true, false, false, false,
        false, false, false, true, true, true, true, true, true, true, false, true, true, true,
        true, true, true, true, true, true, true, false, false, false, false, true, true, true,
        true, false, false, false, false, true, true, true, true, true, true, true, true, true,
        true, false, true, true, true, true, true, true, false,
    ];
    const SEVEN: [bool; LETTER_PX_COUNT] = [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, false, false, false, false, false, false, true, true, false, false, false, false,
        false, true, true, true, false, false, false, false, true, true, true, false, false, false,
        false, true, true, true, false, false, false, false, true, true, true, false, false, false,
        false, true, true, true, false, false, false, false, false, true, true, false, false,
        false, false, false, false, true, true, false, false, false, false, false,
    ];
    const EIGHT: [bool; LETTER_PX_COUNT] = [
        false, true, true, true, true, true, true, false, true, true, true, true, true, true, true,
        true, true, true, false, false, false, false, true, true, true, true, false, false, false,
        false, true, true, false, true, true, true, true, true, true, false, false, true, true,
        true, true, true, true, false, true, true, false, false, false, false, true, true, true,
        true, false, false, false, false, true, true, true, true, true, true, true, true, true,
        true, false, true, true, true, true, true, true, false,
    ];
    const NINE: [bool; LETTER_PX_COUNT] = [
        false, true, true, true, true, true, true, false, true, true, true, true, true, true, true,
        true, true, true, false, false, false, false, true, true, true, true, false, false, false,
        false, true, true, true, true, true, true, true, true, true, true, false, true, true, true,
        true, true, true, true, false, false, false, false, false, false, true, true, true, true,
        false, false, false, false, true, true, true, true, true, true, true, true, true, true,
        false, true, true, true, true, true, true, false,
    ];
    const DOUBLE_QUOTE: [bool; LETTER_PX_COUNT] = [
        false, true, true, false, false, true, true, false, false, true, true, false, false, true,
        true, false, false, true, true, false, false, true, true, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false,
    ];
    const QUOTE: [bool; LETTER_PX_COUNT] = [
        false, false, false, true, true, false, false, false, false, false, false, true, true,
        false, false, false, false, false, false, true, true, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false,
    ];
    const PERIOD: [bool; LETTER_PX_COUNT] = [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, true, true, false, false, false, false, false, false, true, true, false,
        false, false,
    ];
    const COLON: [bool; LETTER_PX_COUNT] = [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, true, true, false, false, false, false, false,
        false, true, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, true,
        true, false, false, false, false, false, false, true, true, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false,
    ];
    const COMMA: [bool; LETTER_PX_COUNT] = [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, true, true, false, false, false, false,
        false, false, true, true, false, false, false, false, false, true, true, false, false,
        false, false,
    ];
    const UNDERSCORE: [bool; LETTER_PX_COUNT] = [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    ];
    const EXCLAIM: [bool; LETTER_PX_COUNT] = [
        false, false, false, true, true, false, false, false, false, false, false, true, true,
        false, false, false, false, false, false, true, true, false, false, false, false, false,
        false, true, true, false, false, false, false, false, false, true, true, false, false,
        false, false, false, false, true, true, false, false, false, false, false, false, true,
        true, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, true, true, false, false, false, false, false, false, true, true, false,
        false, false,
    ];
    const QUESTION: [bool; LETTER_PX_COUNT] = [
        false, true, true, true, true, true, true, false, true, true, true, true, true, true, true,
        true, true, true, false, false, false, false, true, true, true, true, false, false, false,
        false, true, true, false, false, false, false, true, true, true, true, false, false, false,
        true, true, true, true, false, false, false, false, true, true, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, true, true, false,
        false, false, false, false, false, true, true, false, false, false,
    ];
    const PAREN_L: [bool; LETTER_PX_COUNT] = [
        false, false, false, false, true, true, false, false, false, false, false, true, true,
        false, false, false, false, false, true, true, false, false, false, false, false, false,
        true, true, false, false, false, false, false, false, true, true, false, false, false,
        false, false, false, true, true, false, false, false, false, false, false, true, true,
        false, false, false, false, false, false, true, true, false, false, false, false, false,
        false, false, true, true, false, false, false, false, false, false, false, true, true,
        false, false,
    ];
    const PAREN_R: [bool; LETTER_PX_COUNT] = [
        false, false, true, true, false, false, false, false, false, false, false, true, true,
        false, false, false, false, false, false, false, true, true, false, false, false, false,
        false, false, true, true, false, false, false, false, false, false, true, true, false,
        false, false, false, false, false, true, true, false, false, false, false, false, false,
        true, true, false, false, false, false, false, false, true, true, false, false, false,
        false, false, true, true, false, false, false, false, false, true, true, false, false,
        false, false,
    ];
    const SQUARE_L: [bool; LETTER_PX_COUNT] = [
        false, false, true, true, true, true, false, false, false, false, true, true, false, false,
        false, false, false, false, true, true, false, false, false, false, false, false, true,
        true, false, false, false, false, false, false, true, true, false, false, false, false,
        false, false, true, true, false, false, false, false, false, false, true, true, false,
        false, false, false, false, false, true, true, false, false, false, false, false, false,
        true, true, false, false, false, false, false, false, true, true, true, true, false, false,
    ];
    const SQUARE_R: [bool; LETTER_PX_COUNT] = [
        false, false, true, true, true, true, false, false, false, false, false, false, true, true,
        false, false, false, false, false, false, true, true, false, false, false, false, false,
        false, true, true, false, false, false, false, false, false, true, true, false, false,
        false, false, false, false, true, true, false, false, false, false, false, false, true,
        true, false, false, false, false, false, false, true, true, false, false, false, false,
        false, false, true, true, false, false, false, false, true, true, true, true, false, false,
    ];
    const ANGLE_L: [bool; LETTER_PX_COUNT] = [
        false, false, false, false, false, true, true, false, false, false, false, false, true,
        true, false, false, false, false, false, true, true, false, false, false, false, false,
        true, true, false, false, false, false, false, true, true, false, false, false, false,
        false, false, true, true, false, false, false, false, false, false, false, true, true,
        false, false, false, false, false, false, false, true, true, false, false, false, false,
        false, false, false, true, true, false, false, false, false, false, false, false, true,
        true, false,
    ];
    const ANGLE_R: [bool; LETTER_PX_COUNT] = [
        false, true, true, false, false, false, false, false, false, false, true, true, false,
        false, false, false, false, false, false, true, true, false, false, false, false, false,
        false, false, true, true, false, false, false, false, false, false, false, true, true,
        false, false, false, false, false, false, true, true, false, false, false, false, false,
        true, true, false, false, false, false, false, true, true, false, false, false, false,
        false, true, true, false, false, false, false, false, true, true, false, false, false,
        false, false,
    ];
    const MINUS: [bool; LETTER_PX_COUNT] = [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, true, true, true, true, true, true, false,
        false, true, true, true, true, true, true, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
    ];
    const PLUS: [bool; LETTER_PX_COUNT] = [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, true, true, false, false, false, false, false,
        false, true, true, false, false, false, false, true, true, true, true, true, true, false,
        false, true, true, true, true, true, true, false, false, false, false, true, true, false,
        false, false, false, false, false, true, true, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
    ];
    const EQUALS: [bool; LETTER_PX_COUNT] = [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, true, true, true, true, true, true, false, false, true, true,
        true, true, true, true, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, true, true, true, true,
        true, true, false, false, true, true, true, true, true, true, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
    ];
    const SLASH: [bool; LETTER_PX_COUNT] = [
        false, false, false, false, false, false, false, true, false, false, false, false, false,
        false, true, true, false, false, false, false, false, true, true, true, false, false,
        false, false, true, true, true, false, false, false, false, true, true, true, false, false,
        false, false, true, true, true, false, false, false, false, true, true, true, false, false,
        false, false, true, true, true, false, false, false, false, false, true, true, false,
        false, false, false, false, false, true, false, false, false, false, false, false, false,
    ];
    const PERCENT: [bool; LETTER_PX_COUNT] = [
        false, true, false, false, false, false, false, true, true, false, true, false, false,
        false, true, true, false, true, false, false, false, true, true, true, false, false, false,
        false, true, true, true, false, false, false, false, true, true, true, false, false, false,
        false, true, true, true, false, false, false, false, true, true, true, false, false, false,
        false, true, true, true, false, false, false, true, false, true, true, false, false, false,
        true, false, true, true, false, false, false, false, false, true, false,
    ];
    const HASH: [bool; LETTER_PX_COUNT] = [
        false, false, true, true, false, true, true, false, false, false, true, true, false, true,
        true, false, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, false, false, true, true, false, true, true, false, false, false, true,
        true, false, true, true, false, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, false, false, true, true, false, true, true, false,
        false, false, true, true, false, true, true, false,
    ];
    const UNKNOWN: [bool; LETTER_PX_COUNT] = [
        true, true, true, true, true, true, true, true, true, false, false, false, false, false,
        false, true, true, false, false, false, false, false, false, true, true, false, false,
        false, false, false, false, true, true, false, false, false, false, false, false, true,
        true, false, false, false, false, false, false, true, true, false, false, false, false,
        false, false, true, true, false, false, false, false, false, false, true, true, false,
        false, false, false, false, false, true, true, true, true, true, true, true, true, true,
    ];
}
