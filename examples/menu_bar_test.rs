use anyhow::Result;
use buffer_graphics_lib::prelude::*;
use buffer_graphics_lib::text::PixelFont::Standard6x7;
use winit::keyboard::KeyCode;

use pixels_graphics_lib::prelude::SceneUpdateResult::*;
use pixels_graphics_lib::prelude::*;
use pixels_graphics_lib::render;
use pixels_graphics_lib::ui::prelude::*;

const WIDTH: usize = 280;
const HEIGHT: usize = 240;

fn main() -> Result<()> {
    let switcher: SceneSwitcher<SceneResult, SceneName> = |_, _, _| {};
    let options = Options::default();
    let test = MenuTest::new(&options.style);
    run_scenes(
        WIDTH,
        HEIGHT,
        "Menu Tester",
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

pub struct MenuTest {
    menubar: MenuBar<usize>,
}

impl MenuTest {
    pub fn new(style: &UiStyle) -> Box<Self> {
        let mut menubar = MenuBar::new(
            &style.menu,
            Coord::default(),
            (WIDTH, HEIGHT),
            true,
            &[
                MenuBarItem::new(
                    0,
                    "File",
                    vec![
                        MenuBarItem::new_button(1, "New"),
                        MenuBarItem::new_button(2, "Open"),
                        MenuBarItem::new_button(3, "Save"),
                        MenuBarItem::new_button(4, "Save As"),
                        MenuBarItem::new_button(5, "Quit"),
                    ],
                ),
                MenuBarItem::new(
                    6,
                    "Edit",
                    vec![
                        MenuBarItem::new_button(7, "Flip vert"),
                        MenuBarItem::new_button(8, "Flip Horz"),
                        MenuBarItem::new_menu(9, "Rotate", &[(10, "90"), (11, "180"), (12, "270")]),
                        MenuBarItem::new_checkable(13, "Show guidelines", true),
                    ],
                ),
                MenuBarItem::new(
                    14,
                    "Image",
                    vec![MenuBarItem::new_options(
                        15,
                        "Palette",
                        &[(16, "Nothing"), (17, "File"), (18, "ID")],
                        0,
                    )],
                ),
            ],
        );

        menubar.set_state(8, ViewState::Disabled);

        Box::new(MenuTest { menubar })
    }
}

impl Scene<SceneResult, SceneName> for MenuTest {
    fn render(&self, graphics: &mut Graphics, mouse: &MouseData, _: &[KeyCode]) {
        graphics.clear(BLUE);
        render!(graphics, mouse, self.menubar);

        graphics.draw_text(
            &format!("{:?}", mouse.xy),
            TextPos::px(coord!(4, 80)),
            (
                WHITE,
                Standard6x7,
                WrappingStrategy::AtCol(Standard6x7.px_to_cols(WIDTH - 8)),
            ),
        );
    }

    fn on_key_up(&mut self, _: KeyCode, _: &MouseData, _: &[KeyCode]) {}

    fn on_mouse_click(&mut self, xy: Coord, mouse: &MouseData, button: MouseButton, _: &[KeyCode]) {
        if button == MouseButton::Left {
            if let Some(path) = self.menubar.on_mouse_click(xy, mouse.xy) {
                println!("Clicked on {path:?} {:?}", self.menubar.label_for(path));
            }
        }
    }

    fn update(
        &mut self,
        _: &Timing,
        mouse: &MouseData,
        _: &[KeyCode],
    ) -> SceneUpdateResult<SceneResult, SceneName> {
        self.menubar.on_mouse_move(mouse.xy);
        Nothing
    }

    fn resuming(&mut self, _: Option<SceneResult>) {}
}
