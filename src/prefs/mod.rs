pub mod preferences;

use thiserror::Error;
use serde::{Deserialize, Serialize};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::window::Window;
use crate::prefs::preferences::{get_pref_dir, Preferences};

const PREF_WINDOW: &str = "window.pref";

#[derive(Error, Debug)]
pub enum PrefError {
    #[error("Unable to get app pref dir")]
    AppPrefDir,
    #[error("Saving prefs: {0} to {1}")]
    Saving(String, String),
    #[error("Serializing data: {0}")]
    Serializing(String),
    #[error("Loading prefs: {0} from {1}")]
    Loading(String, String),
    #[error("Deserializing data: {0}")]
    Deserializing(String),
    #[error("Creating pref dir: {0} at {1}")]
    MakingDirs(String, String),
}

pub struct WindowPreferences {
    preferences: Preferences<WindowPref>,
}

impl WindowPreferences {
    pub fn new(qualifier: &str, org: &str, name: &str) -> Result<Self, PrefError> {
        let preferences = Preferences::new(get_pref_dir(qualifier, org, name)?, PREF_WINDOW);
        Ok(WindowPreferences { preferences })
    }
}

impl WindowPreferences {
    pub fn load(&mut self) -> Result<(), PrefError> {
        self.preferences.load()
    }

    pub fn save(&self) -> Result<(), PrefError> {
        self.preferences.save()
    }

    pub fn store(&mut self, window: &Window) {
        if let Ok(pos) = window.outer_position() {
            let size = window.inner_size();
            self.preferences.set(
                PREF_WINDOW,
                WindowPref::new(pos.x, pos.y, size.width, size.height),
            );
        }
    }

    pub fn restore(&self, window: &mut Window) {
        let default = WindowPref::default();
        let prefs = self.preferences.get(PREF_WINDOW).unwrap_or(&default);

        window.set_outer_position(PhysicalPosition::new(prefs.x, prefs.y));

        window.set_inner_size(PhysicalSize::new(prefs.w, prefs.h));
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WindowPref {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
}

impl WindowPref {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        WindowPref { x, y, w, h }
    }
}

impl Default for WindowPref {
    fn default() -> Self {
        WindowPref::new(100, 100, 480, 320)
    }
}