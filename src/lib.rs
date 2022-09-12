//! Rust Graphics Lib
//!
//! This is a simple wrapper around [`Pixels`](https://github.com/parasyte/pixels), it provides basic shape, image and text rendering.
//!
//! This boilerplate code is needed to use it:
//!
//! ```
//! # use std::error::Error;
//! # use winit::event::{Event, VirtualKeyCode};
//! # use pixels_graphics_lib::{WindowScaling, setup};
//! # use winit_input_helper::WinitInputHelper;
//! # use winit::event_loop::{ControlFlow, EventLoop};
//! # use pixels_graphics_lib::System;
//! # use buffer_graphics_lib::Graphics;
//! # use pixels_graphics_lib::run;
//!
//! struct Example {
//! }
//!
//! impl System for Example {
//!   fn update(&mut self, delta: f32) {
//!
//!   }
//!
//!   fn render(&self, graphics: &mut Graphics) {
//!
//!   }
//! }
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!   let system = Example {};
//!   run(240, 160, WindowScaling::Auto, "Doc Example", Box::new(system))?;
//!   Ok(())
//! }
//!```

#![deny(clippy::all)]

#[cfg(feature = "window_prefs")]
pub mod prefs;
pub mod utilities;

use crate::prefs::WindowPreferences;
use crate::GraphicsError::LoadingWindowPref;
use buffer_graphics_lib::Graphics;
use pixels::{Pixels, SurfaceTexture};
use std::thread::sleep;
use std::time::{Duration, Instant};
use thiserror::Error;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

#[derive(Error, Debug)]
pub enum GraphicsError {
    #[error("Creating a window: {0}")]
    WindowInit(String),
    #[error("Initialising Pixels: {0}")]
    PixelsInit(#[source] pixels::Error),
    #[error("Saving window pref: {0}")]
    SavingWindowPref(String),
    #[error("Loading window pref: {0}")]
    LoadingWindowPref(String),
    #[error("Invalid pixel array length, expected: {0}, found: {1}")]
    ImageInitSize(usize, usize),
    #[error("Both images must be the same size, expected: {0}x{1}, found: {2}x{3}")]
    ImageBlendSize(usize, usize, usize, usize),
}

/// Creates the window and pixels wrapper
///
/// The inner size mentioned in the arguments refers to the size of the area available to draw in, it doesn't include the window frame, etc
///
/// This uses logical pixels, where on a low DPI screen each library pixel is one display pixel but on higher DPI screens (and if
/// `scale` != `None`) then a library pixel will be represented by multiple display pixels
///
/// # Arguments
///
/// * `canvas_size` - Inner width and height of window in logical pixels
/// * `scale` - Type of scaling the window should use, see [WindowScaling]
/// * `title` - Title for window
/// * `event_loop` - Provided by `EventLoop::new()`, this allows the window to receive events from the OS
///
/// # Example
///
/// This creates a 160x160 window:
///
/// `let (mut window, graphics) = setup(160, 160, "Example", true, &event_loop)?;`
///
/// # Returns
///
/// A result with a pair of Window and PixelsWrapper
///
/// # Errors
///
/// * `WindowInit` - If the window can not be created
pub fn setup(
    canvas_size: (usize, usize),
    window_scaling: WindowScaling,
    title: &str,
    event_loop: &EventLoop<()>,
) -> Result<(Window, Pixels), GraphicsError> {
    let win = create_window(canvas_size, title, window_scaling, event_loop)?;
    let surface = SurfaceTexture::new(win.inner_size().width, win.inner_size().height, &win);
    let pixels = Pixels::new(canvas_size.0 as u32, canvas_size.1 as u32, surface)
        .map_err(GraphicsError::PixelsInit)?;

    Ok((win, pixels))
}

pub enum WindowScaling {
    /// Make the canvas and window be the same of numbers, this ignores DPI
    None,
    /// Scale the window to account for DPI
    Auto,
    /// Scale the window by a fixed amount, ignoring DPI
    Fixed(usize),
    /// Scale the window by a fixed amount and by DPI
    /// So, if the monitor DPI is 2x and 2x is passed then the result will be 4x
    AutoFixed(usize),
}

fn create_window(
    size: (usize, usize),
    title: &str,
    scale: WindowScaling,
    event_loop: &EventLoop<()>,
) -> Result<Window, GraphicsError> {
    let window = WindowBuilder::new()
        .with_visible(false)
        .with_title(title)
        .build(event_loop)
        .map_err(|err| GraphicsError::WindowInit(format!("{:?}", err)))?;
    let factor = match scale {
        WindowScaling::None => 1.,
        WindowScaling::Auto => window.scale_factor().ceil(),
        WindowScaling::Fixed(amount) => {
            if amount == 0 {
                return Err(GraphicsError::WindowInit(String::from(
                    "Fixed window scaling must be at least 1",
                )));
            }
            amount as f64
        }
        WindowScaling::AutoFixed(amount) => {
            if amount == 0 {
                return Err(GraphicsError::WindowInit(String::from(
                    "AutoFixed window scaling must be at least 1",
                )));
            }
            amount as f64 + window.scale_factor().ceil()
        }
    };

    let px_size = LogicalSize::new(size.0 as f64 * factor, size.1 as f64 * factor);

    window.set_inner_size(px_size);
    window.set_min_inner_size(Some(px_size));
    window.set_visible(true);

    Ok(window)
}

pub enum MouseButton {
    Left,
    Right,
}

#[allow(unused_variables)]
pub trait System {
    fn action_keys(&self) -> Vec<VirtualKeyCode> {
        vec![]
    }
    #[cfg(feature = "window_prefs")]
    fn window_prefs(&self) -> Option<WindowPreferences> {
        None
    }
    fn update(&mut self, delta: f32);
    fn render(&self, graphics: &mut Graphics);
    fn on_mouse_move(&mut self, x: usize, y: usize) {}
    fn on_mouse_down(&mut self, x: usize, y: usize, button: MouseButton) {}
    fn on_mouse_up(&mut self, x: usize, y: usize, button: MouseButton) {}
    fn on_key_pressed(&mut self, keys: Vec<VirtualKeyCode>) {}
    fn on_key_down(&mut self, keys: Vec<VirtualKeyCode>) {}
    fn on_key_up(&mut self, keys: Vec<VirtualKeyCode>) {}
    fn on_window_closed(&mut self) {}
    fn should_exit(&self) -> bool {
        false
    }
}

pub fn run(
    width: usize,
    height: usize,
    window_scaling: WindowScaling,
    title: &str,
    mut system: Box<dyn System>,
) -> Result<(), GraphicsError> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let (mut window, mut pixels) = setup((width, height), window_scaling, title, &event_loop)?;

    #[cfg(feature = "window_prefs")]
    if let Some(mut prefs) = system.window_prefs() {
        prefs.load().map_err(|e| LoadingWindowPref(e.to_string()))?;
        prefs.restore(&mut window);
    }

    let mut time = Instant::now();
    let mut mouse_x = 0;
    let mut mouse_y = 0;

    event_loop.run(move |event, _, control_flow| {
        let now = Instant::now();
        let delta = now.duration_since(time).as_secs_f32();
        time = now;
        if let Event::LoopDestroyed = event {
            system.on_window_closed();
            #[cfg(feature = "window_prefs")]
            if let Some(mut prefs) = system.window_prefs() {
                prefs.store(&window);
                //can't return from here so just print out error
                let _ = prefs
                    .save()
                    .map_err(|err| eprintln!("Unable to save prefs: {:?}", err));
            }
        }
        if let Event::RedrawRequested(_) = event {
            let mut graphics = Graphics::new(pixels.get_frame(), width, height).unwrap();
            system.render(&mut graphics);
            if pixels
                .render()
                .map_err(|e| eprintln!("pixels.render() failed: {:?}", e))
                .is_err()
            {
                system.on_window_closed();
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        system.update(delta);

        if input.update(&event) {
            if input.quit() {
                system.on_window_closed();
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            if let Some(mc) = input.mouse() {
                let (x, y) = pixels
                    .window_pos_to_pixel(mc)
                    .unwrap_or_else(|pos| pixels.clamp_pixel_pos(pos));
                mouse_x = x;
                mouse_y = y;
                system.on_mouse_move(x, y);
            }

            let mut held_buttons = vec![];
            for button in system.action_keys() {
                if input.key_held(button) {
                    held_buttons.push(button);
                }
            }
            system.on_key_down(held_buttons);

            let mut released_buttons = vec![];
            for button in system.action_keys() {
                if input.key_released(button) {
                    released_buttons.push(button);
                }
            }
            system.on_key_up(released_buttons);

            let mut typed_buttons = vec![];
            for button in system.action_keys() {
                if input.key_pressed(button) {
                    typed_buttons.push(button);
                }
            }
            system.on_key_pressed(typed_buttons);

            if input.mouse_held(0) {
                system.on_mouse_down(mouse_x, mouse_y, MouseButton::Left);
            }
            if input.mouse_held(1) {
                system.on_mouse_down(mouse_x, mouse_y, MouseButton::Right);
            }

            if input.mouse_released(0) {
                system.on_mouse_up(mouse_x, mouse_y, MouseButton::Left);
            }
            if input.mouse_released(1) {
                system.on_mouse_up(mouse_x, mouse_y, MouseButton::Right);
            }

            window.request_redraw();
        }

        if system.should_exit() {
            *control_flow = ControlFlow::Exit;
        }

        sleep(Duration::from_millis(1));
    });
}
