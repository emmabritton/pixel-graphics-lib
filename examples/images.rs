use anyhow::Result;
use buffer_graphics_lib::color::BLACK;
use buffer_graphics_lib::image::Image;
use buffer_graphics_lib::image_loading::load_image;
use buffer_graphics_lib::scaling::Scaling;
use buffer_graphics_lib::{Graphics, Tint};
use pixels_graphics_lib::{setup, WindowScaling};
use std::rc::Rc;
use std::time::Instant;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;

/// This example shows how to load, display and alter an image
/// It also shows an example of how to do delta

fn main() -> Result<()> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let (window, mut pixels) = setup(
        (300, 300),
        WindowScaling::Fixed(2),
        "Image Example",
        &event_loop,
    )?;
    let mut time = Instant::now();

    let mut scene = ImageScene::new("examples/resources/marker.png", 300, 300)?;

    event_loop.run(move |event, _, control_flow| {
        let now = Instant::now();
        let delta = now.duration_since(time).as_secs_f32();
        time = now;
        let mut graphics = Graphics::new(pixels.get_frame(), 300, 300).unwrap();
        if let Event::RedrawRequested(_) = event {
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

        scene.update(delta);

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            //put your input handling code here

            window.request_redraw();
        }
    });
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
}

impl ImageScene {
    pub fn new(path: &str, width: usize, height: usize) -> Result<Self> {
        let image = load_image(path)?;
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
        })
    }
}

impl ImageScene {
    fn update(&mut self, delta: f32) {
        let sw = self.width;
        let sh = self.height;

        let is_off_screen = |sprite: &Sprite| {
            sprite.pos.0 < -(sprite.size.0 as f32)
                || sprite.pos.1 < -(sprite.size.1 as f32)
                || sprite.pos.0 > (sw + sprite.size.0) as f32
                || sprite.pos.1 > (sh + sprite.size.1) as f32
        };

        for sprite in self.sprites.iter_mut() {
            sprite.pos.0 += sprite.dir.0 * sprite.speed * delta;
            sprite.pos.1 += sprite.dir.1 * sprite.speed * delta;
            if is_off_screen(sprite) {
                sprite.pos = (fastrand::f32() * sw as f32, fastrand::f32() * sh as f32);
            }
        }
    }

    fn render(&self, graphics: &mut Graphics<'_>) {
        graphics.clear(BLACK);
        for sprite in &self.sprites {
            graphics.draw_image((sprite.pos.0.round(), sprite.pos.1.round()), &sprite.image);
        }
    }
}
