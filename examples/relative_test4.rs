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
        "Relative Tester 4",
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
    vc: Button,
    bounds_tl: Button,
    bounds_br: Button,
    g_c_v_ct: Button,
    g_c_v_cb: Button,
    g_c_v_cl: Button,
    g_c_v_cr: Button,
}

impl LayoutTest {
    pub fn new(style: &UiStyle) -> Box<Self> {
        let mut vc = Button::new((0, 0), "VC", Some(60), &style.button);
        let mut bounds_tl = Button::new((0, 0), "TL", Some(40), &style.button);
        let mut bounds_br = Button::new((0, 0), "TR", Some(40), &style.button);
        let mut g_c_v_ct = Button::new((0, 0), "G_C_V_CT", Some(40), &style.button);
        let mut g_c_v_cb = Button::new((0, 0), "G_C_V_CB", Some(40), &style.button);
        let mut g_c_v_cl = Button::new((0, 0), "G_C_V_CL", Some(40), &style.button);
        let mut g_c_v_cr = Button::new((0, 0), "G_C_V_CR", Some(40), &style.button);

        let context = LayoutContext::new(Rect::new_with_size((0, 0), WIDTH, HEIGHT));

        layout!(context, vc, align_centerh);
        layout!(context, vc, align_centerv);

        layout!(context, bounds_tl, align_left, px!(30));
        layout!(context, bounds_tl, align_top, px!(30));

        layout!(context, bounds_br, align_right, px!(30));
        layout!(context, bounds_br, align_bottom, px!(30));

        layout!(context, g_c_v_ct, centerh_to_centerh_of vc);
        layout!(context, g_c_v_ct, bottom_to_top_of vc);
        layout!(context, grow g_c_v_ct, top_to_top_of bounds_tl);

        layout!(context, g_c_v_cb, centerh_to_centerh_of vc);
        layout!(context, g_c_v_cb, top_to_bottom_of vc);
        layout!(context, grow g_c_v_cb, bottom_to_bottom_of bounds_br);

        layout!(context, g_c_v_cl, centerv_to_centerv_of  vc);
        layout!(context, g_c_v_cl, right_to_left_of  vc);
        layout!(context, grow g_c_v_cl, left_to_left_of  bounds_tl);

        layout!(context, g_c_v_cr, centerv_to_centerv_of  vc);
        layout!(context, g_c_v_cr, left_to_right_of   vc);
        layout!(context, grow g_c_v_cr, left_to_right_of  bounds_br);

        Box::new(LayoutTest {
            vc,
            bounds_tl,
            bounds_br,
            g_c_v_ct,
            g_c_v_cb,
            g_c_v_cl,
            g_c_v_cr,
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
            self.bounds_br,
            self.vc,
            self.g_c_v_ct,
            self.g_c_v_cb,
            self.g_c_v_cl,
            self.g_c_v_cr,
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
