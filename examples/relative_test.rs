use anyhow::Result;
use buffer_graphics_lib::prelude::*;
use winit::keyboard::KeyCode;

use pixels_graphics_lib::prelude::SceneUpdateResult::*;
use pixels_graphics_lib::prelude::*;
use pixels_graphics_lib::ui::prelude::relative::*;
use pixels_graphics_lib::ui::prelude::*;
use pixels_graphics_lib::{layout, parent, px, render};

const WIDTH: usize = 280;
const HEIGHT: usize = 240;

fn main() -> Result<()> {
    let switcher: SceneSwitcher<SceneResult, SceneName> = |_, _, _| {};
    let options = Options::default();
    let test = LayoutTest::new(&options.style);
    run_scenes(
        WIDTH,
        HEIGHT,
        "Relative Tester",
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
    mid: Button,
    above: Button,
    below: Button,
    left: Button,
    right: Button,
    bl: Button,
    br: Button,
    tr: Button,
    tl: Button,
    fw: TextField,
    fh: Button,
    sv: Button,
    sh: Button,
    pl: Button,
    pt: Button,
    pr: ToggleButton,
    pb: Button,
    gl: Button,
    gr: Button,
    gt: Button,
    gb: Button,
}

impl LayoutTest {
    pub fn new(style: &UiStyle) -> Box<Self> {
        let mut mid = Button::new((0, 0), "MID", Some(60), &style.button);
        let mut above = Button::new((0, 0), "T", Some(10), &style.button);
        let mut below = Button::new((0, 0), "B", Some(10), &style.button);
        let mut left = Button::new((0, 0), "L", Some(10), &style.button);
        let mut right = Button::new((0, 0), "R", Some(10), &style.button);
        let mut bl = Button::new((0, 0), "BL", Some(10), &style.button);
        let mut br = Button::new((0, 0), "BR", Some(10), &style.button);
        let mut tr = Button::new((0, 0), "TR", Some(10), &style.button);
        let mut tl = Button::new((0, 0), "TL", Some(10), &style.button);
        let mut fw = TextField::new(
            (0, 0),
            10,
            PixelFont::Standard4x5,
            (None, None),
            "FW",
            &[],
            &style.text_field,
        );
        let mut fh = Button::new((0, 0), "FH", Some(10), &style.button);
        let mut sv = Button::new((0, 0), "SV", Some(10), &style.button);
        let mut sh = Button::new((0, 0), "SH", Some(10), &style.button);
        let mut pl = Button::new((0, 0), "P.L", Some(10), &style.button);
        let mut pr = ToggleButton::new((0, 0), "P.R", Some(10), &style.toggle_button);
        let mut pt = Button::new((0, 0), "P.T", Some(10), &style.button);
        let mut pb = Button::new((0, 0), "P.B", Some(10), &style.button);
        let mut gl = Button::new((0, 0), "G.L", Some(10), &style.button);
        let mut gr = Button::new((0, 0), "G.R", Some(10), &style.button);
        let mut gt = Button::new((0, 0), "G.T", Some(10), &style.button);
        let mut gb = Button::new((0, 0), "G.B", Some(10), &style.button);

        let context = LayoutContext::new(Rect::new_with_size((0, 0), WIDTH, HEIGHT));

        layout!(context, mid, align_left, parent!(0.4));
        layout!(context, mid, align_top, parent!(0.4));

        layout!(context, above, left_to_left_of mid);
        layout!(context, above, bottom_to_top_of mid, px!(40));

        layout!(context, below, left_to_left_of mid);
        layout!(context, below, top_to_bottom_of mid, px!(40));

        layout!(context, left, right_to_left_of  mid, px!(40));
        layout!(context, left, top_to_top_of  mid);

        layout!(context, right, left_to_right_of   mid, px!(40));
        layout!(context, right, top_to_top_of  mid);

        layout!(context, tl, left_to_left_of mid);
        layout!(context, tl, bottom_to_top_of mid);

        layout!(context, bl, left_to_left_of mid);
        layout!(context, bl, top_to_bottom_of  mid);

        layout!(context, br, right_to_right_of  mid);
        layout!(context, br, top_to_bottom_of  mid);

        layout!(context, tr, right_to_right_of  mid);
        layout!(context, tr, bottom_to_top_of mid);

        layout!(context, grow fw, fill_width);

        layout!(context, grow fh, fill_height);

        layout!(context, sh, left_to_left_of left);
        layout!(context, grow sh, right_to_right_of  right);
        layout!(context, sh, top_to_bottom_of bl);

        layout!(context, sv, top_to_top_of  above);
        layout!(context, grow sv, bottom_to_bottom_of below);
        layout!(context, sv, right_to_left_of  bl);

        layout!(context, pt, align_top);
        layout!(context, pt, align_left, parent!(0.3));

        layout!(context, pb, align_bottom);
        layout!(context, pb, align_left, parent!(0.3));

        layout!(context, pl, align_left);
        layout!(context, pl, align_top, parent!(0.3));

        layout!(context, pr, align_right);
        layout!(context, pr, align_top, parent!(0.3));

        layout!(context, gl, right_to_left_of right);
        layout!(context, gl, top_to_bottom_of below);
        layout!(context, grow gl, align_left);

        layout!(context, gr, left_to_right_of  right);
        layout!(context, gr, top_to_bottom_of below);
        layout!(context, grow gr, align_right );

        layout!(context, gb, left_to_left_of  right);
        layout!(context, gb, top_to_bottom_of gl);
        layout!(context, grow gb, align_bottom  );

        layout!(context, gt, left_to_left_of  right);
        layout!(context, grow gt, right_to_right_of   right);
        layout!(context, gt, bottom_to_top_of  right, px!(8));
        layout!(context, grow gt, align_top   );

        Box::new(LayoutTest {
            mid,
            above,
            below,
            left,
            right,
            bl,
            br,
            tr,
            tl,
            fw,
            fh,
            sv,
            sh,
            pl,
            pt,
            pr,
            pb,
            gl,
            gr,
            gt,
            gb,
        })
    }
}

impl Scene<SceneResult, SceneName> for LayoutTest {
    fn render(&self, graphics: &mut Graphics, mouse: &MouseData, _: &FxHashSet<KeyCode>) {
        graphics.clear(BLUE);
        render!(
            graphics, mouse, self.mid, self.above, self.below, self.left, self.right, self.br,
            self.bl, self.tl, self.tr, self.fw, self.fh, self.sv, self.sh, self.pt, self.pb,
            self.pl, self.pr, self.gl, self.gr, self.gb, self.gt
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
