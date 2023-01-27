use anyhow::Result;
use buffer_graphics_lib::color::{BLUE, LIGHT_GRAY, RED};
use buffer_graphics_lib::drawable::{stroke, Drawable};
use buffer_graphics_lib::image::Image;
use buffer_graphics_lib::image_loading::tilesets::BasicTileset;
use buffer_graphics_lib::shapes::CreateDrawable;
use buffer_graphics_lib::text::format::Positioning::RightTop;
use buffer_graphics_lib::text::pos::TextPos;
use buffer_graphics_lib::text::TextSize::Normal;
use buffer_graphics_lib::Graphics;
use graphics_shapes::circle::Circle;
use graphics_shapes::rect::Rect;
use pixels_graphics_lib::prelude::*;
use winit::event::VirtualKeyCode;

/// This example shows how to use BasicTileset and handle user keyboard input

fn main() -> Result<()> {
    let system = TilesetScene::new("examples/resources/num_set.json")?;
    run(
        300,
        300,
        WindowScaling::None,
        "Tileset Example",
        Box::new(system),
        ExecutionSpeed::standard(),
    )?;
    Ok(())
}

struct TilesetScene {
    one: Image,
    two: Image,
    one_pos: (isize, isize),
    two_pos: (isize, isize),
    one_shape: Drawable<Rect>,
    two_shape: Drawable<Circle>,
    active: bool,
    should_exit: bool,
    fps: usize,
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
            one_shape: Drawable::from_obj(Rect::new((0, 0), (18, 18)), stroke(BLUE)),
            two_shape: Drawable::from_obj(Circle::new((0, 0), 9), stroke(BLUE)),
            should_exit: false,
            fps: 0,
        })
    }
}

impl System for TilesetScene {
    fn action_keys(&self) -> Vec<VirtualKeyCode> {
        vec![
            VirtualKeyCode::Escape,
            VirtualKeyCode::Key1,
            VirtualKeyCode::Key2,
            VirtualKeyCode::Up,
            VirtualKeyCode::Down,
            VirtualKeyCode::Left,
            VirtualKeyCode::Right,
        ]
    }

    fn update(&mut self, timing: &Timing) {
        self.fps = timing.stats.fps;
    }

    fn render(&self, graphics: &mut Graphics) {
        graphics.clear(LIGHT_GRAY);
        graphics.draw_image(self.one_pos, &self.one);
        graphics.draw_image(self.two_pos, &self.two);
        if self.active {
            graphics.draw_offset(self.one_pos, &self.one_shape);
        } else {
            graphics.draw(&self.two_shape);
        }
        graphics.draw_text(
            &format!("{}", self.fps),
            TextPos::px((299, 1)),
            (RED, Normal, RightTop),
        );
    }

    fn on_key_up(&mut self, keys: Vec<VirtualKeyCode>) {
        if keys.contains(&VirtualKeyCode::Key1) {
            self.active = true;
        }
        if keys.contains(&VirtualKeyCode::Key2) {
            self.active = false;
        }

        let mut diff = (0, 0);
        if keys.contains(&VirtualKeyCode::Up) {
            diff.1 = -1;
        }
        if keys.contains(&VirtualKeyCode::Down) {
            diff.1 = 1;
        }
        if keys.contains(&VirtualKeyCode::Left) {
            diff.0 = -1;
        }
        if keys.contains(&VirtualKeyCode::Right) {
            diff.0 = 1;
        }
        if self.active {
            self.one_pos.0 += diff.0;
            self.one_pos.1 += diff.1;
        } else {
            self.two_pos.0 += diff.0;
            self.two_pos.1 += diff.1;
            self.two_shape = self
                .two_shape
                .with_move(self.two_pos)
                .with_translation((9, 9))
        }
        if keys.contains(&VirtualKeyCode::Escape) {
            self.should_exit = true;
        }
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }
}
