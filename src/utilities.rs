use winit::event::VirtualKeyCode;

pub mod virtual_key_codes {
    use winit::event::VirtualKeyCode;

    pub const LETTERS: [VirtualKeyCode; 26] = [
        VirtualKeyCode::A,
        VirtualKeyCode::B,
        VirtualKeyCode::C,
        VirtualKeyCode::D,
        VirtualKeyCode::E,
        VirtualKeyCode::F,
        VirtualKeyCode::G,
        VirtualKeyCode::H,
        VirtualKeyCode::I,
        VirtualKeyCode::J,
        VirtualKeyCode::K,
        VirtualKeyCode::L,
        VirtualKeyCode::M,
        VirtualKeyCode::N,
        VirtualKeyCode::O,
        VirtualKeyCode::P,
        VirtualKeyCode::Q,
        VirtualKeyCode::R,
        VirtualKeyCode::S,
        VirtualKeyCode::T,
        VirtualKeyCode::U,
        VirtualKeyCode::V,
        VirtualKeyCode::W,
        VirtualKeyCode::X,
        VirtualKeyCode::Y,
        VirtualKeyCode::Z,
    ];
    pub const NUMBERS: [VirtualKeyCode; 10] = [
        VirtualKeyCode::Key0,
        VirtualKeyCode::Key1,
        VirtualKeyCode::Key2,
        VirtualKeyCode::Key3,
        VirtualKeyCode::Key4,
        VirtualKeyCode::Key5,
        VirtualKeyCode::Key6,
        VirtualKeyCode::Key7,
        VirtualKeyCode::Key8,
        VirtualKeyCode::Key9,
    ];
    pub const ARROWS: [VirtualKeyCode; 4] = [
        VirtualKeyCode::Up,
        VirtualKeyCode::Down,
        VirtualKeyCode::Left,
        VirtualKeyCode::Right,
    ];
    pub const MODIFIERS: [VirtualKeyCode; 8] = [
        VirtualKeyCode::LShift,
        VirtualKeyCode::RShift,
        VirtualKeyCode::LAlt,
        VirtualKeyCode::RAlt,
        VirtualKeyCode::RWin,
        VirtualKeyCode::LWin,
        VirtualKeyCode::LControl,
        VirtualKeyCode::RControl,
    ];
    pub const TYPING: [VirtualKeyCode; 4] = [
        VirtualKeyCode::Space,
        VirtualKeyCode::Return,
        VirtualKeyCode::Tab,
        VirtualKeyCode::Back,
    ];
    pub const SYMBOLS: [VirtualKeyCode; 9] = [
        VirtualKeyCode::Minus,
        VirtualKeyCode::Semicolon,
        VirtualKeyCode::Equals,
        VirtualKeyCode::LBracket,
        VirtualKeyCode::RBracket,
        VirtualKeyCode::Comma,
        VirtualKeyCode::Period,
        VirtualKeyCode::Slash,
        VirtualKeyCode::Apostrophe,
    ];
}

pub fn key_press_to_char(code: VirtualKeyCode, shift: bool) -> Option<char> {
    key_code_to_char(code).map(|(c1, c2)| if shift { c2 } else { c1 })
}

pub fn key_code_to_char(code: VirtualKeyCode) -> Option<(char, char)> {
    match code {
        VirtualKeyCode::Key1 => Some(('1', '!')),
        VirtualKeyCode::Key2 => Some(('2', '@')),
        VirtualKeyCode::Key3 => Some(('3', '£')),
        VirtualKeyCode::Key4 => Some(('4', '$')),
        VirtualKeyCode::Key5 => Some(('5', '%')),
        VirtualKeyCode::Key6 => Some(('6', '^')),
        VirtualKeyCode::Key7 => Some(('7', '&')),
        VirtualKeyCode::Key8 => Some(('8', '*')),
        VirtualKeyCode::Key9 => Some(('9', '(')),
        VirtualKeyCode::Key0 => Some(('0', ')')),
        VirtualKeyCode::A => Some(('a', 'A')),
        VirtualKeyCode::B => Some(('b', 'B')),
        VirtualKeyCode::C => Some(('c', 'C')),
        VirtualKeyCode::D => Some(('d', 'D')),
        VirtualKeyCode::E => Some(('e', 'E')),
        VirtualKeyCode::F => Some(('f', 'F')),
        VirtualKeyCode::G => Some(('g', 'G')),
        VirtualKeyCode::H => Some(('h', 'H')),
        VirtualKeyCode::I => Some(('i', 'I')),
        VirtualKeyCode::J => Some(('j', 'J')),
        VirtualKeyCode::K => Some(('k', 'K')),
        VirtualKeyCode::L => Some(('l', 'L')),
        VirtualKeyCode::M => Some(('m', 'M')),
        VirtualKeyCode::N => Some(('n', 'N')),
        VirtualKeyCode::O => Some(('o', 'O')),
        VirtualKeyCode::P => Some(('p', 'P')),
        VirtualKeyCode::Q => Some(('q', 'Q')),
        VirtualKeyCode::R => Some(('r', 'R')),
        VirtualKeyCode::S => Some(('s', 'S')),
        VirtualKeyCode::T => Some(('t', 'T')),
        VirtualKeyCode::U => Some(('u', 'U')),
        VirtualKeyCode::V => Some(('v', 'V')),
        VirtualKeyCode::W => Some(('w', 'W')),
        VirtualKeyCode::X => Some(('x', 'X')),
        VirtualKeyCode::Y => Some(('y', 'Y')),
        VirtualKeyCode::Z => Some(('z', 'Z')),
        VirtualKeyCode::Space => Some((' ', ' ')),
        VirtualKeyCode::Numpad0 => Some(('0', '0')),
        VirtualKeyCode::Numpad1 => Some(('1', '1')),
        VirtualKeyCode::Numpad2 => Some(('2', '2')),
        VirtualKeyCode::Numpad3 => Some(('3', '3')),
        VirtualKeyCode::Numpad4 => Some(('4', '4')),
        VirtualKeyCode::Numpad5 => Some(('5', '5')),
        VirtualKeyCode::Numpad6 => Some(('6', '6')),
        VirtualKeyCode::Numpad7 => Some(('7', '7')),
        VirtualKeyCode::Numpad8 => Some(('8', '8')),
        VirtualKeyCode::Numpad9 => Some(('9', '9')),
        VirtualKeyCode::NumpadAdd => Some(('+', '+')),
        VirtualKeyCode::NumpadDivide => Some(('/', '/')),
        VirtualKeyCode::NumpadDecimal => Some(('.', '.')),
        VirtualKeyCode::NumpadComma => Some((',', ',')),
        VirtualKeyCode::NumpadEquals => Some(('=', '=')),
        VirtualKeyCode::NumpadMultiply => Some(('*', '*')),
        VirtualKeyCode::NumpadSubtract => Some(('-', '-')),
        VirtualKeyCode::Apostrophe => Some(('\'', '"')),
        VirtualKeyCode::Asterisk => Some(('*', '*')),
        VirtualKeyCode::At => Some(('@', '@')),
        VirtualKeyCode::Backslash => Some(('\\', '|')),
        VirtualKeyCode::Colon => Some((':', ':')),
        VirtualKeyCode::Comma => Some((',', '<')),
        VirtualKeyCode::Equals => Some(('=', '+')),
        VirtualKeyCode::Grave => Some(('1', '1')),
        VirtualKeyCode::LBracket => Some(('[', '{')),
        VirtualKeyCode::Minus => Some(('-', '_')),
        VirtualKeyCode::Period => Some(('.', '>')),
        VirtualKeyCode::Plus => Some(('+', '+')),
        VirtualKeyCode::Semicolon => Some((';', ':')),
        VirtualKeyCode::Slash => Some(('/', '?')),
        VirtualKeyCode::Underline => Some(('_', '_')),
        _ => None,
    }
}