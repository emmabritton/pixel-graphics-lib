use anyhow::Result;
use buffer_graphics_lib::prelude::DrawType::Stroke;
use buffer_graphics_lib::prelude::Positioning::*;
use buffer_graphics_lib::prelude::*;
use pixels_graphics_lib::prefs::WindowPreferences;
use pixels_graphics_lib::prelude::SceneUpdateResult::*;
use pixels_graphics_lib::prelude::*;
use pixels_graphics_lib::ui::prelude::*;
use pixels_graphics_lib::ui::text_field::TextFilter::All;

#[allow(clippy::upper_case_acronyms)]
type SUR = SceneUpdateResult<SceneResult, SceneName>;

const WIDTH: usize = 280;
const HEIGHT: usize = 240;

fn main() -> Result<()> {
    let switcher: SceneSwitcher<SceneResult, SceneName> = |style, list, name| {};
    let options = Options::default();
    let menu = Menu::new(&options.style);
    run_scenes(
        WIDTH,
        HEIGHT,
        "UI Tester",
        Some(WindowPreferences::new("app", "emmabritton", "pixels_ui_tester").unwrap()),
        switcher,
        menu,
        options,
    )?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
enum SceneName {}

#[derive(Debug, Clone, PartialEq)]
enum SceneResult {}

pub struct Menu {
    result: SUR,
    background: Color,
    title: Text,
    tooltip_rect: Drawable<Rect>,
    tooltip: Tooltip,
    button1: Button,
    button2: Button,
    toggle_buttons: ToggleButtonGroup<usize>,
    icon_button1: IconButton,
    icon_button2: IconButton,
    field1: TextField,
    field2: TextField,
    field3: TextField,
    icon_group: ToggleIconButtonGroup<usize>,
}

impl Menu {
    pub fn new(style: &UiStyle) -> Box<Self> {
        let (icon, _) =
            IndexedImage::from_file_contents(include_bytes!("resources/icon.ici")).unwrap();
        let (large_icon, _) =
            IndexedImage::from_file_contents(include_bytes!("resources/large_icon.ici")).unwrap();
        let title = Text::new("UI Tester", Px(8, 8), style.title_text.clone());
        let tooltip_rect = Drawable::from_obj(Rect::new_with_size((8, 40), 10, 10), Stroke(WHITE));
        let tooltip = Tooltip::new((8, 40), "This is a test tooltip", LeftTop, &style.tooltip);
        let button1 = Button::new((8, 60), "Test Button", None, &style.button);
        let button2 = Button::new((8, 80), "Test Button", Some(100), &style.button);
        let toggle_button1 = ToggleButton::new((160, 40), "TB 1", None, &style.toggle_button);
        let toggle_button2 = ToggleButton::new((160, 60), "TB 2", Some(60), &style.toggle_button);
        let icon_button1 = IconButton::new(
            (160, 8),
            "Test",
            CenterTop,
            icon.clone(),
            &style.icon_button,
        );
        let icon_button2 = IconButton::new((180, 8), "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.", CenterTop, large_icon, &style.icon_button);
        let field1 = TextField::new(
            (8, 100),
            6,
            Normal,
            (None, Some(30)),
            "",
            &[All],
            &style.text_field,
        );
        let field2 = TextField::new(
            (8, 120),
            6,
            Normal,
            (None, None),
            "",
            &[All],
            &style.text_field,
        );
        let field3 = TextField::new(
            (8, 140),
            6,
            Normal,
            (Some(100), None),
            "",
            &[All],
            &style.text_field,
        );
        let toggle_buttons = ToggleButtonGroup::new(vec![(0, toggle_button1), (1, toggle_button2)]);
        let toggle_icon1 = ToggleIconButton::new(
            (160, 100),
            "1",
            LeftTop,
            icon.clone(),
            &style.toggle_icon_button,
        );
        let toggle_icon2 = ToggleIconButton::new(
            (180, 100),
            "2",
            LeftTop,
            icon.clone(),
            &style.toggle_icon_button,
        );
        let toggle_icon3 = ToggleIconButton::new(
            (200, 100),
            "3",
            LeftTop,
            icon.clone(),
            &style.toggle_icon_button,
        );
        let icon_group = ToggleIconButtonGroup::new(vec![
            (0, toggle_icon1),
            (1, toggle_icon2),
            (2, toggle_icon3),
        ]);
        Box::new(Self {
            icon_group,
            result: Nothing,
            background: style.background,
            title,
            tooltip_rect,
            tooltip,
            button1,
            button2,
            toggle_buttons,
            icon_button1,
            icon_button2,
            field1,
            field2,
            field3,
        })
    }
}

impl Scene<SceneResult, SceneName> for Menu {
    fn render(&self, graphics: &mut Graphics, mouse_xy: Coord) {
        graphics.clear(self.background);
        self.title.render(graphics);
        self.tooltip_rect.render(graphics);
        if self.tooltip_rect.obj().contains(mouse_xy) {
            self.tooltip.render(graphics, mouse_xy);
        }
        self.button1.render(graphics, mouse_xy);
        self.button2.render(graphics, mouse_xy);
        self.toggle_buttons.render(graphics, mouse_xy);
        self.icon_group.render(graphics, mouse_xy);
        self.icon_button1.render(graphics, mouse_xy);
        self.icon_button2.render(graphics, mouse_xy);
        self.field1.render(graphics, mouse_xy);
        self.field2.render(graphics, mouse_xy);
        self.field3.render(graphics, mouse_xy);
    }

    fn on_key_press(&mut self, key: VirtualKeyCode, held_keys: &Vec<&VirtualKeyCode>) {
        self.field1.on_key_press(key);
        self.field2.on_key_press(key);
        self.field3.on_key_press(key);
    }

    fn on_mouse_click(&mut self, xy: Coord, held_keys: &Vec<&VirtualKeyCode>) {
        self.field1.on_mouse_click(xy);
        self.field2.on_mouse_click(xy);
        self.field3.on_mouse_click(xy);
        self.toggle_buttons.on_mouse_click(xy);
        self.icon_group.on_mouse_click(xy);
    }

    fn update(
        &mut self,
        timing: &Timing,
        mouse_xy: Coord,
        held_keys: &Vec<&VirtualKeyCode>,
    ) -> SceneUpdateResult<SceneResult, SceneName> {
        self.field1.update(timing);
        self.field2.update(timing);
        self.field3.update(timing);
        self.result.clone()
    }

    fn resuming(&mut self, _: Option<SceneResult>) {
        self.result = Nothing;
    }
}
