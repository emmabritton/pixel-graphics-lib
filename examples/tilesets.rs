use anyhow::Result;
use buffer_graphics_lib::color::{BLUE, LIGHT_GRAY};
use buffer_graphics_lib::image::Image;
use buffer_graphics_lib::image_loading::tilesets::BasicTileset;
use buffer_graphics_lib::shapes::{stroke, Shape};
use buffer_graphics_lib::Graphics;
use pixels_graphics_lib::{setup, WindowScaling};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;

/// This example shows how to use BasicTileset and handle user keyboard input

fn main() -> Result<()> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let (window, mut pixels) = setup(
        (300, 300),
        WindowScaling::None,
        "Tileset Example",
        &event_loop,
    )?;

    let mut scene = TilesetScene::new("examples/resources/num_set.json")?;

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            let mut graphics = Graphics::new(pixels.get_frame(), 300, 300).unwrap();
            scene.render(&mut graphics);
            if pixels
                .render()
                .map_err(|e| eprintln!("pixels.render() failed: {:?}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
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
    one_shape: Shape,
    two_shape: Shape,
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
            active: true,
            one_shape: Shape::rect((0, 0), (18, 18), stroke(BLUE)),
            two_shape: Shape::circle((0, 0), 9, stroke(BLUE)),
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
            self.two_shape = self.two_shape.move_to(self.two_pos).translate_by((9, 9))
        }
    }

    fn render(&self, graphics: &mut Graphics<'_>) {
        graphics.clear(LIGHT_GRAY);
        graphics.draw_image(self.one_pos, &self.one);
        graphics.draw_image(self.two_pos, &self.two);
        if self.active {
            graphics.draw_at(self.one_pos, &self.one_shape);
        } else {
            graphics.draw(&self.two_shape);
        }
    }
}
