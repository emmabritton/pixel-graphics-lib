use serde::{Deserialize, Serialize};
use simple_game_utils::prelude::*;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::window::Window;

const PREF_WINDOW: &str = "window.pref";

#[derive(Clone, Debug)]
pub struct WindowPreferences {
    preferences: Preferences<WindowPref>,
}

impl WindowPreferences {
    pub fn new(
        qualifier: &str,
        org: &str,
        name: &str,
        version: usize,
    ) -> Result<Self, GameUtilError> {
        let preferences = Preferences::new(
            get_pref_dir(qualifier, org, name)?,
            &format!("{PREF_WINDOW}{version}"),
        );
        Ok(WindowPreferences { preferences })
    }
}

impl WindowPreferences {
    pub fn load(&mut self) -> Result<(), GameUtilError> {
        self.preferences.load()
    }

    pub fn save(&self) -> Result<(), GameUtilError> {
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

            let _ = window.request_inner_size(PhysicalSize::new(prefs.w, prefs.h));
        } else if let Some(monitor) = window.current_monitor() {
            let mid_x = monitor.size().width / 2;
            let mid_y = monitor.size().height / 2;
            let mid_w = window.inner_size().width / 2;
            let mid_h = window.inner_size().height / 2;
            window.set_outer_position(PhysicalPosition::new(
                mid_x.saturating_sub(mid_w),
                mid_y.saturating_sub(mid_h),
            ));
        } else {
            window.set_outer_position(PhysicalPosition::new(100, 100));
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
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
