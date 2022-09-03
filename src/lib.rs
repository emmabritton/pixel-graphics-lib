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
//!
//! # fn main() -> Result<(), Box<dyn Error>> {
//! let event_loop = EventLoop::new();
//! let mut input = WinitInputHelper::new();
//! let (mut window, mut pixels) = setup((240, 160), WindowScaling::Auto, "Doc Example", &event_loop)?;
//!
//! # let mut loop_count = 0;
//!
//! event_loop.run(move |event, _, control_flow| {
//!     if let Event::RedrawRequested(_) = event {
//!        //put your rendering code here
//!         if pixels
//!         .render()
//!         .map_err(|e| eprintln ! ("pixels.render() failed: {:?}", e))
//!         .is_err()
//!         {
//!             *control_flow = ControlFlow::Exit;
//!             return;
//!         }
//!     }
//!
//!
//!     if input.update(&event) {
//!         if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
//!             *control_flow = ControlFlow::Exit;
//!             return;
//!         }
//!
//!         if let Some(size) = input.window_resized() {
//!             pixels.resize_surface(size.width, size.height);
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

#![deny(clippy::all)]

#[cfg(feature = "window_prefs")]
pub mod prefs;

use pixels::{Pixels, SurfaceTexture};
use thiserror::Error;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};
#[cfg(target_os = "windows")]
use winit::platform::windows::WindowBuilderExtWindows;

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
    #[allow()(unused_mut)] //needs to be mut for windows
    let mut window_builder = WindowBuilder::new()
        .with_visible(false)
        .with_title(title);

    #[cfg(target_os = "windows")] {
        window_builder = window_builder.with_drag_and_drop(false);
    }

    let window = window_builder.build(event_loop)
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
