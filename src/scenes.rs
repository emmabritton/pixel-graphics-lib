use crate::prelude::*;
use crate::utilities::virtual_key_codes::{ARROWS, LETTERS, MODIFIERS, NUMBERS, SYMBOLS, TYPING};
use crate::GraphicsError;
use buffer_graphics_lib::prelude::*;
use std::collections::HashSet;
use std::fmt::Debug;
use crate::ui::styles::UiStyle;

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
    let system = Box::new(SceneHost::new(init_scene, window_prefs, scene_switcher, options.style.clone()));
    run(width, height, title, system, options)?;
    Ok(())
}

pub type SceneSwitcher<SR, SN> = fn(style: &UiStyle, scenes: &mut Vec<Box<dyn Scene<SR, SN>>>, new_scene: SN);

#[derive(Debug, Clone, PartialEq)]
pub enum SceneUpdateResult<SR: Clone + PartialEq + Debug, SN: Clone + PartialEq + Debug> {
    Nothing,
    Push(bool, SN),
    Pop(Option<SR>),
}

pub trait Scene<SR: Clone + PartialEq + Debug, SN: Clone + PartialEq + Debug> {
    ///mouse_xy will be -1,-1 if this screen is in the background
    fn render(&self, graphics: &mut Graphics, mouse_xy: Coord);
    fn on_key_press(&mut self, key: VirtualKeyCode, held_keys: &Vec<&VirtualKeyCode>);
    fn on_mouse_click(&mut self, xy: Coord, held_keys: &Vec<&VirtualKeyCode>);
    #[allow(unused_variables)]
    fn on_scroll(&mut self, diff: isize) {}
    fn update(
        &mut self,
        timing: &Timing,
        mouse_xy: Coord,
        held_keys: &Vec<&VirtualKeyCode>,
    ) -> SceneUpdateResult<SR, SN>;
    fn resuming(&mut self, result: Option<SR>);
    ///Return true if this scene doesn't fill the screen
    fn is_dialog(&self) -> bool {
        false
    }
}

struct SceneHost<SR: Clone + PartialEq + Debug, SN: Clone + PartialEq + Debug> {
    should_exit: bool,
    held_keys: HashSet<VirtualKeyCode>,
    mouse_coord: Coord,
    keys: Vec<VirtualKeyCode>,
    scenes: Vec<Box<dyn Scene<SR, SN>>>,
    window_prefs: Option<WindowPreferences>,
    scene_switcher: SceneSwitcher<SR, SN>,
    style: UiStyle
}

impl<SR: Clone + PartialEq + Debug, SN: Clone + PartialEq + Debug> SceneHost<SR, SN> {
    pub fn new(
        init_scene: Box<dyn Scene<SR, SN>>,
        window_prefs: Option<WindowPreferences>,
        scene_switcher: SceneSwitcher<SR, SN>,
        style: UiStyle
    ) -> Self {
        let mut keys = vec![];
        keys.extend_from_slice(&LETTERS);
        keys.extend_from_slice(&NUMBERS);
        keys.extend_from_slice(&MODIFIERS);
        keys.extend_from_slice(&ARROWS);
        keys.extend_from_slice(&SYMBOLS);
        keys.extend_from_slice(&TYPING);
        Self {
            keys,
            should_exit: false,
            held_keys: HashSet::new(),
            mouse_coord: Coord::new(100, 100),
            scenes: vec![init_scene],
            window_prefs,
            scene_switcher,
            style
        }
    }
}

impl<SR: Clone + PartialEq + Debug, SN: Clone + PartialEq + Debug> System for SceneHost<SR, SN> {
    fn action_keys(&self) -> Vec<VirtualKeyCode> {
        self.keys.clone()
    }

    fn window_prefs(&self) -> Option<WindowPreferences> {
        self.window_prefs.clone()
    }

    fn update(&mut self, timing: &Timing) {
        if let Some(scene) = self.scenes.last_mut() {
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

    fn render(&self, graphics: &mut Graphics) {
        if let Some(active) = self.scenes.last() {
            if active.is_dialog() {
                match self.scenes.iter().rposition(|scn| !scn.is_dialog()) {
                    None => graphics.clear(BLACK),
                    Some(i) => self.scenes[i].render(graphics, Coord::new(-1, -1)),
                }
                active.render(graphics, self.mouse_coord);
            } else {
                active.render(graphics, self.mouse_coord);
            }
        }
    }

    fn on_mouse_move(&mut self, x: usize, y: usize) {
        self.mouse_coord = Coord::from((x, y));
    }

    fn on_mouse_up(&mut self, x: usize, y: usize, button: MouseButton) {
        self.mouse_coord = Coord::from((x, y));
        if button == MouseButton::Left {
            if let Some(active) = self.scenes.last_mut() {
                active.on_mouse_click(self.mouse_coord, &self.held_keys.iter().collect());
            }
        }
    }

    fn on_scroll(&mut self, diff: isize) {
        if let Some(active) = self.scenes.last_mut() {
            active.on_scroll(diff);
        }
    }

    fn on_key_pressed(&mut self, keys: Vec<VirtualKeyCode>) {
        for key in keys {
            if let Some(active) = self.scenes.last_mut() {
                active.on_key_press(key, &self.held_keys.iter().collect());
            }
        }
    }

    fn on_key_down(&mut self, keys: Vec<VirtualKeyCode>) {
        for key in keys {
            self.held_keys.insert(key);
        }
    }

    fn on_key_up(&mut self, keys: Vec<VirtualKeyCode>) {
        for key in keys {
            self.held_keys.remove(&key);
        }
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }
}
