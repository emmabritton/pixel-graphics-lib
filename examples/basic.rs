use std::thread::sleep;
use std::time::Duration;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;
use pixels_graphics_lib::color::Color;
use pixels_graphics_lib::drawing::PixelWrapper;
use pixels_graphics_lib::{WindowScaling, setup};
use anyhow::Result;

/// This example shows the minimum code needed to use the library

fn main() -> Result<()> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let (window, mut graphics) = setup((240, 160), WindowScaling::Auto, "Basic Example", &event_loop)?;

    let mut basic = Basic::new();

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            if graphics.pixels
                .render()
                .map_err( | e | eprintln ! ("pixels.render() failed: {:?}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }

            basic.render(&mut graphics);
        }

        basic.update();

        if input.update( & event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.window_resized() {
                graphics.pixels.resize_surface(size.width, size.height);
            }

            //put your input handling code here

            window.request_redraw();
        }

        sleep(Duration::from_millis(1));
    });
}

struct Basic {
    greyscale: u8
}

impl Basic {
    pub fn new() -> Self {
        Basic { greyscale: 0 }
    }
}

impl Basic {
    fn update(&mut self) {
        if self.greyscale < 255 {
            self.greyscale += 1;
        } else {
            self.greyscale = 0;
        }
    }

    fn render(&self, graphics: &mut PixelWrapper) {
        graphics.clear(Color::gray(self.greyscale))
    }
}