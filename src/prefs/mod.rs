pub mod preferences;

use crate::prefs::preferences::{get_pref_dir, Preferences};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::window::Window;

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

    pub fn clear(&mut self) {
        self.preferences.clear(PREF_WINDOW);
    }

    pub fn delete_file(&self) -> bool {
        self.preferences.delete_file()
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
        if let Some(prefs) = self.preferences.get(PREF_WINDOW) {
            window.set_outer_position(PhysicalPosition::new(prefs.x, prefs.y));

            window.set_inner_size(PhysicalSize::new(prefs.w, prefs.h));
        } else if let Some(monitor) = window.current_monitor() {
            let mid_x = monitor.size().width / 2;
            let mid_y = monitor.size().height / 2;
            let mid_w = window.inner_size().width / 2;
            let mid_h = window.inner_size().height / 2;
            window.set_outer_position(PhysicalPosition::new(mid_x - mid_w, mid_y - mid_h));
        } else {
            window.set_outer_position(PhysicalPosition::new(100, 100));
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct WindowPref {
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
