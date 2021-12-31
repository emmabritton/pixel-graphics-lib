//! Rust Graphics Lib
//!
//! This is a simple wrapper around [`Pixels`](https://github.com/parasyte/pixels), it provides basic shape, image and text rendering.
//!
//! This boilerplate code is needed to use it:
//!
//! ```
//! # use std::error::Error;
//! # use winit::event::{Event, VirtualKeyCode};
//! # use pixels_graphics_lib::setup;
//! # use winit_input_helper::WinitInputHelper;
//! # use winit::event_loop::{ControlFlow, EventLoop};
//!
//! # fn main() -> Result<(), Box<dyn Error>> {
//! let event_loop = EventLoop::new();
//! let mut input = WinitInputHelper::new();
//! let (mut window, mut graphics) = setup(240, 160, "Doc Example", true, &event_loop)?;
//!
//! # let mut loop_count = 0;
//!
//! event_loop.run(move |event, _, control_flow| {
//!     if let Event::RedrawRequested(_) = event {
//!         if graphics.pixels
//!         .render()
//!         .map_err(|e| eprintln ! ("pixels.render() failed: {:?}", e))
//!         .is_err()
//!         {
//!             *control_flow = ControlFlow::Exit;
//!             return;
//!         }
//!     }
//!
//!     //put your rendering code here
//!
//!     if input.update(&event) {
//!         if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
//!             *control_flow = ControlFlow::Exit;
//!             return;
//!         }
//!
//!         if let Some(size) = input.window_resized() {
//!             graphics.pixels.resize_surface(size.width, size.height);
//!         }
//!
//!         //put your update/input handling code here
//!
//!         window.request_redraw();
//!     }
//! # loop_count += 1;
//! # if loop_count > 3 {
//! #   *control_flow = ControlFlow::Exit;
//! # }
//! });
//!
//!    # Ok(())
//! # }
//!```
//!
//! Using the library is as simple as:
//!```
//! # use std::error::Error;
//! # use pixels_graphics_lib::drawing::PixelWrapper;
//! # use pixels_graphics_lib::setup;
//! # use pixels_graphics_lib::color::{BLUE, BLACK};
//! # use winit::event_loop::EventLoop;
//! # use pixels_graphics_lib::text::TextSize;
//! # fn main() -> Result<(), Box<dyn Error>> {
//! # let event_loop = EventLoop::new();
//! # let (window, mut graphics) = setup(240, 160, "Example", true, &event_loop)?;
//! graphics.draw_text("Some text", 9, 1, 1, TextSize::Normal, BLACK);
//! graphics.draw_line(30, 30, 100, 120, BLUE);
//! # Ok(()) }
//! ```

#![deny(clippy::all)]

pub mod color;
pub mod drawing;
pub mod image;
pub mod math;
pub mod scaling;
pub mod text;
#[cfg(feature = "image_loading")]
pub mod image_loading;
#[cfg(feature = "window_prefs")]
pub mod prefs;

use crate::drawing::PixelWrapper;
use pixels::{Pixels, SurfaceTexture};
use thiserror::Error;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

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
/// `scale` = `true`) then a library pixel will be represented by multiple display pixels
///
/// # Arguments
///
/// * `width` - Inner width of window in logical pixels
/// * `height` - Inner width of window in logical pixels
/// * `title` - Title for window
/// * `scale` - True if window should account for high DPI
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
    width: u32,
    height: u32,
    title: &str,
    scale: bool,
    event_loop: &EventLoop<()>,
) -> Result<(Window, PixelWrapper), GraphicsError> {
    let win = create_window(width, height, title, scale, event_loop)?;
    let surface = SurfaceTexture::new(width, height, &win);
    let pixels = Pixels::new(width, height, surface).map_err(GraphicsError::PixelsInit)?;

    Ok((
        win,
        PixelWrapper::new(pixels, width as usize, height as usize),
    ))
}

fn create_window(
    width: u32,
    height: u32,
    title: &str,
    scale: bool,
    event_loop: &EventLoop<()>,
) -> Result<Window, GraphicsError> {
    let window = WindowBuilder::new()
        .with_visible(false)
        .with_title(title)
        .build(event_loop)
        .map_err(|err| GraphicsError::WindowInit(format!("{:?}", err)))?;
    let width = width as f64;
    let height = height as f64;
    let hidpi_factor = if scale { window.scale_factor() } else { 1.0 };
    let size = LogicalSize::new(width * hidpi_factor, height * hidpi_factor);

    window.set_inner_size(size);
    window.set_min_inner_size(Some(size));
    window.set_visible(true);

    Ok(window)
}

pub trait Tint {
    /// Add to the RGBA channels by the amounts specified
    ///
    /// Channels are clamped to 0..=255
    fn tint_add(&mut self, r_diff: isize, g_diff: isize, b_diff: isize, a_diff: isize);
    /// Multiple the RGBA channels by the amounts specified
    ///
    /// Channels are clamped to 0..=255
    fn tint_mul(&mut self, r_diff: f32, g_diff: f32, b_diff: f32, a_diff: f32);
}
