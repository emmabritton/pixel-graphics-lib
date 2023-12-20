//! Rust Graphics Lib
//!
//! This is a simple wrapper around [`Pixels`](https://github.com/parasyte/pixels), it provides basic shape, image and text rendering.
//!
//! This boilerplate code is needed to use it:
//!
//! ```
//! # use std::error::Error;
//! # use pixels_graphics_lib::prelude::*;
//! # use buffer_graphics_lib::Graphics;
//! # use simple_game_utils::prelude::Timing;
//!
//! struct Example {
//! }
//!
//! impl System for Example {
//!   fn update(&mut self, delta: &Timing) {
//!
//!   }
//!
//!   fn render(&mut self, graphics: &mut Graphics) {
//!
//!   }
//! }
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//! let system = Example {};
//!   run(240, 160, "Doc Example", Box::new(system), Options::default())?;
//!   Ok(())
//! }
//!```

#![deny(clippy::all)]
#![allow(clippy::ptr_arg)]

pub mod dialogs;
#[cfg(feature = "window_prefs")]
pub mod prefs;
pub mod scenes;
pub mod ui;
pub mod utilities;

use crate::prefs::WindowPreferences;
use crate::prelude::ALL_KEYS;
use crate::ui::styles::UiStyle;
use crate::GraphicsError::LoadingWindowPref;
pub use buffer_graphics_lib;
use buffer_graphics_lib::Graphics;
pub use graphics_shapes;
use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use simple_game_utils::prelude::*;
use thiserror::Error;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::{CursorGrabMode, Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

pub mod prelude {
    pub use crate::dialogs::*;
    pub use crate::prefs::*;
    pub use crate::scenes::*;
    pub use crate::utilities::virtual_key_codes::*;
    pub use crate::*;
    pub use simple_game_utils::prelude::*;
    pub use winit::keyboard::KeyCode;
}

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
    options: &Options,
    title: &str,
    event_loop: &EventLoop<()>,
) -> Result<(Window, Pixels), GraphicsError> {
    let win = create_window(canvas_size, title, options.scaling, event_loop)?;
    let surface = SurfaceTexture::new(win.inner_size().width, win.inner_size().height, &win);
    let pixels = PixelsBuilder::new(canvas_size.0 as u32, canvas_size.1 as u32, surface)
        .enable_vsync(options.vsync)
        .build()
        .map_err(GraphicsError::PixelsInit)?;
    Ok((win, pixels))
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
        .map_err(|err| GraphicsError::WindowInit(format!("{err:?}")))?;
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

    window.set_min_inner_size(Some(px_size));
    let _ = window.request_inner_size(px_size);
    window.set_visible(true);

    Ok(window)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum MouseButton {
    Left,
    Right,
}

#[allow(unused_variables)]
pub trait System {
    /// List of keys that your app uses
    fn keys_used(&self) -> &[KeyCode] {
        &ALL_KEYS
    }
    #[cfg(feature = "window_prefs")]
    fn window_prefs(&mut self) -> Option<WindowPreferences> {
        None
    }
    fn update(&mut self, timing: &Timing);
    fn render(&mut self, graphics: &mut Graphics);
    fn on_mouse_move(&mut self, x: usize, y: usize) {}
    fn on_mouse_down(&mut self, x: usize, y: usize, button: MouseButton) {}
    fn on_mouse_up(&mut self, x: usize, y: usize, button: MouseButton) {}
    fn on_scroll(&mut self, x: usize, y: usize, x_diff: isize, y_diff: isize) {}
    fn on_key_down(&mut self, keys: Vec<KeyCode>) {}
    fn on_key_up(&mut self, keys: Vec<KeyCode>) {}
    fn on_window_closed(&mut self) {}
    fn on_visibility_changed(&mut self, visible: bool) {}
    fn on_focus_changed(&mut self, focused: bool) {}
    fn should_exit(&mut self) -> bool {
        false
    }
}

/// Options for programs
#[derive(Debug)]
pub struct Options {
    /// Target and max number of times [Scene::update] can be called per second
    /// Default is 240
    pub ups: usize,
    /// How the window should be scaled
    /// Default is [Auto][WindowScaling::Auto]
    pub scaling: WindowScaling,
    /// If vsync should be enabled
    /// Default is true
    pub vsync: bool,
    /// If OS mouse cursor should be hidden
    /// (you'll have to draw your own if this is true, this is often called software cursor in other programs)
    /// Default is false
    pub hide_cursor: bool,
    /// If the mouse cursor should be locked to within this window while it's in the foreground
    /// Default is false
    pub confine_cursor: bool,
    /// Style data for [UiElement]
    pub style: UiStyle,
}

impl Options {
    pub fn new(
        ups: usize,
        scaling: WindowScaling,
        vsync: bool,
        hide_cursor: bool,
        confine_cursor: bool,
        style: UiStyle,
    ) -> Self {
        Self {
            ups,
            scaling,
            vsync,
            hide_cursor,
            confine_cursor,
            style,
        }
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            ups: 240,
            scaling: WindowScaling::Auto,
            vsync: true,
            hide_cursor: false,
            confine_cursor: false,
            style: UiStyle::default(),
        }
    }
}

/// Helper method that sets up the screen and runs the loop
///
/// If you want to use [Scene][scenes::Scene]s consider [run_scenes][scenes::run_scenes]
///
/// # Arguments
/// * `width` - Width of the whole window canvas in pixels
/// * `height` - Height of the whole window canvas in pixels
/// * `title` - Window title
/// * `system` - Your program
/// * `options` - [Options] controls how fast the program can update, [UiElement] styling, etc
pub fn run(
    width: usize,
    height: usize,
    title: &str,
    mut system: Box<dyn System>,
    options: Options,
) -> Result<(), GraphicsError> {
    let event_loop = EventLoop::new().expect("Failed to setup event loop");
    let mut input = WinitInputHelper::new();
    let (mut window, mut pixels) = setup((width, height), &options, title, &event_loop)?;

    if options.confine_cursor {
        #[cfg(target_os = "macos")]
        let _ = window.set_cursor_grab(CursorGrabMode::Locked);
        #[cfg(not(target_os = "macos"))]
        let _ = window.set_cursor_grab(CursorGrabMode::Confined);
    }

    if options.hide_cursor {
        window.set_cursor_visible(false);
    }

    #[cfg(feature = "window_prefs")]
    if let Some(mut prefs) = system.window_prefs() {
        prefs.load().map_err(|e| LoadingWindowPref(e.to_string()))?;
        prefs.restore(&mut window);
    }

    let mut timing = Timing::new(options.ups);
    let mut mouse_x = 0;
    let mut mouse_y = 0;

    event_loop
        .run(move |event, target| {
            timing.update();
            match &event {
                Event::LoopExiting => {
                    system.on_window_closed();
                    #[cfg(feature = "window_prefs")]
                    if let Some(mut prefs) = system.window_prefs() {
                        prefs.store(&window);
                        //can't return from here so just print out error
                        let _ = prefs
                            .save()
                            .map_err(|err| eprintln!("Unable to save prefs: {err:?}"));
                    }
                }
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Occluded(hidden) => system.on_visibility_changed(!hidden),
                    WindowEvent::Focused(focused) => system.on_focus_changed(*focused),
                    WindowEvent::RedrawRequested => {
                        let mut graphics =
                            Graphics::new(pixels.frame_mut(), width, height).unwrap();
                        system.render(&mut graphics);
                        timing.renders += 1;
                        if pixels
                            .render()
                            .map_err(|e| eprintln!("pixels.render() failed: {e:?}"))
                            .is_err()
                        {
                            system.on_window_closed();
                            target.exit();
                            return;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }

            timing.accumulated_time += timing.delta;
            while timing.accumulated_time >= timing.fixed_time_step {
                system.update(&timing);
                timing.accumulated_time -= timing.fixed_time_step;
                timing.updates += 1;
            }

            if input.update(&event) {
                if input.close_requested() || input.destroyed() {
                    system.on_window_closed();
                    target.exit();
                    return;
                }

                if let Some(size) = input.window_resized() {
                    pixels
                        .resize_surface(size.width, size.height)
                        .expect("Unable to resize buffer");
                }

                if let Some(mc) = input.cursor() {
                    let (x, y) = pixels
                        .window_pos_to_pixel(mc)
                        .unwrap_or_else(|pos| pixels.clamp_pixel_pos(pos));
                    mouse_x = x;
                    mouse_y = y;
                    system.on_mouse_move(x, y);
                }

                let mut held_buttons = vec![];
                for button in system.keys_used() {
                    if input.key_held(*button) {
                        held_buttons.push(*button);
                    }
                }
                system.on_key_down(held_buttons);

                let mut released_buttons = vec![];
                for button in system.keys_used() {
                    if input.key_released(*button) {
                        released_buttons.push(*button);
                    }
                }
                system.on_key_up(released_buttons);

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
                let scroll = input.scroll_diff();
                if scroll.0 != 0.0 || scroll.1 != 0.0 {
                    system.on_scroll(
                        mouse_x,
                        mouse_y,
                        scroll.0.trunc() as isize,
                        scroll.1.trunc() as isize,
                    );
                }

                window.request_redraw();
            }

            if system.should_exit() {
                target.exit();
            }

            timing.update_fps();

            timing.last = timing.now;
        })
        .expect("Error when executing event loop");

    Ok(())
}
