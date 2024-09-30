//! Rust Graphics Lib
//!
//! This is a simple pixel graphics and GUI library, it provides basic shape, image and text rendering.
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
//!   fn update(&mut self, timing: &Timing, _: &Window) {
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

#[cfg(all(not(feature = "pixels"), not(feature = "softbuffer"),))]
compile_error!("You must pick one windowing feature either pixels or softbuffer");

pub mod dialogs;
mod integration;
#[cfg(feature = "scenes")]
pub mod scenes;
pub mod ui;
pub mod utilities;
#[cfg(feature = "window_prefs")]
pub mod window_prefs;

use crate::prelude::{winit, Coord, ALL_KEYS};
use crate::ui::styles::UiStyle;
#[cfg(feature = "window_prefs")]
use crate::window_prefs::WindowPreferences;
pub use buffer_graphics_lib;
use buffer_graphics_lib::Graphics;
use rustc_hash::FxHashMap;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use simple_game_utils::prelude::*;
use thiserror::Error;
use winit::event::MouseButton;
#[cfg(feature = "softbuffer")]
pub use winit::event_loop::ControlFlow;
use winit::keyboard::KeyCode;
use winit::window::Window;

pub mod prelude {
    pub use crate::dialogs::*;
    #[cfg(feature = "pixels")]
    pub use crate::integration::pixels_winit::run;
    #[cfg(feature = "softbuffer")]
    pub use crate::integration::softbuffer_winit::run;
    #[cfg(feature = "scenes")]
    pub use crate::scenes::*;
    pub use crate::utilities::virtual_key_codes::*;
    #[cfg(feature = "window_prefs")]
    pub use crate::window_prefs::*;
    pub use crate::GraphicsError;
    pub use crate::MouseData;
    pub use crate::Options;
    pub use crate::System;
    pub use crate::WindowScaling;
    pub use buffer_graphics_lib::prelude::*;
    pub use rustc_hash::FxHashSet;
    pub use simple_game_utils::prelude::*;
    pub use winit::event::MouseButton;
    pub use winit::keyboard::KeyCode;
    pub use winit::window::Window;
    #[cfg(feature = "pixels")]
    pub use winit_29 as winit;
    #[cfg(feature = "softbuffer")]
    pub use winit_30 as winit;
}

#[derive(Error, Debug)]
pub enum GraphicsError {
    #[error("Creating a window: {0}")]
    WindowInit(String),
    #[cfg(any(feature = "pixels"))]
    #[error("Initialising Pixels: {0}")]
    PixelsInit(#[source] pixels::Error),
    #[error("Saving window pref: {0}")]
    SavingWindowPref(String),
    #[cfg(feature = "window_prefs")]
    #[error("Loading window pref: {0}")]
    LoadingWindowPref(String),
    #[error("Invalid pixel array length, expected: {0}, found: {1}")]
    ImageInitSize(usize, usize),
    #[error("Both images must be the same size, expected: {0}x{1}, found: {2}x{3}")]
    ImageBlendSize(usize, usize, usize, usize),
    #[cfg(feature = "controller")]
    #[error("Unable to init controller: {0}")]
    ControllerInit(String),
    #[cfg(any(feature = "pixels", feature = "softbuffer"))]
    #[error("Initialing Winit: {0}")]
    WinitInit(#[source] winit::error::EventLoopError),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum WindowScaling {
    /// Use system DPI
    Native,
    /// Use system DPI + 2
    Double,
    /// Use system DPI + 4
    Quad,
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
    fn update(&mut self, timing: &Timing, window: &Window);
    fn render(&mut self, graphics: &mut Graphics);
    fn on_mouse_move(&mut self, mouse: &MouseData) {}
    fn on_mouse_down(&mut self, mouse: &MouseData, button: MouseButton) {}
    fn on_mouse_up(&mut self, mouse: &MouseData, button: MouseButton) {}
    fn on_scroll(&mut self, mouse: &MouseData, x_diff: isize, y_diff: isize) {}
    fn on_key_down(&mut self, keys: Vec<KeyCode>) {}
    fn on_key_up(&mut self, keys: Vec<KeyCode>) {}
    fn on_window_closed(&mut self) {}
    fn on_visibility_changed(&mut self, visible: bool) {}
    fn on_focus_changed(&mut self, focused: bool) {}
    fn should_exit(&mut self) -> bool {
        false
    }
}

/// Options for program windows
#[cfg_attr(feature = "pixels_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "softbuffer_serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq)]
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
    /// (you'll have to draw your own if this is true, this is often called software cursor in games)
    /// Default is false
    pub hide_cursor: bool,
    /// If the mouse cursor should be locked to within this window while it's in the foreground
    /// Default is false
    pub confine_cursor: bool,
    /// Style data for [UiElement]
    pub style: UiStyle,
    /// Control how the program loops, see [Winit ControlFlow](https://docs.rs/winit/latest/winit/event_loop/enum.ControlFlow.html)
    #[cfg(feature = "softbuffer")]
    pub control_flow: ControlFlow,
}

impl Options {
    pub fn new(
        ups: usize,
        scaling: WindowScaling,
        vsync: bool,
        hide_cursor: bool,
        confine_cursor: bool,
        style: UiStyle,
        #[cfg(feature = "softbuffer")] control_flow: ControlFlow,
    ) -> Self {
        Self {
            ups,
            scaling,
            vsync,
            hide_cursor,
            confine_cursor,
            style,
            #[cfg(feature = "softbuffer")]
            control_flow,
        }
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            ups: 240,
            scaling: WindowScaling::Double,
            vsync: true,
            hide_cursor: false,
            confine_cursor: false,
            style: UiStyle::default(),
            #[cfg(feature = "softbuffer")]
            control_flow: ControlFlow::Poll,
        }
    }
}

#[cfg_attr(feature = "pixels_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "softbuffer_serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct MouseData {
    pub xy: Coord,
    buttons: FxHashMap<MouseButton, Coord>,
}

impl MouseData {
    pub fn any_held(&self) -> bool {
        !self.buttons.is_empty()
    }

    /// Returns the press location if the mouse button is currently held down
    pub fn is_down(&self, button: MouseButton) -> Option<Coord> {
        self.buttons.get(&button).cloned()
    }

    pub(crate) fn add_up(&mut self, button: MouseButton) {
        self.buttons.remove(&button);
    }

    pub(crate) fn add_down(&mut self, xy: Coord, button: MouseButton) {
        self.buttons.insert(button, xy);
    }
}
