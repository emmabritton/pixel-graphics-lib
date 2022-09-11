use anyhow::Result;
use buffer_graphics_lib::color::Color;
use buffer_graphics_lib::Graphics;
use pixels_graphics_lib::{run, System, WindowScaling};
use winit::event::VirtualKeyCode;

/// This example shows the minimum code needed to use the library

fn main() -> Result<()> {
    let system = Box::new(Basic::new());
    run(240, 160, WindowScaling::Auto, "Basic Example", system)?;
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
    fn action_keys(&self) -> Vec<VirtualKeyCode> {
        vec![VirtualKeyCode::Escape]
    }

    fn update(&mut self, _delta: f32) {
        if self.greyscale < 255 {
            self.greyscale += 1;
        } else {
            self.greyscale = 0;
        }
    }

    fn render(&self, graphics: &mut Graphics) {
        graphics.clear(Color::gray(self.greyscale))
    }

    fn on_key_down(&mut self, _keys: Vec<VirtualKeyCode>) {
        self.should_exit = true;
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }
}
