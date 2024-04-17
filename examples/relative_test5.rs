use anyhow::Result;
use buffer_graphics_lib::prelude::*;
use winit::keyboard::KeyCode;

use pixels_graphics_lib::prelude::SceneUpdateResult::*;
use pixels_graphics_lib::prelude::*;
use pixels_graphics_lib::ui::prelude::relative::*;
use pixels_graphics_lib::ui::prelude::*;
use pixels_graphics_lib::{layout, px, render};

const WIDTH: usize = 280;
const HEIGHT: usize = 240;

fn main() -> Result<()> {
    let switcher: SceneSwitcher<SceneResult, SceneName> = |_, _, _| {};
    let options = Options::default();
    let test = LayoutTest::new(&options.style);
    run_scenes(
        WIDTH,
        HEIGHT,
        "Relative Tester 5",
        None,
        switcher,
        test,
        options,
        empty_pre_post(),
    )?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
enum SceneName {}

#[derive(Debug, Clone, PartialEq)]
enum SceneResult {}

pub struct LayoutTest {
    bounds_tl: Button,
    bounds_tr: Button,
    bounds_bl: Button,
    mid_h: Button,
    mid_v: Button,
}

impl LayoutTest {
    pub fn new(style: &UiStyle) -> Box<Self> {
        let mut bounds_tl = Button::new((0, 0), "TL", Some(40), &style.button);
        let mut bounds_tr = Button::new((0, 0), "TR", Some(40), &style.button);
        let mut bounds_bl = Button::new((0, 0), "BR", Some(40), &style.button);
        let mut mid_h = Button::new((0, 0), "H", Some(40), &style.button);
        let mut mid_v = Button::new((0, 0), "V", Some(40), &style.button);

        let context = LayoutContext::new(Rect::new_with_size((0, 0), WIDTH, HEIGHT));

        layout!(context, bounds_tl, align_left, px!(30));
        layout!(context, bounds_tl, align_top, px!(30));

        layout!(context, bounds_tr, align_right, px!(30));
        layout!(context, bounds_tr, align_top, px!(30));

        layout!(context, bounds_bl, align_left, px!(30));
        layout!(context, bounds_bl, align_bottom, px!(30));

        layout!(context, mid_h, top_to_top_of bounds_tl);
        layout!(context, centerh mid_h, between bounds_tl, bounds_tr);

        layout!(context, mid_v, left_to_left_of  bounds_tl);
        layout!(context, centerv mid_v, between bounds_tl, bounds_bl);

        Box::new(LayoutTest {
            bounds_tl,
            bounds_tr,
            bounds_bl,
            mid_h,
            mid_v,
        })
    }
}

impl Scene<SceneResult, SceneName> for LayoutTest {
    fn render(&self, graphics: &mut Graphics, mouse: &MouseData, _: &FxHashSet<KeyCode>) {
        graphics.clear(BLUE);
        render!(
            graphics,
            mouse,
            self.bounds_tl,
            self.bounds_tr,
            self.bounds_bl,
            self.mid_h,
            self.mid_v,
        );
    }

    fn on_key_up(&mut self, _: KeyCode, _: &MouseData, _: &FxHashSet<KeyCode>) {}

    fn on_mouse_click(&mut self, _: Coord, _: &MouseData, _: MouseButton, _: &FxHashSet<KeyCode>) {}

    fn update(
        &mut self,
        _: &Timing,
        _: &MouseData,
        _: &FxHashSet<KeyCode>,
    ) -> SceneUpdateResult<SceneResult, SceneName> {
        Nothing
    }

    fn resuming(&mut self, _: Option<SceneResult>) {}
}
