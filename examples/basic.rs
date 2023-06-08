use anyhow::Result;
use buffer_graphics_lib::color::Color;
use buffer_graphics_lib::Graphics;
use pixels_graphics_lib::prelude::*;
use winit::event::VirtualKeyCode;

/// This example shows the minimum code needed to use the library

fn main() -> Result<()> {
    let system = Box::new(Basic::new());
    run(240, 160, "Basic Example", system, Options::default())?;
    Ok(())
}

struct Basic {
    greyscale: u8,
    should_exit: bool,
}

impl Basic {
    pub fn new() -> Self {
        Basic {
            greyscale: 0,
            should_exit: false,
        }
    }
}

impl System for Basic {
    fn action_keys(&mut self) -> &[VirtualKeyCode] {
        &[VirtualKeyCode::Escape]
    }

    fn update(&mut self, _delta: &Timing) {
        if self.greyscale < 255 {
            self.greyscale += 1;
        } else {
            self.greyscale = 0;
        }
    }

    fn render(&mut self, graphics: &mut Graphics) {
        graphics.clear(Color::gray(self.greyscale))
    }

    fn on_key_down(&mut self, keys: Vec<VirtualKeyCode>) {
        if keys.contains(&VirtualKeyCode::Escape) {
            self.should_exit = true;
        }
    }

    fn should_exit(&mut self) -> bool {
        self.should_exit
    }
}
