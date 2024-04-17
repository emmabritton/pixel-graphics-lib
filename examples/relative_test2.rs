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
        "Relative Tester 2",
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
    center: Button,
    p_cch: Button,
    p_ccv: Button,
    v_cch: Button,
    v_ccv: Button,
    v_ct: Button,
    v_cb: Button,
    v_tc: Button,
    v_bc: Button,
    v_cl: Button,
    v_cr: Button,
    v_lc: Button,
    v_rc: Button,
}

impl LayoutTest {
    pub fn new(style: &UiStyle) -> Box<Self> {
        let mut center = Button::new((0, 0), "Center", Some(60), &style.button);
        let mut p_cch = Button::new((50, 50), "P_CCH", Some(60), &style.button);
        let mut p_ccv = Button::new((50, 50), "P_CCV", Some(60), &style.button);
        let mut v_cch = Button::new((20, 20), "V_CCH", Some(60), &style.button);
        let mut v_ccv = Button::new((20, 20), "V_CCV", Some(60), &style.button);

        let mut v_ct = Button::new((0, 0), "V_CT", Some(60), &style.button);
        let mut v_cb = Button::new((0, 0), "V_CB", Some(60), &style.button);
        let mut v_tc = Button::new((160, 40), "V_TC", Some(60), &style.button);
        let mut v_bc = Button::new((160, 40), "V_BC", Some(60), &style.button);

        let mut v_cl = Button::new((160, 160), "V_CL", Some(60), &style.button);
        let mut v_cr = Button::new((160, 160), "V_CR", Some(60), &style.button);
        let mut v_lc = Button::new((200, 200), "V_LC", Some(60), &style.button);
        let mut v_rc = Button::new((200, 200), "V_RC", Some(60), &style.button);

        let context = LayoutContext::new(Rect::new_with_size((0, 0), WIDTH, HEIGHT));

        layout!(context, center, align_centerv, px!(10));
        layout!(context, center, align_centerh, px!(10));

        layout!(context, v_ct, centerv_to_top_of center);
        layout!(context, v_cb, centerv_to_bottom_of center);
        layout!(context, v_tc, top_to_centerv_of center);
        layout!(context, v_bc, bottom_to_centerv_of center);

        layout!(context, v_cl, centerh_to_left_of center);
        layout!(context, v_cr, centerh_to_right_of center);
        layout!(context, v_lc, left_to_centerh_of center);
        layout!(context, v_rc, right_to_centerh_of center);

        layout!(context, p_ccv, align_centerv);
        layout!(context, p_cch, align_centerh);
        layout!(context, v_cch, centerh_to_centerh_of center);
        layout!(context, v_ccv, centerv_to_centerv_of center);

        Box::new(LayoutTest {
            center,
            p_cch,
            p_ccv,
            v_cch,
            v_ccv,
            v_ct,
            v_cb,
            v_tc,
            v_bc,
            v_cl,
            v_cr,
            v_lc,
            v_rc,
        })
    }
}

impl Scene<SceneResult, SceneName> for LayoutTest {
    fn render(&self, graphics: &mut Graphics, mouse: &MouseData, _: &FxHashSet<KeyCode>) {
        graphics.clear(BLUE);
        render!(
            graphics,
            mouse,
            self.center,
            self.p_cch,
            self.p_ccv,
            self.v_cch,
            self.v_ccv,
            self.v_ct,
            self.v_cb,
            self.v_tc,
            self.v_bc,
            self.v_cl,
            self.v_cr,
            self.v_lc,
            self.v_rc,
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
