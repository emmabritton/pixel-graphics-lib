use crate::SceneName::*;
use crate::SceneResult::*;
use anyhow::Result;
use buffer_graphics_lib::prelude::*;
use pixels_graphics_lib::prelude::SceneUpdateResult::*;
use pixels_graphics_lib::prelude::*;
use winit::event::VirtualKeyCode;

fn main() -> Result<()> {
    let switcher: SceneSwitcher<SceneResult, SceneName> = |_, scenes, name| match name {
        S2 => scenes.push(Box::new(Scene2 { result: Nothing })),
        S3(c) => scenes.push(Box::new(Scene3 {
            result: Nothing,
            back: c,
        })),
    };
    run_scenes(
        200,
        200,
        "Scene Test",
        None,
        switcher,
        Box::new(Scene1 { result: Nothing }),
        Options::default(),
    )?;

    Ok(())
}

struct Scene1 {
    result: SceneUpdateResult<SceneResult, SceneName>,
}

struct Scene2 {
    result: SceneUpdateResult<SceneResult, SceneName>,
}

struct Scene3 {
    result: SceneUpdateResult<SceneResult, SceneName>,
    back: Color,
}

#[derive(Clone, Debug, PartialEq)]
enum SceneName {
    //Scene 1 isn't listed as it's representing a menu that is the default screen
    //with no way to open it (instead the user would need to go back to it)
    S2,
    S3(Color),
}

#[derive(Clone, Debug, PartialEq)]
enum SceneResult {
    FromKey(VirtualKeyCode),
    FromMouse(Coord),
}

impl Scene<SceneResult, SceneName> for Scene1 {
    fn render(&self, graphics: &mut Graphics, _: Coord) {
        graphics.clear(BLUE);
    }

    fn on_mouse_up(&mut self, _: Coord, button: MouseButton, _: &Vec<&VirtualKeyCode>) {
        if button != MouseButton::Left {
            return;
        }
        self.result = Push(false, S2)
    }

    fn update(
        &mut self,
        _: &Timing,
        _: Coord,
        _: &Vec<&VirtualKeyCode>,
    ) -> SceneUpdateResult<SceneResult, SceneName> {
        self.result.clone()
    }

    fn resuming(&mut self, result: Option<SceneResult>) {
        if let Some(result) = result {
            match result {
                FromKey(_) => {}
                FromMouse(_) => {}
            }
        }
        self.result = Nothing
    }
}

impl Scene<SceneResult, SceneName> for Scene2 {
    fn render(&self, graphics: &mut Graphics, _: Coord) {
        graphics.clear(RED);
    }

    fn on_key_up(&mut self, key: VirtualKeyCode, _: Coord, _: &Vec<&VirtualKeyCode>) {
        self.result = Pop(Some(FromKey(key)))
    }

    fn on_mouse_up(&mut self, xy: Coord, button: MouseButton, _: &Vec<&VirtualKeyCode>) {
        if button != MouseButton::Left {
            return;
        }
        if xy.y < 100 {
            self.result = Push(false, S3(GREEN))
        } else {
            self.result = Pop(Some(FromMouse(xy)))
        }
    }

    fn update(
        &mut self,
        _: &Timing,
        _: Coord,
        _: &Vec<&VirtualKeyCode>,
    ) -> SceneUpdateResult<SceneResult, SceneName> {
        self.result.clone()
    }

    fn resuming(&mut self, _: Option<SceneResult>) {
        self.result = Nothing
    }
}

impl Scene<SceneResult, SceneName> for Scene3 {
    fn render(&self, graphics: &mut Graphics, _: Coord) {
        graphics.draw_rect(Rect::new((90, 90), (190, 150)), fill(self.back));
    }

    fn on_mouse_up(&mut self, _: Coord, button: MouseButton, _: &Vec<&VirtualKeyCode>) {
        if button != MouseButton::Left {
            return;
        }
        self.result = Pop(None);
    }

    fn update(
        &mut self,
        _: &Timing,
        _: Coord,
        _: &Vec<&VirtualKeyCode>,
    ) -> SceneUpdateResult<SceneResult, SceneName> {
        self.result.clone()
    }

    fn resuming(&mut self, _: Option<SceneResult>) {}

    fn is_dialog(&self) -> bool {
        true
    }
}
