use crate::prelude::*;
use crate::ui::styles::UiStyle;
use crate::GraphicsError;
use buffer_graphics_lib::prelude::*;
use fnv::FnvHashSet;
use std::fmt::Debug;

/// Convenience method for programs built using [Scene]s
///
/// If you're not using scenes consider [run]
///
/// # Arguments
/// * `width` - Width of the whole window canvas in pixels
/// * `height` - Height of the whole window canvas in pixels
/// * `title` - Window title
/// * `window_prefs` - Optionally program info, if passed the window position and size will be persisted
/// * `scene_switcher` - [SceneSwitcher] Adds new scenes to the stack
/// * `init_scene` - The initial [Scene] to use
/// * `options` - [Options] controls how fast the program can update, [UiElement] styling, etc
pub fn run_scenes<
    SR: Clone + PartialEq + Debug + 'static,
    SN: Clone + PartialEq + Debug + 'static,
>(
    width: usize,
    height: usize,
    title: &str,
    window_prefs: Option<WindowPreferences>,
    scene_switcher: SceneSwitcher<SR, SN>,
    init_scene: Box<dyn Scene<SR, SN>>,
    options: Options,
) -> Result<(), GraphicsError> {
    let system = Box::new(SceneHost::new(
        init_scene,
        window_prefs,
        scene_switcher,
        options.style.clone(),
    )?);
    run(width, height, title, system, options)?;
    Ok(())
}

/// Creates new scenes.
///
/// # Important
/// This method must add the new scene to `scenes`
///
/// # Arguments
/// * `style` - Style data for [UiElement]s, can be ignored if UI is custom
/// * `scenes` - The current scene stack
/// * `new_scene` - The name and data for a new scene
pub type SceneSwitcher<SR, SN> =
    fn(style: &UiStyle, scenes: &mut Vec<Box<dyn Scene<SR, SN>>>, new_scene: SN);

/// When a scene wants to add or remove a scene from the stack it should return [Push][SceneUpdateResult::Push] or [Pop][SceneUpdateResult::Pop] from `Scene.update`
#[derive(Debug, Clone, PartialEq)]
pub enum SceneUpdateResult<SR: Clone + PartialEq + Debug, SN: Clone + PartialEq + Debug> {
    /// Do nothing, the current scene remains the foreground scene
    Nothing,
    /// Open a child scene
    /// # Arguments
    /// * `0` `bool` - If true this scene will be closed as well as opening the new scene
    /// * `1` `SN` - Data for [SceneSwitcher] so it can create the new scene
    Push(bool, SN),
    /// Close this scene, data may be included to be returned to the parent scene
    Pop(Option<SR>),
}

/// Scenes represent a mode/feature of a programs UI
/// For example in an image editor you could have the main menu, editor, and save dialog as scenes
/// and in an RPG you could have the field, battle and menu screens as scenes
///
/// Scenes can be fullscreen or smaller, such as a dialog
///
/// # Common mistakes
///
/// * If you use a field to store the [SceneUpdateResult] and return in [update()][Scene::update]
/// and then forget to clear it in [resuming][Scene::resuming] after a child returns then the child
/// will immediately reopen
pub trait Scene<SR: Clone + PartialEq + Debug, SN: Clone + PartialEq + Debug> {
    /// Render scene contents using `graphics`
    ///
    /// If this is a fullscreen scene it should draw a color over the whole screen otherwise
    /// you may see rendering issues (use `graphics.clear(Color)`).
    /// # Note
    /// mouse_xy will be -1,-1 if this screen is in the background and a non full screen scene is active
    #[cfg(feature = "controller")]
    fn render(
        &self,
        graphics: &mut Graphics,
        mouse_xy: Coord,
        held_keys: &Vec<&KeyCode>,
        controller: &GameController,
    );
    #[cfg(not(feature = "controller"))]
    fn render(&self, graphics: &mut Graphics, mouse_xy: Coord, held_keys: &Vec<&KeyCode>) {}
    /// Called when a keyboard key is being pressed down
    ///
    /// # Arguments
    /// * `key` - The latest pressed key
    /// * `held_keys` - Any other keys that are being pressed down
    #[allow(unused_variables)]
    fn on_key_down(&mut self, key: KeyCode, mouse_xy: Coord, held_keys: &Vec<&KeyCode>);
    /// Called when a keyboard key has been released
    ///
    /// # Arguments
    /// * `key` - The latest pressed key
    /// * `held_keys` - Any other keys that are being pressed down
    #[allow(unused_variables)]
    fn on_key_up(&mut self, key: KeyCode, mouse_xy: Coord, held_keys: &Vec<&KeyCode>) {}
    /// Called when a mouse button has been pressed down
    ///
    /// # Arguments
    /// * `xy` - The on screen coord of the cursor
    /// * `button` - The pressed mouse button
    /// * `held_keys` - Any keyboards keys that are being pressed down
    #[allow(unused_variables)]
    fn on_mouse_down(&mut self, xy: Coord, button: MouseButton, held_keys: &Vec<&KeyCode>) {}
    /// Called when a mouse button has been released
    ///
    /// # Arguments
    /// * `xy` - The on screen coord of the cursor
    /// * `button` - The pressed mouse button
    /// * `held_keys` - Any keyboards keys that are being pressed down
    #[allow(unused_variables)]
    fn on_mouse_up(&mut self, xy: Coord, button: MouseButton, held_keys: &Vec<&KeyCode>) {}
    /// Called when the mouse scroll function has been used
    ///
    /// # Arguments
    /// * `xy` - The on screen coord of the cursor
    /// * `y_diff` - The distance scrolled vertically
    /// * `x_diff` - The distance scrolled horizontally
    /// * `held_keys` - Any keyboards keys that are being pressed down
    #[allow(unused_variables)]
    fn on_scroll(&mut self, xy: Coord, x_diff: isize, y_diff: isize, held_keys: &Vec<&KeyCode>) {}
    /// During this method the scene should update animations and anything else that relies on time
    /// or on held keys
    ///
    /// # Arguments
    /// * `timing` - Deltas and other timing info, generally you should use the `fixed_time_step` field
    /// * `xy` - The on screen coord of the mouse cursor
    /// * `held_keys` - Any keyboards keys that are being pressed down
    ///
    /// # Returns
    ///
    /// [SceneUpdateResult]
    /// * In normal function this is will be [Nothing][SceneUpdateResult::Nothing]
    /// * To close this scene return [Pop][SceneUpdateResult::Pop]
    /// * To open a child scene return [Push][SceneUpdateResult::Push]
    #[cfg(feature = "controller")]
    fn update(
        &mut self,
        timing: &Timing,
        mouse_xy: Coord,
        held_keys: &Vec<&KeyCode>,
        controller: &GameController,
    ) -> SceneUpdateResult<SR, SN>;
    #[cfg(not(feature = "controller"))]
    fn update(
        &mut self,
        timing: &Timing,
        mouse_xy: Coord,
        held_keys: &Vec<&KeyCode>,
    ) -> SceneUpdateResult<SR, SN>;
    /// Called when a child scene is closing
    ///
    /// # Arguments
    /// * `result` - Optional data from child scene
    #[allow(unused_variables)]
    fn resuming(&mut self, result: Option<SR>) {}
    /// Return true if this scene doesn't fill the screen or is transparent
    /// If this returns false the previous fullscreen scene will render as well
    fn is_dialog(&self) -> bool {
        false
    }
}

struct SceneHost<SR: Clone + PartialEq + Debug, SN: Clone + PartialEq + Debug> {
    should_exit: bool,
    held_keys: FnvHashSet<KeyCode>,
    mouse_coord: Coord,
    scenes: Vec<Box<dyn Scene<SR, SN>>>,
    window_prefs: Option<WindowPreferences>,
    scene_switcher: SceneSwitcher<SR, SN>,
    style: UiStyle,
    #[cfg(feature = "controller")]
    controller: GameController,
}

impl<SR: Clone + PartialEq + Debug, SN: Clone + PartialEq + Debug> SceneHost<SR, SN> {
    pub fn new(
        init_scene: Box<dyn Scene<SR, SN>>,
        window_prefs: Option<WindowPreferences>,
        scene_switcher: SceneSwitcher<SR, SN>,
        style: UiStyle,
    ) -> Result<Self, GraphicsError> {
        Ok(Self {
            should_exit: false,
            held_keys: FnvHashSet::default(),
            mouse_coord: Coord::new(100, 100),
            scenes: vec![init_scene],
            window_prefs,
            scene_switcher,
            style,
            #[cfg(feature = "controller")]
            controller: GameController::new()
                .map_err(|e| GraphicsError::ControllerInit(e.to_string()))?,
        })
    }
}

impl<SR: Clone + PartialEq + Debug, SN: Clone + PartialEq + Debug> System for SceneHost<SR, SN> {
    fn window_prefs(&mut self) -> Option<WindowPreferences> {
        self.window_prefs.clone()
    }

    fn update(&mut self, timing: &Timing) {
        if let Some(scene) = self.scenes.last_mut() {
            #[cfg(feature = "controller")]
            let result = scene.update(
                timing,
                self.mouse_coord,
                &self.held_keys.iter().collect(),
                &self.controller,
            );
            #[cfg(not(feature = "controller"))]
            let result = scene.update(timing, self.mouse_coord, &self.held_keys.iter().collect());
            match result {
                SceneUpdateResult::Nothing => {}
                SceneUpdateResult::Push(pop_current, name) => {
                    if pop_current {
                        self.scenes.remove(self.scenes.len() - 1);
                    }
                    (self.scene_switcher)(&self.style, &mut self.scenes, name);
                }
                SceneUpdateResult::Pop(result) => {
                    self.scenes.remove(self.scenes.len() - 1);
                    if let Some(previous) = self.scenes.last_mut() {
                        previous.resuming(result);
                    }
                }
            }
        }
        if self.scenes.is_empty() {
            self.should_exit = true;
        }
    }

    fn render(&mut self, graphics: &mut Graphics) {
        if let Some(active) = self.scenes.last() {
            if active.is_dialog() {
                match self.scenes.iter().rposition(|scn| !scn.is_dialog()) {
                    None => graphics.clear(BLACK),
                    Some(i) => {
                        #[cfg(feature = "controller")]
                        self.scenes[i].render(
                            graphics,
                            Coord::new(-1, -1),
                            &self.held_keys.iter().collect(),
                            &self.controller,
                        );
                        #[cfg(not(feature = "controller"))]
                        active.render(
                            graphics,
                            Coord::new(-1, -1),
                            &self.held_keys.iter().collect(),
                        );
                    }
                }
                #[cfg(feature = "controller")]
                active.render(
                    graphics,
                    self.mouse_coord,
                    &self.held_keys.iter().collect(),
                    &self.controller,
                );
                #[cfg(not(feature = "controller"))]
                active.render(graphics, self.mouse_coord, &self.held_keys.iter().collect());
            } else {
                #[cfg(feature = "controller")]
                active.render(
                    graphics,
                    self.mouse_coord,
                    &self.held_keys.iter().collect(),
                    &self.controller,
                );
                #[cfg(not(feature = "controller"))]
                active.render(graphics, self.mouse_coord, &self.held_keys.iter().collect());
            }
        }
    }

    fn on_mouse_move(&mut self, x: usize, y: usize) {
        self.mouse_coord = Coord::from((x, y));
    }

    fn on_mouse_down(&mut self, x: usize, y: usize, button: MouseButton) {
        self.mouse_coord = Coord::from((x, y));
        if let Some(active) = self.scenes.last_mut() {
            active.on_mouse_down(self.mouse_coord, button, &self.held_keys.iter().collect());
        }
    }

    fn on_mouse_up(&mut self, x: usize, y: usize, button: MouseButton) {
        self.mouse_coord = Coord::from((x, y));
        if let Some(active) = self.scenes.last_mut() {
            active.on_mouse_up(self.mouse_coord, button, &self.held_keys.iter().collect());
        }
    }

    fn on_scroll(&mut self, x: usize, y: usize, x_diff: isize, y_diff: isize) {
        self.mouse_coord = Coord::from((x, y));
        if let Some(active) = self.scenes.last_mut() {
            active.on_scroll(
                self.mouse_coord,
                x_diff,
                y_diff,
                &self.held_keys.iter().collect(),
            );
        }
    }

    fn on_key_down(&mut self, keys: Vec<KeyCode>) {
        for key in keys {
            self.held_keys.insert(key);
            if let Some(active) = self.scenes.last_mut() {
                active.on_key_down(key, self.mouse_coord, &self.held_keys.iter().collect());
            }
        }
    }

    fn on_key_up(&mut self, keys: Vec<KeyCode>) {
        for key in keys {
            self.held_keys.remove(&key);
            if let Some(active) = self.scenes.last_mut() {
                active.on_key_up(key, self.mouse_coord, &self.held_keys.iter().collect());
            }
        }
    }

    fn should_exit(&mut self) -> bool {
        self.should_exit
    }
}
