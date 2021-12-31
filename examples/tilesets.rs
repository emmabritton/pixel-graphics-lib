use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;
use pixels_graphics_lib::color::{BLUE, LIGHT_GRAY};
use pixels_graphics_lib::drawing::PixelWrapper;
use pixels_graphics_lib::setup;
use anyhow::Result;
use pixels_graphics_lib::image::Image;
use pixels_graphics_lib::image_loading::tilesets::BasicTileset;

/// This example shows how to use BasicTileset and handle user keyboard input

fn main() -> Result<()> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let (window, mut graphics) = setup(300, 300, "Tileset Example", true, &event_loop)?;

    let mut scene = TilesetScene::new("examples/resources/num_set.json")?;

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            if graphics.pixels
                .render()
                .map_err(|e| eprintln!("pixels.render() failed: {:?}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }

            scene.render(&mut graphics);
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.window_resized() {
                graphics.pixels.resize_surface(size.width, size.height);
            }

            scene.input(&input);

            window.request_redraw();
        }
    });
}


struct TilesetScene {
    one: Image,
    two: Image,
    one_pos: (isize, isize),
    two_pos: (isize, isize),
    active: bool,
}

impl TilesetScene {
    pub fn new(path: &str) -> Result<Self> {
        let mut images = BasicTileset::load_from_file(path)?;
        let one = images.remove("one").unwrap();
        let two = images.remove("two").unwrap();

        Ok(TilesetScene {
            one,
            two,
            one_pos: (100, 100),
            two_pos: (200, 100),
            active: true
        })
    }
}

impl TilesetScene {
    fn input(&mut self, input: &WinitInputHelper) {
        if input.key_pressed(VirtualKeyCode::Key1) {
            self.active = true;
        }
        if input.key_pressed(VirtualKeyCode::Key2) {
            self.active = false;
        }
        let mut diff = (0, 0);
        if input.key_held(VirtualKeyCode::Up) {
            diff.1 = -1;
        }
        if input.key_held(VirtualKeyCode::Down) {
            diff.1 = 1;
        }
        if input.key_held(VirtualKeyCode::Left) {
            diff.0 = -1;
        }
        if input.key_held(VirtualKeyCode::Right) {
            diff.0 = 1;
        }
        if self.active {
            self.one_pos.0 += diff.0;
            self.one_pos.1 += diff.1;
        } else {
            self.two_pos.0 += diff.0;
            self.two_pos.1 += diff.1;
        }
    }

    fn render(&self, graphics: &mut PixelWrapper) {
        graphics.clear(LIGHT_GRAY);
        graphics.draw_image(self.one_pos.0, self.one_pos.1, &self.one);
        graphics.draw_image(self.two_pos.0, self.two_pos.1, &self.two);
        if self.active {
            graphics.draw_frame(self.one_pos.0 - 1, self.one_pos.1 - 1, self.one_pos.0 + 18, self.one_pos.1 + 18, BLUE);
        } else {
            graphics.draw_frame(self.two_pos.0 - 1, self.two_pos.1 - 1, self.two_pos.0 + 16, self.two_pos.1 + 16, BLUE);
        }
    }
}