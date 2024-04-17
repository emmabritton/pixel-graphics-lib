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
        "Relative Tester 3",
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
    pc: Button,
    vc: Button,
    bounds_tl: Button,
    bounds_br: Button,
    g_c_pt: Button,
    g_c_pl: Button,
    g_c_pr: Button,
    g_c_pb: Button,
    g_c_v_tt: Button,
    g_c_v_bb: Button,
    g_c_v_ll: Button,
    g_c_v_rr: Button,
}

impl LayoutTest {
    pub fn new(style: &UiStyle) -> Box<Self> {
        let mut pc = Button::new((0, 0), "PC", Some(60), &style.button);
        let mut vc = Button::new((0, 0), "VC", Some(60), &style.button);
        let mut bounds_tl = Button::new((0, 0), "TL", Some(40), &style.button);
        let mut bounds_br = Button::new((0, 0), "TR", Some(40), &style.button);
        let mut g_c_pt = Button::new((0, 0), "G_C_PT", Some(40), &style.button);
        let mut g_c_pb = Button::new((0, 0), "G_C_PB", Some(40), &style.button);
        let mut g_c_pl = Button::new((0, 0), "G_C_PL", Some(40), &style.button);
        let mut g_c_pr = Button::new((0, 0), "G_C_PR", Some(40), &style.button);
        let mut g_c_v_tt = Button::new((0, 0), "TT", Some(40), &style.button);
        let mut g_c_v_bb = Button::new((0, 0), "BB", Some(40), &style.button);
        let mut g_c_v_ll = Button::new((0, 0), "LL", Some(40), &style.button);
        let mut g_c_v_rr = Button::new((0, 0), "RR", Some(40), &style.button);

        let context = LayoutContext::new(Rect::new_with_size((0, 0), WIDTH, HEIGHT));

        layout!(context, pc, align_left, px!(70));
        layout!(context, pc, align_top, px!(70));

        layout!(context, vc, align_centerh);
        layout!(context, vc, align_centerv);

        layout!(context, bounds_tl, align_left, px!(30));
        layout!(context, bounds_tl, align_top, px!(30));

        layout!(context, bounds_br, align_right, px!(30));
        layout!(context, bounds_br, align_bottom, px!(30));

        layout!(context, g_c_pt, centerh_to_centerh_of pc);
        layout!(context, g_c_pt, bottom_to_top_of pc);
        layout!(context, grow g_c_pt, align_top);

        layout!(context, g_c_pb, centerh_to_centerh_of pc);
        layout!(context, g_c_pb, top_to_bottom_of  pc);
        layout!(context, grow g_c_pb, align_bottom);

        layout!(context, g_c_pl, centerv_to_centerv_of  pc);
        layout!(context, g_c_pl, right_to_left_of   pc);
        layout!(context, grow g_c_pl, align_left);

        layout!(context, g_c_pr, centerv_to_centerv_of  pc);
        layout!(context, g_c_pr, left_to_right_of    pc);
        layout!(context, grow g_c_pr, align_right );

        layout!(context, g_c_v_tt, centerh_to_centerh_of vc);
        layout!(context, g_c_v_tt, bottom_to_top_of vc);
        layout!(context, grow g_c_v_tt, top_to_centerv_of bounds_tl);

        layout!(context, g_c_v_bb, centerh_to_centerh_of vc);
        layout!(context, g_c_v_bb, top_to_bottom_of vc);
        layout!(context, grow g_c_v_bb, bottom_to_centerv_of bounds_br);

        layout!(context, g_c_v_ll, centerv_to_centerv_of  vc);
        layout!(context, g_c_v_ll, right_to_left_of  vc);
        layout!(context, grow g_c_v_ll, left_to_centerh_of  bounds_tl);

        layout!(context, g_c_v_rr, centerv_to_centerv_of  vc);
        layout!(context, g_c_v_rr, left_to_right_of   vc);
        layout!(context, grow g_c_v_rr, right_to_centerh_of   bounds_br);

        Box::new(LayoutTest {
            pc,
            vc,
            bounds_tl,
            bounds_br,
            g_c_pt,
            g_c_pb,
            g_c_pl,
            g_c_pr,
            g_c_v_tt,
            g_c_v_bb,
            g_c_v_ll,
            g_c_v_rr,
        })
    }
}

impl Scene<SceneResult, SceneName> for LayoutTest {
    fn render(&self, graphics: &mut Graphics, mouse: &MouseData, _: &FxHashSet<KeyCode>) {
        graphics.clear(BLUE);
        render!(
            graphics,
            mouse,
            self.pc,
            self.bounds_tl,
            self.bounds_br,
            self.g_c_pt,
            self.g_c_pb,
            self.g_c_pl,
            self.g_c_pr,
            self.vc,
            self.g_c_v_tt,
            self.g_c_v_bb,
            self.g_c_v_ll,
            self.g_c_v_rr,
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
