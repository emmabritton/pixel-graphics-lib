use crate::prelude::*;
use crate::ui::styles::UiStyle;
use crate::GraphicsError;
use buffer_graphics_lib::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use std::fmt::Debug;
use winit::event::MouseButton;

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
#[allow(clippy::too_many_arguments)]
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
    pre_post: Box<dyn PrePost<SR, SN>>,
) -> Result<(), GraphicsError> {
    let system = Box::new(SceneHost::new(
        init_scene,
        window_prefs,
        scene_switcher,
        options.style.clone(),
        pre_post,
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
///   and then forget to clear it in [resuming][Scene::resuming] after a child returns then the child
///   will immediately reopen
pub trait Scene<SR: Clone + PartialEq + Debug, SN: Clone + PartialEq + Debug> {
    fn id(&self) -> u32 {
        0
    }

    /// Render scene contents using `graphics`
    ///
    /// If this is a fullscreen scene it should draw a color over the whole screen otherwise
    /// you may see rendering issues (use `graphics.clear(Color)`).
    /// # Note
    /// mouse will be empty if this screen is in the background and a non full screen scene is active
    #[cfg(any(feature = "controller", feature = "controller_xinput"))]
    #[allow(unused_variables)]
    fn render(
        &self,
        graphics: &mut Graphics,
        mouse: &MouseData,
        held_keys: &FxHashSet<KeyCode>,
        controller: &GameController,
    );
    /// Render scene contents using `graphics`
    ///
    /// If this is a fullscreen scene it should draw a color over the whole screen otherwise
    /// you may see rendering issues (use `graphics.clear(Color)`).
    /// # Note
    /// mouse will be empty if this screen is in the background and a non full screen scene is active
    #[cfg(not(any(feature = "controller", feature = "controller_xinput")))]
    #[allow(unused_variables)]
    fn render(&self, graphics: &mut Graphics, mouse: &MouseData, held_keys: &FxHashSet<KeyCode>) {}
    /// Called when a keyboard key is being pressed down
    ///
    /// # Arguments
    /// * `key` - The latest pressed key
    /// * `mouse` - position, held state of mouse
    /// * `held_keys` - Any other keys that are being pressed down
    #[allow(unused_variables)]
    fn on_key_down(&mut self, key: KeyCode, mouse: &MouseData, held_keys: &FxHashSet<KeyCode>) {}
    /// Called when a keyboard key has been released
    ///
    /// # Arguments
    /// * `key` - The latest pressed key
    /// * `mouse` - position, held state of mouse
    /// * `held_keys` - Any other keys that are being pressed down
    #[allow(unused_variables)]
    fn on_key_up(&mut self, key: KeyCode, mouse: &MouseData, held_keys: &FxHashSet<KeyCode>) {}
    /// Called when a mouse button has been pressed down
    ///
    /// # Arguments
    /// * `mouse` - position, held state of mouse
    /// * `mouse_button` = which button was pressed
    /// * `held_keys` - Any keyboards keys that are being pressed down
    #[allow(unused_variables)]
    fn on_mouse_down(
        &mut self,
        mouse: &MouseData,
        mouse_button: MouseButton,
        held_keys: &FxHashSet<KeyCode>,
    ) {
    }
    /// Called when a mouse button has been released
    ///
    /// [on_mouse_click] will also be called after
    ///
    /// # Arguments
    /// * `mouse` - position, held state of mouse
    /// * `mouse_button` = which button was released
    /// * `held_keys` - Any keyboards keys that are being pressed down
    #[allow(unused_variables)]
    fn on_mouse_up(
        &mut self,
        mouse: &MouseData,
        mouse_button: MouseButton,
        held_keys: &FxHashSet<KeyCode>,
    ) {
    }
    /// Called when a mouse button has been pressed and released
    ///
    /// [on_mouse_up] will also be called before
    ///
    /// # Arguments
    /// * `down_at` - position where mouse button was clicked
    /// * `mouse` - position, held state of mouse
    /// * `mouse_button` = which button was clicked
    /// * `held_keys` - Any keyboards keys that are being pressed down
    #[allow(unused_variables)]
    fn on_mouse_click(
        &mut self,
        down_at: Coord,
        mouse: &MouseData,
        mouse_button: MouseButton,
        held_keys: &FxHashSet<KeyCode>,
    ) {
    }
    /// Called when the mouse moved while any button is held down
    ///
    /// # Arguments
    /// * `mouse` - position, held state of mouse
    /// * `held_keys` - Any keyboards keys that are being pressed down
    #[allow(unused_variables)]
    fn on_mouse_drag(&mut self, mouse: &MouseData, held_keys: &FxHashSet<KeyCode>) {}
    /// Called when the mouse scroll function has been used
    ///
    /// # Arguments
    /// * `xy` - The on screen coord of the cursor
    /// * `y_diff` - The distance scrolled vertically
    /// * `x_diff` - The distance scrolled horizontally
    /// * `held_keys` - Any keyboards keys that are being pressed down
    #[allow(unused_variables)]
    fn on_scroll(
        &mut self,
        mouse: &MouseData,
        x_diff: isize,
        y_diff: isize,
        held_keys: &FxHashSet<KeyCode>,
    ) {
    }
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
    #[cfg(any(feature = "controller", feature = "controller_xinput"))]
    fn update(
        &mut self,
        timing: &Timing,
        mouse: &MouseData,
        held_keys: &FxHashSet<KeyCode>,
        controller: &GameController,
    ) -> SceneUpdateResult<SR, SN>;
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
    #[cfg(not(any(feature = "controller", feature = "controller_xinput")))]
    fn update(
        &mut self,
        timing: &Timing,
        mouse: &MouseData,
        held_keys: &FxHashSet<KeyCode>,
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

pub trait PrePost<SR, SN> {
    #[cfg(not(any(feature = "controller", feature = "controller_xinput")))]
    fn pre_render(
        &mut self,
        graphics: &mut Graphics,
        mouse: &MouseData,
        held_keys: &FxHashSet<KeyCode>,
        scenes: &mut [Box<dyn Scene<SR, SN>>],
    );
    #[cfg(not(any(feature = "controller", feature = "controller_xinput")))]
    fn post_render(
        &mut self,
        graphics: &mut Graphics,
        mouse: &MouseData,
        held_keys: &FxHashSet<KeyCode>,
        scenes: &mut [Box<dyn Scene<SR, SN>>],
    );
    #[cfg(any(feature = "controller", feature = "controller_xinput"))]
    fn pre_render(
        &mut self,
        graphics: &mut Graphics,
        mouse: &MouseData,
        held_keys: &FxHashSet<KeyCode>,
        scenes: &mut [Box<dyn Scene<SR, SN>>],
        controller: &GameController,
    );
    #[cfg(any(feature = "controller", feature = "controller_xinput"))]
    fn post_render(
        &mut self,
        graphics: &mut Graphics,
        mouse: &MouseData,
        held_keys: &FxHashSet<KeyCode>,
        scenes: &mut [Box<dyn Scene<SR, SN>>],
        controller: &GameController,
    );
    #[cfg(not(any(feature = "controller", feature = "controller_xinput")))]
    fn pre_update(
        &mut self,
        timing: &Timing,
        mouse: &MouseData,
        held_keys: &FxHashSet<KeyCode>,
        scenes: &mut [Box<dyn Scene<SR, SN>>],
    );
    #[cfg(not(any(feature = "controller", feature = "controller_xinput")))]
    fn post_update(
        &mut self,
        timing: &Timing,
        mouse: &MouseData,
        held_keys: &FxHashSet<KeyCode>,
        scenes: &mut [Box<dyn Scene<SR, SN>>],
    );
    #[cfg(any(feature = "controller", feature = "controller_xinput"))]
    fn pre_update(
        &mut self,
        timing: &Timing,
        mouse: &MouseData,
        held_keys: &FxHashSet<KeyCode>,
        scenes: &mut [Box<dyn Scene<SR, SN>>],
        controller: &GameController,
    );
    #[cfg(any(feature = "controller", feature = "controller_xinput"))]
    fn post_update(
        &mut self,
        timing: &Timing,
        mouse: &MouseData,
        held_keys: &FxHashSet<KeyCode>,
        scenes: &mut [Box<dyn Scene<SR, SN>>],
        controller: &GameController,
    );
}
#[cfg(any(feature = "controller", feature = "controller_xinput"))]
pub fn empty_pre_post<SR, SN>() -> Box<dyn PrePost<SR, SN>> {
    struct Empty {}
    impl<SR, SN> PrePost<SR, SN> for Empty {
        fn pre_render(
            &mut self,
            _: &mut Graphics,
            _: &MouseData,
            _: &FxHashSet<KeyCode>,
            _: &mut [Box<dyn Scene<SR, SN>>],
            _: &GameController,
        ) {
        }

        fn post_render(
            &mut self,
            _: &mut Graphics,
            _: &MouseData,
            _: &FxHashSet<KeyCode>,
            _: &mut [Box<dyn Scene<SR, SN>>],
            _: &GameController,
        ) {
        }

        fn pre_update(
            &mut self,
            _: &Timing,
            _: &MouseData,
            _: &FxHashSet<KeyCode>,
            _: &mut [Box<dyn Scene<SR, SN>>],
            _: &GameController,
        ) {
        }

        fn post_update(
            &mut self,
            _: &Timing,
            _: &MouseData,
            _: &FxHashSet<KeyCode>,
            _: &mut [Box<dyn Scene<SR, SN>>],
            _: &GameController,
        ) {
        }
    }
    Box::new(Empty {})
}
#[cfg(not(any(feature = "controller", feature = "controller_xinput")))]
pub fn empty_pre_post<SR, SN>() -> Box<dyn PrePost<SR, SN>> {
    struct Empty {}
    impl<SR, SN> PrePost<SR, SN> for Empty {
        fn pre_render(
            &mut self,
            _: &mut Graphics,
            _: &MouseData,
            _: &FxHashSet<KeyCode>,
            _: &mut [Box<dyn Scene<SR, SN>>],
        ) {
        }

        fn post_render(
            &mut self,
            _: &mut Graphics,
            _: &MouseData,
            _: &FxHashSet<KeyCode>,
            _: &mut [Box<dyn Scene<SR, SN>>],
        ) {
        }

        fn pre_update(
            &mut self,
            _: &Timing,
            _: &MouseData,
            _: &FxHashSet<KeyCode>,
            _: &mut [Box<dyn Scene<SR, SN>>],
        ) {
        }

        fn post_update(
            &mut self,
            _: &Timing,
            _: &MouseData,
            _: &FxHashSet<KeyCode>,
            _: &mut [Box<dyn Scene<SR, SN>>],
        ) {
        }
    }
    Box::new(Empty {})
}

struct SceneHost<SR: Clone + PartialEq + Debug, SN: Clone + PartialEq + Debug> {
    should_exit: bool,
    held_keys: FxHashSet<KeyCode>,
    scenes: Vec<Box<dyn Scene<SR, SN>>>,
    window_prefs: Option<WindowPreferences>,
    scene_switcher: SceneSwitcher<SR, SN>,
    style: UiStyle,
    #[cfg(any(feature = "controller", feature = "controller_xinput"))]
    controller: GameController,
    mouse: MouseData,
    mouse_down_at: FxHashMap<MouseButton, Coord>,
    pre_post: Box<dyn PrePost<SR, SN>>,
}

impl<SR: Clone + PartialEq + Debug, SN: Clone + PartialEq + Debug> SceneHost<SR, SN> {
    pub fn new(
        init_scene: Box<dyn Scene<SR, SN>>,
        window_prefs: Option<WindowPreferences>,
        scene_switcher: SceneSwitcher<SR, SN>,
        style: UiStyle,
        pre_post: Box<dyn PrePost<SR, SN>>,
    ) -> Result<Self, GraphicsError> {
        Ok(Self {
            pre_post,
            should_exit: false,
            held_keys: FxHashSet::default(),
            scenes: vec![init_scene],
            window_prefs,
            scene_switcher,
            style,
            mouse: MouseData::default(),
            mouse_down_at: FxHashMap::default(),
            #[cfg(any(feature = "controller", feature = "controller_xinput"))]
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
        #[cfg(any(feature = "controller", feature = "controller_xinput"))]
        self.pre_post.pre_update(
            timing,
            &MouseData::default(),
            &self.held_keys,
            &mut self.scenes,
            &self.controller,
        );
        #[cfg(not(any(feature = "controller", feature = "controller_xinput")))]
        self.pre_post.pre_update(
            timing,
            &MouseData::default(),
            &self.held_keys,
            &mut self.scenes,
        );
        #[cfg(any(feature = "controller", feature = "controller_xinput"))]
        self.controller.update();
        if let Some(scene) = self.scenes.last_mut() {
            #[cfg(any(feature = "controller", feature = "controller_xinput"))]
            let result = scene.update(timing, &self.mouse, &self.held_keys, &self.controller);
            #[cfg(not(any(feature = "controller", feature = "controller_xinput")))]
            let result = scene.update(timing, &self.mouse, &self.held_keys);
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
        #[cfg(any(feature = "controller", feature = "controller_xinput"))]
        self.pre_post.post_update(
            timing,
            &MouseData::default(),
            &self.held_keys,
            &mut self.scenes,
            &self.controller,
        );
        #[cfg(not(any(feature = "controller", feature = "controller_xinput")))]
        self.pre_post.post_update(
            timing,
            &MouseData::default(),
            &self.held_keys,
            &mut self.scenes,
        );
        if self.scenes.is_empty() {
            self.should_exit = true;
        }
    }

    fn render(&mut self, graphics: &mut Graphics) {
        #[cfg(any(feature = "controller", feature = "controller_xinput"))]
        self.pre_post.pre_render(
            graphics,
            &MouseData::default(),
            &self.held_keys,
            &mut self.scenes,
            &self.controller,
        );
        #[cfg(not(any(feature = "controller", feature = "controller_xinput")))]
        self.pre_post.pre_render(
            graphics,
            &MouseData::default(),
            &self.held_keys,
            &mut self.scenes,
        );
        if let Some(active) = self.scenes.last() {
            if active.is_dialog() {
                match self.scenes.iter().rposition(|scn| !scn.is_dialog()) {
                    None => graphics.clear(BLACK),
                    Some(i) => {
                        #[cfg(any(feature = "controller", feature = "controller_xinput"))]
                        self.scenes[i].render(
                            graphics,
                            &MouseData::default(),
                            &self.held_keys,
                            &self.controller,
                        );
                        #[cfg(not(any(feature = "controller", feature = "controller_xinput")))]
                        self.scenes[i].render(graphics, &MouseData::default(), &self.held_keys);
                    }
                }
                #[cfg(any(feature = "controller", feature = "controller_xinput"))]
                active.render(graphics, &self.mouse, &self.held_keys, &self.controller);
                #[cfg(not(any(feature = "controller", feature = "controller_xinput")))]
                active.render(graphics, &self.mouse, &self.held_keys);
            } else {
                #[cfg(any(feature = "controller", feature = "controller_xinput"))]
                active.render(graphics, &self.mouse, &self.held_keys, &self.controller);
                #[cfg(not(any(feature = "controller", feature = "controller_xinput")))]
                active.render(graphics, &self.mouse, &self.held_keys);
            }
        }
        #[cfg(any(feature = "controller", feature = "controller_xinput"))]
        self.pre_post.post_render(
            graphics,
            &MouseData::default(),
            &self.held_keys,
            &mut self.scenes,
            &self.controller,
        );
        #[cfg(not(any(feature = "controller", feature = "controller_xinput")))]
        self.pre_post.post_render(
            graphics,
            &MouseData::default(),
            &self.held_keys,
            &mut self.scenes,
        );
    }

    fn on_mouse_move(&mut self, mouse_data: &MouseData) {
        self.mouse = mouse_data.clone();
        if self.mouse.any_held() {
            if let Some(active) = self.scenes.last_mut() {
                active.on_mouse_drag(&self.mouse, &self.held_keys)
            }
        }
    }

    fn on_mouse_down(&mut self, mouse: &MouseData, button: MouseButton) {
        self.mouse = mouse.clone();
        self.mouse_down_at.insert(button, self.mouse.xy);
        if let Some(active) = self.scenes.last_mut() {
            active.on_mouse_down(&self.mouse, button, &self.held_keys);
        }
    }

    fn on_mouse_up(&mut self, mouse: &MouseData, button: MouseButton) {
        self.mouse = mouse.clone();
        if let Some(active) = self.scenes.last_mut() {
            active.on_mouse_up(&self.mouse, button, &self.held_keys);
            if let Some(down) = self.mouse_down_at.get(&button) {
                active.on_mouse_click(*down, &self.mouse, button, &self.held_keys);
            }
            self.mouse_down_at.remove(&button);
        }
    }

    fn on_scroll(&mut self, mouse: &MouseData, x_diff: isize, y_diff: isize) {
        self.mouse = mouse.clone();
        if let Some(active) = self.scenes.last_mut() {
            active.on_scroll(&self.mouse, x_diff, y_diff, &self.held_keys);
        }
    }

    fn on_key_down(&mut self, keys: Vec<KeyCode>) {
        for key in keys {
            self.held_keys.insert(key);
            if let Some(active) = self.scenes.last_mut() {
                active.on_key_down(key, &self.mouse, &self.held_keys);
            }
        }
    }

    fn on_key_up(&mut self, keys: Vec<KeyCode>) {
        for key in keys {
            self.held_keys.remove(&key);
            if let Some(active) = self.scenes.last_mut() {
                active.on_key_up(key, &self.mouse, &self.held_keys);
            }
        }
    }

    fn should_exit(&mut self) -> bool {
        self.should_exit
    }
}
