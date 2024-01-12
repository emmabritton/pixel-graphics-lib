use buffer_graphics_lib::text::TextSize::Normal;
use pixels_graphics_lib::prelude::*;
use pixels_graphics_lib::scenes::SceneUpdateResult::Nothing;
use pixels_graphics_lib::ui::prelude::*;

#[derive(Debug, Clone, PartialEq)]
enum SR {}

#[derive(Debug, Clone, PartialEq)]
enum SN {}

struct WhiteTextScene {
    text: Text,
}

impl WhiteTextScene {
    pub fn new() -> Box<Self> {
        Box::new(WhiteTextScene {
            text: Text::new(
                "Test string",
                TextPos::Px(50, 50),
                (GB_3, Normal, Positioning::Center),
            ),
        })
    }
}

impl Scene<SR, SN> for WhiteTextScene {
    fn render(&self, graphics: &mut Graphics, _: &MouseData, _: &[KeyCode], _: &GameController) {
        self.text.render(graphics);
    }

    fn update(
        &mut self,
        _: &Timing,
        _: &MouseData,
        _: &[KeyCode],
        _: &GameController,
    ) -> SceneUpdateResult<SR, SN> {
        Nothing
    }
}

#[derive(Debug)]
struct ExtrasImpl {
    pixel: Coord,
    timer: Timer,
}

impl ExtrasImpl {
    pub fn new() -> Box<Self> {
        Box::new(ExtrasImpl {
            pixel: coord!(0, 91),
            timer: Timer::new(0.2),
        })
    }
}

impl PrePost<SR, SN> for ExtrasImpl {
    fn pre_render(
        &mut self,
        graphics: &mut Graphics,
        _: &MouseData,
        _: &[KeyCode],
        _: &mut [Box<dyn Scene<SR, SN>>],
        _: &GameController,
    ) {
        graphics.clear(GB_0);
        graphics.draw_line((0, 48), (100, 48), WHITE);
    }

    fn post_render(
        &mut self,
        graphics: &mut Graphics,
        _: &MouseData,
        _: &[KeyCode],
        _: &mut [Box<dyn Scene<SR, SN>>],
        _: &GameController,
    ) {
        graphics.draw_rect(Rect::new((10, 10), (90, 90)), stroke(GB_2));
        graphics.draw_line((0, 51), (100, 51), DARK_GRAY);
        graphics.set_pixel(self.pixel.x, self.pixel.y, RED);
    }

    fn pre_update(
        &mut self,
        _: &Timing,
        _: &MouseData,
        _: &[KeyCode],
        _: &mut [Box<dyn Scene<SR, SN>>],
        _: &GameController,
    ) {
    }

    fn post_update(
        &mut self,
        timing: &Timing,
        _: &MouseData,
        _: &[KeyCode],
        _: &mut [Box<dyn Scene<SR, SN>>],
        _: &GameController,
    ) {
        if self.timer.update(timing) {
            self.pixel.x += 1;
            if self.pixel.x > 99 {
                self.pixel.x = 0
            }
        }
    }
}

fn main() {
    let switcher = |style: &UiStyle, scenes: &mut Vec<Box<dyn Scene<SR, SN>>>, new_scene: SN| {};
    run_scenes(
        100,
        100,
        "Pre/Post test",
        None,
        switcher,
        WhiteTextScene::new(),
        Options::default(),
        ExtrasImpl::new(),
    )
    .unwrap()
}
