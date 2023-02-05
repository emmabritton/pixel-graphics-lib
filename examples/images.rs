use anyhow::Result;
use buffer_graphics_lib::color::BLACK;
use buffer_graphics_lib::image::Image;
use buffer_graphics_lib::scaling::Scaling;
use buffer_graphics_lib::{Graphics, Tint};
use pixels_graphics_lib::prelude::*;
use std::rc::Rc;
use winit::event::VirtualKeyCode;

/// This example shows how to load, display and alter an image
/// It also shows an example of how to do delta

fn main() -> Result<()> {
    let width = 300;
    let height = 300;
    let system = ImageScene::new("examples/resources/marker.png", width, height)?;
    run(
        width,
        height,
        "Image Example",
        Box::new(system),
        Options::default(),
    )?;
    Ok(())
}

struct Sprite {
    pos: (f32, f32),
    size: (usize, usize),
    image: Rc<Image>,
    dir: (f32, f32),
    speed: f32,
}

impl Sprite {
    pub fn new(image: Rc<Image>) -> Self {
        Sprite {
            pos: (
                fastrand::isize(100..200) as f32,
                fastrand::isize(100..200) as f32,
            ),
            size: (image.width(), image.height()),
            image,
            dir: (fastrand::f32() * 2.0 - 1.0, fastrand::f32() * 2.0 - 1.0),
            speed: fastrand::f32() * 100.0,
        }
    }
}

struct ImageScene {
    width: usize,
    height: usize,
    sprites: Vec<Sprite>,
    should_exit: bool,
}

impl ImageScene {
    pub fn new(path: &str, width: usize, height: usize) -> Result<Self> {
        let image = open_image(path)?;
        let mut red = image.clone();
        red.tint_mul(1.0, 0.0, 0.0, 1.0); //the original image is white so set the green and blue channels to 0
        let mut double_blue = image.scale(Scaling::Epx2x);
        double_blue.tint_add(-255, -255, 0, 0);

        let image = Rc::new(image);
        let red = Rc::new(red);
        let blue = Rc::new(double_blue);

        let mut sprites = vec![];

        for _ in 0..10 {
            sprites.push(Sprite::new(image.clone()));
            sprites.push(Sprite::new(red.clone()));
            sprites.push(Sprite::new(blue.clone()));
        }

        Ok(ImageScene {
            width,
            height,
            sprites,
            should_exit: false,
        })
    }
}

impl System for ImageScene {
    fn action_keys(&self) -> Vec<VirtualKeyCode> {
        vec![VirtualKeyCode::Escape]
    }

    fn update(&mut self, timing: &Timing) {
        let sw = self.width;
        let sh = self.height;

        let is_off_screen = |sprite: &Sprite| {
            sprite.pos.0 < -(sprite.size.0 as f32)
                || sprite.pos.1 < -(sprite.size.1 as f32)
                || sprite.pos.0 > (sw + sprite.size.0) as f32
                || sprite.pos.1 > (sh + sprite.size.1) as f32
        };

        for sprite in self.sprites.iter_mut() {
            sprite.pos.0 += sprite.dir.0 * sprite.speed * timing.fixed_time_step_f32;
            sprite.pos.1 += sprite.dir.1 * sprite.speed * timing.fixed_time_step_f32;
            if is_off_screen(sprite) {
                sprite.pos = (fastrand::f32() * sw as f32, fastrand::f32() * sh as f32);
            }
        }
    }

    fn render(&self, graphics: &mut Graphics) {
        graphics.clear(BLACK);
        for sprite in &self.sprites {
            graphics.draw_image((sprite.pos.0.round(), sprite.pos.1.round()), &sprite.image);
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
