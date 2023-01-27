use anyhow::Result;
use buffer_graphics_lib::color::*;
use buffer_graphics_lib::text::TextSize;
use buffer_graphics_lib::Graphics;
use pixels_graphics_lib::prefs::WindowPreferences;
use pixels_graphics_lib::prelude::*;
use winit::event::VirtualKeyCode;

/// Running this example will create preference directories and files on your computer!
///
/// This example shows how to use `WindowPreferences` to save and restore the window size and position
/// It also has a small text demo

fn main() -> Result<()> {
    let width = 240;
    let height = 160;
    let system = WindowPrefsScene::new("Example Text", width, height);
    run(
        width,
        height,
        WindowScaling::AutoFixed(2),
        "Window Pos Example",
        Box::new(system),
        ExecutionSpeed::standard(),
    )?;
    Ok(())
}

struct WindowPrefsScene {
    text: &'static str,
    pos: (usize, usize),
    colors: Vec<Color>,
    idx: usize,
    should_exit: bool,
}

impl WindowPrefsScene {
    pub fn new(text: &'static str, width: usize, height: usize) -> Self {
        let (w, h) = Graphics::get_text_size(text, 12, TextSize::Normal);
        let pos = (width / 2 - w / 2, height / 2 - h / 2);
        WindowPrefsScene {
            text,
            pos,
            colors: vec![GREEN, RED, BLUE, WHITE, MAGENTA, YELLOW, CYAN],
            idx: 0,
            should_exit: false,
        }
    }
}

impl System for WindowPrefsScene {
    fn action_keys(&self) -> Vec<VirtualKeyCode> {
        vec![VirtualKeyCode::Escape]
    }

    fn window_prefs(&self) -> Option<WindowPreferences> {
        Some(WindowPreferences::new("app", "pixels-graphics-lib-example", "window-pos2").unwrap())
    }

    fn update(&mut self, _delta: &Timing) {
        if self.idx < self.colors.len() - 1 {
            self.idx += 1;
        } else {
            self.idx = 0;
        }
    }

    fn render(&self, graphics: &mut Graphics<'_>) {
        graphics.clear(BLACK);
        let mut color_idx = self.idx;
        for (i, letter) in self.text.chars().enumerate() {
            let mut pos = self.pos;
            pos.0 += TextSize::Normal.get_size().0 * i + TextSize::Normal.get_spacing() * i;
            graphics.draw_letter(
                (pos.0 as isize, pos.1 as isize),
                letter,
                TextSize::Normal,
                self.colors[color_idx],
            );

            color_idx += 1;
            if color_idx >= self.colors.len() {
                color_idx = 0;
            }
        }
    }

    fn on_key_down(&mut self, keys: Vec<VirtualKeyCode>) {
        if keys.contains(&VirtualKeyCode::Escape) {
            self.should_exit = true;
        }
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }
}
