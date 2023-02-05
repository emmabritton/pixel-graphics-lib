//! Rust Graphics Lib
//!
//! This is a simple wrapper around [`Pixels`](https://github.com/parasyte/pixels), it provides basic shape, image and text rendering.
//!
//! This boilerplate code is needed to use it:
//!
//! ```
//! # use std::error::Error;
//! use pixels_graphics_lib::prelude::*;
//!
//! struct Example {
//! }
//!
//! impl System for Example {
//!   fn update(&mut self, delta: &Timing) {
//!
//!   }
//!
//!   fn render(&self, graphics: &mut Graphics) {
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
use crate::ui::styles::UiStyle;
use crate::GraphicsError::LoadingWindowPref;
pub use buffer_graphics_lib;
use buffer_graphics_lib::Graphics;
pub use graphics_shapes;
use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use std::time::Instant;
use thiserror::Error;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{CursorGrabMode, Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

pub mod prelude {
    pub use crate::dialogs::*;
    pub use crate::scenes::*;
    pub use crate::ui::*;
    pub use crate::*;
    pub use buffer_graphics_lib::prelude::*;
    pub use winit::event::VirtualKeyCode;
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

    window.set_inner_size(px_size);
    window.set_min_inner_size(Some(px_size));
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
    fn action_keys(&self) -> Vec<VirtualKeyCode> {
        vec![]
    }
    #[cfg(feature = "window_prefs")]
    fn window_prefs(&self) -> Option<WindowPreferences> {
        None
    }
    fn update(&mut self, delta: &Timing);
    fn render(&self, graphics: &mut Graphics);
    fn on_mouse_move(&mut self, x: usize, y: usize) {}
    fn on_mouse_down(&mut self, x: usize, y: usize, button: MouseButton) {}
    fn on_mouse_up(&mut self, x: usize, y: usize, button: MouseButton) {}
    fn on_scroll(&mut self, diff: isize) {}
    fn on_key_pressed(&mut self, keys: Vec<VirtualKeyCode>) {}
    fn on_key_down(&mut self, keys: Vec<VirtualKeyCode>) {}
    fn on_key_up(&mut self, keys: Vec<VirtualKeyCode>) {}
    fn on_window_closed(&mut self) {}
    fn on_visibility_changed(&mut self, visible: bool) {}
    fn on_focus_changed(&mut self, focused: bool) {}
    fn should_exit(&self) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct Stats {
    pub fps: usize,
    pub(crate) last_frame_count: usize,
    pub(crate) last_frame_check: Instant,
}

#[derive(Debug)]
pub struct Timing {
    /// time factor for lerp, etc
    pub delta: f64,
    /// when execution started
    pub started_at: Instant,
    /// time at start of frame
    pub now: Instant,
    /// time at start of last frame
    pub last: Instant,
    /// number of updates so far
    pub updates: usize,
    /// number of renders so far
    pub renders: usize,
    accumulated_time: f64,
    max_render_time: f64,
    pub fixed_time_step: f64,
    pub fixed_time_step_f32: f32,
    pub stats: Stats,
}

impl Timing {
    pub(crate) fn new(speed: usize) -> Timing {
        Timing {
            delta: 0.0,
            started_at: Instant::now(),
            now: Instant::now(),
            last: Instant::now(),
            updates: 0,
            renders: 0,
            accumulated_time: 0.0,
            max_render_time: 0.1,
            fixed_time_step: 1.0 / (speed as f64),
            fixed_time_step_f32: 1.0 / (speed as f32),
            stats: Stats {
                fps: 0,
                last_frame_count: 0,
                last_frame_check: Instant::now(),
            },
        }
    }

    pub(crate) fn update_fps(&mut self) {
        if self
            .now
            .duration_since(self.stats.last_frame_check)
            .as_secs_f32()
            >= 1.0
        {
            self.stats.fps = self.renders - self.stats.last_frame_count;
            self.stats.last_frame_check = self.now;
            self.stats.last_frame_count = self.renders;
        }
    }

    pub(crate) fn update(&mut self) {
        self.now = Instant::now();
        self.delta = self.now.duration_since(self.last).as_secs_f64();
        if self.delta > self.max_render_time {
            self.delta = self.max_render_time;
        }
    }
}

#[derive(Debug)]
pub struct Options {
    pub ups: usize,
    pub scaling: WindowScaling,
    pub vsync: bool,
    pub hide_cursor: bool,
    pub confine_cursor: bool,
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

pub fn run(
    width: usize,
    height: usize,
    title: &str,
    mut system: Box<dyn System>,
    options: Options,
) -> Result<(), GraphicsError> {
    let event_loop = EventLoop::new();
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

    event_loop.run(move |event, _, control_flow| {
        timing.update();
        match &event {
            Event::LoopDestroyed => {
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
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                let mut graphics = Graphics::new(pixels.get_frame_mut(), width, height).unwrap();
                system.render(&mut graphics);
                timing.renders += 1;
                if pixels
                    .render()
                    .map_err(|e| eprintln!("pixels.render() failed: {e:?}"))
                    .is_err()
                {
                    system.on_window_closed();
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Occluded(hidden) => system.on_visibility_changed(!*hidden),
                WindowEvent::Focused(focused) => system.on_focus_changed(*focused),
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
            if input.quit() {
                system.on_window_closed();
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.window_resized() {
                pixels
                    .resize_surface(size.width, size.height)
                    .expect("Unable to resize buffer");
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
            let scroll = input.scroll_diff();
            if scroll != 0.0 {
                system.on_scroll(scroll.trunc() as isize);
            }

            window.request_redraw();
        }

        if system.should_exit() {
            *control_flow = ControlFlow::Exit;
        }

        timing.update_fps();

        timing.last = timing.now;
    });
}
